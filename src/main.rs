mod renderer;
mod support;

use gl::types::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::{BackendRenderTarget, SurfaceOrigin};
use skia_safe::{Color, ColorType, Surface};
use std::convert::TryInto;
use support::{ContextCurrentWrapper, ContextTracker, ContextWrapper};

type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

fn create_surface(
    windowed_context: &WindowedContext,
    fb_info: &FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
) -> skia_safe::Surface {
    let pixel_format = windowed_context.get_pixel_format();
    let size = windowed_context.window().inner_size();
    let backend_render_target = BackendRenderTarget::new_gl(
        (
            size.width.try_into().unwrap(),
            size.height.try_into().unwrap(),
        ),
        pixel_format.multisampling.map(|s| s.try_into().unwrap()),
        pixel_format.stencil_bits.try_into().unwrap(),
        *fb_info,
    );
    Surface::from_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .unwrap()
}

struct Win {
    context_id: usize,
    surface: Surface,
    gr_context: skia_safe::gpu::DirectContext,
    fb_info: FramebufferInfo,
}

fn main() {
    let el = EventLoop::new();
    let mut ct = ContextTracker::default();

    let mut windows = std::collections::HashMap::new();

    for index in 0..3 {
        let title = format!("Charming Window #{}", index + 1);
        let wb = WindowBuilder::new().with_title(title);
        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();
        gl::load_with(|s| windowed_context.get_proc_address(s));
        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };
            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };
        let surface = create_surface(&windowed_context, &fb_info, &mut gr_context);

        let window_id = windowed_context.window().id();
        let context_id = ct.insert(ContextCurrentWrapper::PossiblyCurrent(
            ContextWrapper::Windowed(windowed_context),
        ));

        let win = Win {
            context_id,
            surface,
            gr_context,
            fb_info,
        };

        windows.insert(window_id, win);
    }

    el.run(move |event, _, control_flow| {
        #[allow(deprecated)]
        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::Resized(physical_size) => {
                    if let Some(win) = windows.get_mut(&window_id) {
                        let windowed_context = ct.get_current(win.context_id).unwrap();

                        win.surface = create_surface(
                            windowed_context.windowed(),
                            &win.fb_info,
                            &mut win.gr_context,
                        );
                        windowed_context.windowed().resize(physical_size);
                    }
                }
                WindowEvent::CloseRequested => {
                    if let Some(win) = windows.remove(&window_id) {
                        ct.remove(win.context_id);
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(window_id) => {
                if let Some(win) = windows.get_mut(&window_id) {
                    let windowed_context = ct.get_current(win.context_id).unwrap();

                    let canvas = win.surface.canvas();
                    canvas.clear(Color::WHITE);
                    renderer::render_frame(0, 12, 60, canvas);
                    win.surface.canvas().flush();
                    windowed_context.windowed().swap_buffers().unwrap();
                }
            }
            _ => (),
        }

        if windows.is_empty() {
            *control_flow = ControlFlow::Exit
        } else {
            *control_flow = ControlFlow::Wait
        }
    });
}
