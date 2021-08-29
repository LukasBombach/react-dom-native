mod app_handler;

use app_handler::AppEvent;

use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::serde::Deserialize;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;

pub fn open_window(state: &mut OpState, args: (), _: ()) -> Result<u32, AnyError> {
  let rid = state.resource_table.add(WindowResource {});

  state
    .borrow::<EventLoopProxy<AppEvent>>()
    .send_event(AppEvent::CreateWindow)
    .ok();

  let window = self.recv.recv().unwrap();

  Ok(rid)
}
