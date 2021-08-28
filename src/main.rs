use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowBuilder;
use winit::window::WindowId;

mod js_runtime;

enum CustomEvent {
    CreateWindow,
}

struct AppHandler {
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    windows: HashMap<WindowId, Window>,
    recv: Receiver<Window>,
}

impl AppHandler {
    fn new(event_loop_proxy: EventLoopProxy<CustomEvent>, recv: Receiver<Window>) -> Self {
        let windows = HashMap::new();
        AppHandler {
            event_loop_proxy,
            windows,
            recv,
        }
    }

    fn create_window(&mut self) {
        self.event_loop_proxy
            .send_event(CustomEvent::CreateWindow)
            .ok();
        let window = self.recv.recv().unwrap();
        self.windows.insert(window.id(), window);
    }
}

fn main() {
    let event_loop = EventLoop::<CustomEvent>::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    let (send, recv): (Sender<Window>, Receiver<Window>) = channel();

    thread::spawn(move || {
        let mut app_handler = AppHandler::new(event_loop_proxy, recv);
        app_handler.create_window();

        js_runtime::run("src/main.js");
    });

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(CustomEvent::CreateWindow) => {
                let window = WindowBuilder::new().build(&event_loop).unwrap();
                send.send(window).unwrap();
            }
            _ => (),
        }
    });
}
