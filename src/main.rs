pub mod app_handler;
pub mod js;
pub mod window;

use app_handler::AppEvent;
use app_handler::AppHandler;

use std::sync::mpsc::channel;
use std::thread;

use glutin::event::Event;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::<AppEvent>::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    let (send, recv) = channel();
    let app_handler = AppHandler::new(event_loop_proxy, recv);

    thread::spawn(|| {
        let mut js_runtime = js::Runtime::new();
        js_runtime.run("app/index.js", app_handler);
    });

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(AppEvent::CreateWindow) => {
                let window = WindowBuilder::new().build(&event_loop).unwrap();
                send.send(window).unwrap();
            }
            _ => (),
        }
    });
}
