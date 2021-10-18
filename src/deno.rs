use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::tokio_util::create_basic_runtime;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

// use std::borrow::Cow;
// use std::collections::HashMap;
// use std::path::Path;
// use std::rc::Rc;
// use std::sync::Arc;
// use std::thread;
// use deno_core::error::AnyError;
// use deno_core::op_sync;
// use deno_core::serde::Deserialize;
// use deno_core::FsModuleLoader;
// use deno_core::OpState;
// use deno_core::Resource;
// use deno_core::ResourceId;
// use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
// use deno_runtime::deno_web::BlobStore;
// use deno_runtime::permissions::Permissions;
// use deno_runtime::tokio_util::create_basic_runtime;
// use deno_runtime::worker::MainWorker;
// use deno_runtime::worker::WorkerOptions;

/* fn create_window(state: &mut OpState, args: CreateWindowArgs, _: ()) -> Result<u32, AnyError> {
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
} */

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub fn run(file_path: &str) -> Result<(), AnyError> {
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

  // let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);
  let mut worker = MainWorker::from_options(main_module.clone(), permissions, options);

  // worker.execute_main_module(&main_module).await?;
  // worker.run_event_loop(false).await?;

  let tokio_runtime = create_basic_runtime();
  tokio_runtime.block_on(async {
    worker.bootstrap(&bootstrap_options);
    (worker.execute_main_module(&main_module).await).unwrap();
    (worker.run_event_loop(false).await).unwrap();
  });

  Ok(())

  // let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.js");
  // let main_module = deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
  // let main_module = deno_core::resolve_path(file_path).unwrap();
  // let permissions = Permissions::allow_all();
  // let mut worker = MainWorker::from_options(main_module.clone(), permissions, &options);
  // let tokio_runtime = create_basic_runtime();

  // worker
  //   .js_runtime
  //   .op_state()
  //   .borrow_mut()
  //   .put::<EventLoopProxy<CustomEvent>>(event_loop_proxy);
  // worker
  //   .js_runtime
  //   .register_op("create_window", op_sync(create_window));
  // worker
  //   .js_runtime
  //   .register_op("remove_window", op_sync(remove_window));
  // worker.js_runtime.sync_ops_cache();

  // thread::park();

  // tokio_runtime.block_on(async {
  //   worker.bootstrap(&options);
  //   (worker.execute_module(&main_module).await).unwrap();
  //   (worker.run_event_loop(false).await).unwrap();
  // });
}
