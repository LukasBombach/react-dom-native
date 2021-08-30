use crate::app_handler;

use app_handler::AppHandler;

use std::borrow::Cow;

use deno_core::error::AnyError;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;

use glutin::window::Window;

#[allow(dead_code)]
struct WindowResource {
  window: Window,
}

impl Resource for WindowResource {
  fn name(&self) -> Cow<str> {
    "window".into()
  }
}

pub fn open_window(state: &mut OpState, _args: (), _: ()) -> Result<u32, AnyError> {
  let app_handler = state.borrow_mut::<AppHandler>();

  let window = app_handler.create_window();
  let window_resouce = WindowResource { window };

  Ok(state.resource_table.add(window_resouce))
}

pub fn close_window(state: &mut OpState, rid: ResourceId, _: ()) -> Result<(), AnyError> {
  state.resource_table.close(rid).ok();
  Ok(())
}
