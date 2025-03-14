use crossbeam::channel::{Receiver, Sender};
use winit::{
    error::EventLoopError,
    event_loop::{ActiveEventLoop, EventLoopProxy},
};

pub struct EventLoop {
    event_loop: winit::event_loop::EventLoop<UserEvent>,
    event_send: Sender<Event>,
}

pub struct Events {
    event_recv: Receiver<Event>,
    event_proxy: EventLoopProxy<UserEvent>,
}

pub(crate) enum UserEvent {
    ExitEventLoop,
}

pub(crate) enum Event {
    WindowEvent(winit::event::WindowEvent),
    Exiting,
}

impl EventLoop {
    /// Create a new event loop for processing events.
    pub fn new() -> Result<(Self, Events), EventLoopError> {
        let event_loop = winit::event_loop::EventLoop::with_user_event().build()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let event_proxy = event_loop.create_proxy();

        let (event_send, event_recv) = crossbeam::channel::unbounded();

        let event_loop = Self {
            event_loop,
            event_send,
        };

        let events = Events {
            event_recv,
            event_proxy,
        };

        Ok((event_loop, events))
    }

    /// Run the event loop.
    ///
    /// This will take control of the main thread until the application exits.
    /// Due to some... Pretty wild platform specific details, you'll need to pass a closure to this function in order to create a window.
    pub fn run<F>(self, on_first_resume: F) -> Result<(), EventLoopError>
    where
        F: FnOnce(&ActiveEventLoop),
    {
        struct App<F> {
            event_send: Sender<Event>,
            on_first_resume: Option<F>,
        }

        impl<F> winit::application::ApplicationHandler<UserEvent> for App<F>
        where
            F: FnOnce(&ActiveEventLoop),
        {
            fn resumed(&mut self, event_loop: &ActiveEventLoop) {
                if let Some(on_first_resume) = self.on_first_resume.take() {
                    on_first_resume(event_loop)
                }
            }

            fn window_event(
                &mut self,
                event_loop: &ActiveEventLoop,
                _: winit::window::WindowId,
                event: winit::event::WindowEvent,
            ) {
                if self.event_send.send(Event::WindowEvent(event)).is_err() && !event_loop.exiting()
                {
                    event_loop.exit();
                }
            }

            fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
                match event {
                    UserEvent::ExitEventLoop => event_loop.exit(),
                }
            }

            fn exiting(&mut self, _: &ActiveEventLoop) {
                let _ = self.event_send.send(Event::Exiting);
            }
        }

        let mut app = App {
            event_send: self.event_send,
            on_first_resume: Some(on_first_resume),
        };
        self.event_loop.run_app(&mut app)
    }
}

impl Events {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Event> {
        self.event_recv.try_iter()
    }

    pub(crate) fn send(
        &self,
        event: UserEvent,
    ) -> Result<(), winit::event_loop::EventLoopClosed<UserEvent>> {
        self.event_proxy.send_event(event)
    }
}
