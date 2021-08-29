use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::serde::Deserialize;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;

pub fn create_window(state: &mut OpState, args: (), _: ()) -> Result<u32, AnyError> {
  let rid = state.resource_table.add(WindowResource {});

  state
    .borrow::<EventLoopProxy<CustomEvent>>()
    .send_event(CustomEvent::RequestCreateWindow(args))
    .ok();

  Ok(rid)
}
