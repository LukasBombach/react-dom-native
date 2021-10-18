use crate::deno;
use crate::renderer;
use crate::support;

use gl::types::*;
use glutin::window::WindowBuilder;
use glutin::window::WindowId;
use glutin::ContextBuilder;
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::BackendRenderTarget;
use skia_safe::gpu::SurfaceOrigin;
use skia_safe::Color;
use skia_safe::ColorType;
use skia_safe::Surface;
use std::collections::HashMap;
use std::convert::TryInto;
use std::thread;
use support::ContextCurrentWrapper;
use support::ContextTracker;
use support::ContextWrapper;

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

#[derive(Debug, Clone, Copy)]
pub enum AppEvent {
  NewWindowRequested,
  QuitAppRequested,
}

pub struct Win {
  context_id: usize,
  surface: Surface,
  gr_context: skia_safe::gpu::DirectContext,
  fb_info: FramebufferInfo,
}

pub struct App {
  windows: HashMap<glutin::window::WindowId, Win>,
  ct: ContextTracker,
}

impl App {
  pub fn new(el_proxy: glutin::event_loop::EventLoopProxy<AppEvent>) -> Self {
    thread::spawn(move || {
      deno::run(el_proxy, "src/main.js").unwrap();
    });

    App {
      windows: HashMap::new(),
      ct: ContextTracker::default(),
    }
  }

  pub fn init(&mut self) {}

  pub fn create_window(&mut self, el: &glutin::event_loop::EventLoopWindowTarget<AppEvent>) {
    let wb = WindowBuilder::new().with_title("Charming Window");
    let windowed_context = ContextBuilder::new().build_windowed(wb, el).unwrap();
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
    let context_id = self.ct.insert(ContextCurrentWrapper::PossiblyCurrent(
      ContextWrapper::Windowed(windowed_context),
    ));

    let win = Win {
      context_id,
      surface,
      gr_context,
      fb_info,
    };

    self.windows.insert(window_id, win);
  }

  #[allow(deprecated)]
  pub fn render(&mut self, window_id: WindowId) {
    if let Some(win) = self.windows.get_mut(&window_id) {
      let windowed_context = self.ct.get_current(win.context_id).unwrap();

      let canvas = win.surface.canvas();
      canvas.clear(Color::WHITE);
      renderer::render_frame(0, 12, 60, canvas);
      win.surface.canvas().flush();
      windowed_context.windowed().swap_buffers().unwrap();
    }
  }

  pub fn resize_window(
    &mut self,
    window_id: WindowId,
    physical_size: glutin::dpi::PhysicalSize<u32>,
  ) {
    if let Some(win) = self.windows.get_mut(&window_id) {
      let windowed_context = self.ct.get_current(win.context_id).unwrap();

      win.surface = create_surface(
        windowed_context.windowed(),
        &win.fb_info,
        &mut win.gr_context,
      );
      windowed_context.windowed().resize(physical_size);
    }
  }

  pub fn remove_window(&mut self, window_id: WindowId) {
    if let Some(win) = self.windows.remove(&window_id) {
      self.ct.remove(win.context_id);
    }
  }
}
