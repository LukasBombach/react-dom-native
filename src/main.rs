mod app_handler;
mod js;

use app_handler::AppEvent;
use app_handler::AppHandler;

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::<AppEvent>::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    let (send, recv): (Sender<Window>, Receiver<Window>) = channel();

    thread::spawn(move || {
        let mut app_handler = AppHandler::new(event_loop_proxy, recv);
        app_handler.create_window();

        let mut js_runtime = js::Runtime::new();
        js_runtime.run("app/index.js");
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
