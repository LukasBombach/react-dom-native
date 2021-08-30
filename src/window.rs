use crate::app_handler;

use app_handler::AppEvent;
use app_handler::AppHandler;

use std::borrow::Cow;

use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::serde::Deserialize;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;

use winit::window::Window;

struct WindowResource {
  window: Window,
}

impl Resource for WindowResource {
  fn name(&self) -> Cow<str> {
    "window".into()
  }
}

pub fn open_window(state: &mut OpState, args: (), _: ()) -> Result<u32, AnyError> {
  let app_handler = state.borrow_mut::<AppHandler>();

  let window = app_handler.create_window();
  let window_resouce = WindowResource { window };

  Ok(state.resource_table.add(window_resouce))
}
