use glutin::event_loop::EventLoopProxy;
use glutin::window::Window;

use std::sync::mpsc::Receiver;

pub enum AppEvent {
  CreateWindow,
}

pub struct AppHandler {
  event_loop_proxy: EventLoopProxy<AppEvent>,
  recv: Receiver<Window>,
}

impl AppHandler {
  pub fn new(event_loop_proxy: EventLoopProxy<AppEvent>, recv: Receiver<Window>) -> Self {
    AppHandler {
      event_loop_proxy,
      recv,
    }
  }

  pub fn create_window(&mut self) -> Window {
    self
      .event_loop_proxy
      .send_event(AppEvent::CreateWindow)
      .ok();
    let window = self.recv.recv().unwrap();
    window
  }
}
