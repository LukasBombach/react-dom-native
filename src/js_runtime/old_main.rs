use std::borrow::Cow;

use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use winit::dpi::LogicalPosition;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::StartCause;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use winit::window::WindowBuilder;

use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::serde::Deserialize;
use deno_core::FsModuleLoader;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;

use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::tokio_util::create_basic_runtime;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;

enum CustomEvent {
  RequestCreateWindow(CreateWindowArgs),
  RequestRemoveWindow(u32),
}

struct WindowResource {}

impl Resource for WindowResource {
  fn name(&self) -> Cow<str> {
    "window".into()
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateWindowArgs {
  rid: ResourceId,
  title: String,
  size: LogicalSize<f32>,
  position: LogicalPosition<f32>,
}

impl Default for CreateWindowArgs {
  fn default() -> CreateWindowArgs {
    CreateWindowArgs {
      rid: 0,
      title: "".to_string(),
      size: LogicalSize::new(640.0, 480.0),
      position: LogicalPosition::new(300.0, 300.0),
    }
  }
}

fn create_window(state: &mut OpState, args: CreateWindowArgs, _: ()) -> Result<u32, AnyError> {
  let rid = state.resource_table.add(WindowResource {});
  let args = CreateWindowArgs { rid, ..args };

  state
    .borrow::<EventLoopProxy<CustomEvent>>()
    .send_event(CustomEvent::RequestCreateWindow(args))
    .ok();

  Ok(rid)
}

fn remove_window(state: &mut OpState, rid: ResourceId, _: ()) -> Result<(), AnyError> {
  state.resource_table.close(rid).ok();

  state
    .borrow::<EventLoopProxy<CustomEvent>>()
    .send_event(CustomEvent::RequestRemoveWindow(rid))
    .ok();

  Ok(())
}

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

fn main() {
  // WINIT
  let event_loop = EventLoop::<CustomEvent>::with_user_event();
  let event_loop_proxy = event_loop.create_proxy();
  let mut windows: HashMap<u32, Window> = HashMap::new();

  // DENO
  let js_thread = thread::spawn(move || {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
      todo!("Web workers are not supported in the example");
    });
    let options = WorkerOptions {
      apply_source_maps: false,
      args: vec![],
      debug_flag: false,
      unstable: false,
      enable_testing_features: false,
      unsafely_ignore_certificate_errors: None,
      root_cert_store: None,
      user_agent: "hello_runtime".to_string(),
      seed: None,
      js_error_create_fn: None,
      create_web_worker_cb,
      maybe_inspector_server: None,
      should_break_on_first_statement: false,
      module_loader,
      runtime_version: "x".to_string(),
      ts_version: "x".to_string(),
      no_color: false,
      get_error_class_fn: Some(&get_error_class_name),
      location: None,
      origin_storage_dir: None,
      blob_store: BlobStore::default(),
      broadcast_channel: InMemoryBroadcastChannel::default(),
      shared_array_buffer_store: None,
      cpu_count: 1,
    };
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.js");
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
    let permissions = Permissions::allow_all();
    let mut worker = MainWorker::from_options(main_module.clone(), permissions, &options);
    let tokio_runtime = create_basic_runtime();

    worker
      .js_runtime
      .op_state()
      .borrow_mut()
      .put::<EventLoopProxy<CustomEvent>>(event_loop_proxy);

    worker
      .js_runtime
      .register_op("create_window", op_sync(create_window));

    worker
      .js_runtime
      .register_op("remove_window", op_sync(remove_window));

    worker.js_runtime.sync_ops_cache();

    thread::park();

    tokio_runtime.block_on(async {
      worker.bootstrap(&options);
      (worker.execute_module(&main_module).await).unwrap();
      (worker.run_event_loop(false).await).unwrap();
    });
  });

  // WINIT
  event_loop.run(move |event, event_loop, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => {
        js_thread.thread().unpark();
      }
      Event::UserEvent(CustomEvent::RequestCreateWindow(args)) => {
        let window = WindowBuilder::new()
          .with_title(args.title)
          .with_inner_size(args.size)
          .with_position(args.position)
          .build(&event_loop)
          .unwrap();
        windows.insert(args.rid, window);
      }
      Event::UserEvent(CustomEvent::RequestRemoveWindow(rid)) => {
        windows.remove(&rid);
      }
      _ => (),
    }
  });
}
