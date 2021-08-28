use std::rc::Rc;
use std::sync::Arc;

use deno_core::error::AnyError;
use deno_core::FsModuleLoader;

use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::tokio_util::create_basic_runtime;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub fn run(js_path: &str) {
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
    module_loader: Rc::new(FsModuleLoader),
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

  let main_module = deno_core::resolve_path(js_path).unwrap();

  let mut worker =
    MainWorker::from_options(main_module.clone(), Permissions::allow_all(), &options);
  let tokio_runtime = create_basic_runtime();

  tokio_runtime.block_on(async {
    worker.bootstrap(&options);
    (worker.execute_module(&main_module).await).unwrap();
    (worker.run_event_loop(false).await).unwrap();
  });
}
