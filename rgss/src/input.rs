use crate::event_loop::{Event, Events, UserEvent};
use winit::event::WindowEvent;

mod keycode;
pub use keycode::KeyCode;

pub struct Input {
    events: Events,
    exited: bool,
}

// TODO add an optional pump_events feature that uses winit::EventLoopExtPumpEvents that allows running bindings on the main thread
impl Input {
    pub fn new(events: Events) -> Self {
        Self {
            events,
            exited: false,
        }
    }

    /// Process all incoming events from the event loop, updating all input state.
    pub fn update(&mut self) {
        let mut needs_exit = false;
        for event in self.events.iter() {
            match event {
                // TODO handle window events
                Event::WindowEvent(window_event) => {
                    //
                    match window_event {
                        WindowEvent::CloseRequested | WindowEvent::Destroyed => needs_exit = true,
                        _ => {}
                    }
                }
                Event::Exiting => self.exited = true,
            }
        }
        if needs_exit {
            self.exit();
        }
    }

    /// Notifies the event loop that we'd like to exit.
    pub fn exit(&mut self) {
        // if sending an event returns Err() it means the event loop is closed and we should exit.
        self.exited |= self.events.send(UserEvent::ExitEventLoop).is_err();
    }

    pub fn exited(&self) -> bool {
        self.exited
    }
}
