use pollster::FutureExt;

fn main() {
    env_logger::init();

    let config: rgss::Config = std::fs::read_to_string("sapphire_config.toml")
        .ok()
        // we use expect() so we exit if the config file was invalid
        .map(|config_text| toml::from_str(&config_text).expect("failed to read config file"))
        .unwrap_or_default();

    let printer = color_backtrace::BacktracePrinter::new()
        .lib_verbosity(color_backtrace::Verbosity::Full) // because we have a custom panic handler we use lib_verbosity instead (is weird)
        .message("Sapphire has encountered a fatal error!");
    std::panic::set_hook(Box::new(move |info| {
        let _ = printer.print_panic_info(info, &mut color_backtrace::default_output_stream());

        let mut buf = vec![];
        let mut writer = color_backtrace::termcolor::NoColor::new(&mut buf);
        let _ = printer.print_panic_info(info, &mut writer);

        let backtrace_text = String::from_utf8_lossy(&buf);
        let last_line_index = backtrace_text
            .split_inclusive('\n')
            .map(str::len)
            .take(35)
            .sum();
        let truncated = &backtrace_text[..last_line_index];
        rfd::MessageDialog::new()
            .set_title("Fatal Error!")
            .set_level(rfd::MessageLevel::Error)
            .set_description(truncated)
            .show();
        if config.behaviour.abort_on_panic {
            std::process::abort();
        }
    }));

    #[cfg(feature = "deadlock_detection")]
    std::thread::Builder::new()
        .name("sapphire deadlock detection".to_string())
        .spawn(|| {
            //
            let mut deadlocks = parking_lot::deadlock::check_deadlock();
            while deadlocks.is_empty() {
                std::thread::sleep(std::time::Duration::from_secs(10));
                deadlocks = parking_lot::deadlock::check_deadlock();
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }

            std::process::abort();
        })
        .expect("failed to spawn deadlock thread");

    std::env::set_current_dir(&config.fs.game_dir).unwrap();

    let (event_loop, events) = rgss::EventLoop::new().unwrap();

    let audio = rgss::Audio::new().unwrap();
    let input = rgss::Input::new(events);
    let filesystem = rgss::FileSystem::new(".", None).unwrap();

    let mut binding_thread = None;
    let result = event_loop.run(|active_event_loop| {
        let window_attrs = winit::window::Window::default_attributes()
            .with_inner_size(winit::dpi::PhysicalSize::new(640, 480))
            .with_resizable(false)
            .with_title("Sapphire");
        let window = active_event_loop
            .create_window(window_attrs)
            .map(std::sync::Arc::new)
            .unwrap();

        let mut arenas = rgss::Arenas::default();

        let graphics = rgss::Graphics::new(window, &config, &mut arenas)
            .block_on()
            .unwrap();
        let fonts = rgss::Fonts::new(&graphics, &mut arenas);

        let ctx = rgss::Ctx {
            arenas,

            audio,
            graphics,
            input,
            filesystem,
            fonts,
        };
        binding_thread = Some(binding_magnus::start(ctx));
    });

    if let Err(e) = result {
        eprintln!("An error occured while running the event loop: {e}");
    }

    // wait for binding thread to finish
    let Some(join_handle) = binding_thread else {
        eprintln!("Execution ended before binding started!");
        return;
    };

    let start = std::time::Instant::now();
    let mut wait_time = std::time::Duration::from_micros(250);
    while std::time::Instant::now().duration_since(start) < std::time::Duration::from_secs(8) {
        if join_handle.is_finished() {
            break;
        }
        // wait expontentially longer until either the join handle finishes or 8 seconds elapsed
        std::thread::sleep(wait_time);
        wait_time *= 2;
    }

    // the binding thread didn't finish within a reasonable time, so we exit
    if !join_handle.is_finished() {
        eprintln!("The RGSS script seems to be stuck. Sapphire will now force quit");
        std::process::abort();
    }
    // ok, binding thread finished, so join it.
    let _ = join_handle.join();
}
