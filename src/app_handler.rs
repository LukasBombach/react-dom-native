use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowId;

use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub enum AppEvent {
  CreateWindow,
}

pub struct AppHandler {
  event_loop_proxy: EventLoopProxy<AppEvent>,
  windows: HashMap<WindowId, Window>,
  recv: Receiver<Window>,
}

impl AppHandler {
  pub fn new(event_loop_proxy: EventLoopProxy<AppEvent>, recv: Receiver<Window>) -> Self {
    let windows = HashMap::new();
    AppHandler {
      event_loop_proxy,
      windows,
      recv,
    }
  }

  pub fn create_window(&mut self) {
    self
      .event_loop_proxy
      .send_event(AppEvent::CreateWindow)
      .ok();
    let window = self.recv.recv().unwrap();
    self.windows.insert(window.id(), window);
  }
}
