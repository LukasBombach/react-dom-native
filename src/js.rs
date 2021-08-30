mod app_handler;
mod window;

use app_handler::AppHandler;

use std::rc::Rc;
use std::sync::Arc;

use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;

use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;

use tokio::runtime::Builder;

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub struct Runtime {
  options: WorkerOptions,
}

impl Runtime {
  pub fn new() -> Self {
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

    Self { options }
  }

  pub fn run(&mut self, js_path: &str, app_handler: AppHandler) {
    let main_module = deno_core::resolve_path(js_path).unwrap();
    let mut main_worker =
      MainWorker::from_options(main_module.clone(), Permissions::allow_all(), &self.options);

    worker
      .js_runtime
      .op_state()
      .borrow_mut()
      .put::<AppHandler>(app_handler);

    worker
      .js_runtime
      .register_op("open_window", op_sync(window::open_window));

    worker.js_runtime.sync_ops_cache();

    let tokio_runtime = Builder::new_current_thread()
      .enable_io()
      .enable_time()
      .max_blocking_threads(32)
      .build()
      .unwrap();

    tokio_runtime.block_on(async {
      main_worker.bootstrap(&self.options);
      (main_worker.execute_module(&main_module).await).unwrap();
      (main_worker.run_event_loop(false).await).unwrap();
    });
  }
}
