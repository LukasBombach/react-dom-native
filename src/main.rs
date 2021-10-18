pub mod app;
pub mod deno;
pub mod renderer;
pub mod support;

use crate::app::App;
use crate::app::AppEvent;
use glutin::event::Event;
use glutin::event::StartCause;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;

fn main() {
    let el = EventLoop::<AppEvent>::with_user_event();
    let el_proxy = el.create_proxy();
    let mut app = App::new(el_proxy);

    el.run(move |event, el, control_flow| match event {
        Event::NewEvents(StartCause::Init) => {
            app.init();
        }
        Event::WindowEvent { event, window_id } => match event {
            WindowEvent::Resized(physical_size) => {
                app.resize_window(window_id, physical_size);
            }
            WindowEvent::CloseRequested => {
                app.remove_window(window_id);
            }
            _ => (),
        },
        Event::RedrawRequested(window_id) => {
            app.render(window_id);
        }
        Event::UserEvent(AppEvent::NewWindowRequested) => app.create_window(el),
        Event::UserEvent(AppEvent::QuitAppRequested) => *control_flow = ControlFlow::Exit,
        _ => (),
    });
}
