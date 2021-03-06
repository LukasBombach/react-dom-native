use crate::app::AppEvent;
use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::FsModuleLoader;
use deno_core::OpState;
use deno_core::Resource;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::tokio_util::create_basic_runtime;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use glutin::event_loop::EventLoopProxy;
use glutin::window::WindowId;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub fn run(
  event_loop_proxy: EventLoopProxy<AppEvent>,
  recv: Receiver<WindowId>,
  file_path: &str,
) -> Result<(), AnyError> {
  let module_loader = Rc::new(FsModuleLoader);
  let create_web_worker_cb = Arc::new(|_| {
    todo!("Web workers are not supported in the example");
  });

  let options = WorkerOptions {
    bootstrap: BootstrapOptions {
      apply_source_maps: false,
      args: vec![],
      cpu_count: 1,
      debug_flag: false,
      enable_testing_features: false,
      location: None,
      no_color: false,
      runtime_version: "x".to_string(),
      ts_version: "x".to_string(),
      unstable: false,
    },
    extensions: vec![],
    unsafely_ignore_certificate_errors: None,
    root_cert_store: None,
    user_agent: "hello_runtime".to_string(),
    seed: None,
    js_error_create_fn: None,
    create_web_worker_cb,
    maybe_inspector_server: None,
    should_break_on_first_statement: false,
    module_loader,
    get_error_class_fn: Some(&get_error_class_name),
    origin_storage_dir: None,
    blob_store: BlobStore::default(),
    broadcast_channel: InMemoryBroadcastChannel::default(),
    shared_array_buffer_store: None,
    compiled_wasm_module_store: None,
  };

  let main_module = deno_core::resolve_path(file_path).unwrap();
  let permissions = Permissions::allow_all();
  let bootstrap_options = options.bootstrap.clone();

  let mut worker = MainWorker::from_options(main_module.clone(), permissions, options);

  worker
    .js_runtime
    .op_state()
    .borrow_mut()
    .put::<EventLoopProxy<AppEvent>>(event_loop_proxy);

  worker
    .js_runtime
    .op_state()
    .borrow_mut()
    .put::<Receiver<WindowId>>(recv);

  worker
    .js_runtime
    .register_op("open_window", op_sync(open_window));

  worker.js_runtime.sync_ops_cache();

  let tokio_runtime = create_basic_runtime();
  tokio_runtime.block_on(async {
    worker.bootstrap(&bootstrap_options);
    (worker.execute_main_module(&main_module).await).unwrap();
    (worker.run_event_loop(false).await).unwrap();
  });

  Ok(())
}

#[allow(dead_code)]
struct WindowResource {
  window_id: WindowId,
}

impl Resource for WindowResource {
  fn name(&self) -> Cow<str> {
    "window".into()
  }
}

pub fn open_window(state: &mut OpState, _args: (), _: ()) -> Result<u32, AnyError> {
  let event_loop_proxy = state.borrow::<EventLoopProxy<AppEvent>>();
  let recv = state.borrow::<Receiver<WindowId>>();

  event_loop_proxy
    .send_event(AppEvent::NewWindowRequested)
    .ok();

  let window_id = recv.recv().unwrap();
  let window_resouce = WindowResource { window_id };

  Ok(state.resource_table.add(window_resouce))
}
