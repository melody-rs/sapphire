use magnus::{Module, Object, function, method};

mod journal;
mod steam;
mod wallpaper;

fn set_yes_no(module: magnus::RModule, yes: String, no: String) -> magnus::error::Result<()> {
    module.ivar_set("yes", yes)?;
    module.ivar_set("no", no)
}

fn msgbox(module: magnus::RModule, kind: u8, text: String) -> magnus::error::Result<bool> {
    let yes: String = module.ivar_get("yes")?;
    let no = module.ivar_get("no")?;

    let result = match kind {
        1 => rfd::MessageDialog::new()
            .set_description(text)
            .set_title("Sapphire")
            .set_level(rfd::MessageLevel::Info)
            .show(),
        3 => rfd::MessageDialog::new()
            .set_description(text)
            .set_title("Sapphire")
            .set_buttons(rfd::MessageButtons::OkCancelCustom(yes.clone(), no))
            .show(),
        _ => todo!(),
    };

    Ok(matches!(result, rfd::MessageDialogResult::Custom(res) if res == yes))
}

fn exiting(value: bool) {}

fn allow_exit(value: bool) {}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let module = ruby.define_module("Oneshot")?;

    let save_path = dirs::data_local_dir()
        .expect("no data dir found")
        .join("OneShot");
    if !save_path.exists() {
        std::fs::create_dir(&save_path).expect("failed to create save dir");
    }

    let docs_dir = dirs::document_dir().expect("no document dir found");
    let game_dir = docs_dir.join("MyGames");

    let username = whoami::username();

    module.const_set("SAVE_PATH", save_path)?;
    module.const_set("DOCS_PATH", docs_dir)?;
    module.const_set("GAME_PATH", game_dir)?;
    module.const_set("USER_NAME", username)?;

    module.define_module_function("exiting", function!(exiting, 1))?;
    module.define_module_function("allow_exit", function!(allow_exit, 1))?;

    #[cfg(target_os = "linux")]
    module.const_set("OS", "linux")?;
    #[cfg(target_os = "windows")]
    module.const_set("OS", "windows")?;

    let msg = module.define_module("Msg")?;
    msg.const_set("INFO", 1)?;
    msg.const_set("YESNO", 3)?;

    module.define_module_function("set_yes_no", method!(set_yes_no, 2))?;
    module.define_module_function("msgbox", method!(msgbox, 2))?;

    steam::bind(ruby)?;
    wallpaper::bind(ruby)?;
    journal::bind(ruby)?;

    Ok(())
}
