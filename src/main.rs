use std::borrow::Cow;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

// use deno_core::error::bad_resource_id;
use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::FsModuleLoader;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;
use deno_core::ZeroCopyBuf;

use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::tokio_util::create_basic_runtime;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;

struct EventLoopResource(pub RefCell<EventLoop<()>>);

impl Resource for EventLoopResource {
    fn name(&self) -> Cow<str> {
        "eventLoop".into()
    }
}

struct WindowResource(pub Window);

impl WindowResource {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, AnyError> {
        Ok(Self(winit::window::Window::new(event_loop)?))
    }
}

impl Resource for WindowResource {
    fn name(&self) -> Cow<str> {
        "window".into()
    }
}

fn create_window(
    state: &mut OpState,
    rid: ResourceId,
    _zero_copy: Option<ZeroCopyBuf>,
) -> Result<ResourceId, AnyError> {
    let event_loop = state.resource_table.get::<EventLoopResource>(rid).unwrap();
    let event_loop = event_loop.0.borrow_mut();
    Ok(state.resource_table.add(WindowResource::new(&event_loop)?))
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

fn main() {
    // WINIT
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .unwrap();

    // DENO
    thread::spawn(move || {
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

        // worker
        //     .js_runtime
        //     .op_state()
        //     .borrow_mut()
        //     .resource_table
        //     .add(EventLoopResource(RefCell::new(EventLoop::new())));

        worker
            .js_runtime
            .register_op("create_window", op_sync(create_window));

        worker.js_runtime.sync_ops_cache();

        tokio_runtime.block_on(async {
            worker.bootstrap(&options);
            (worker.execute_module(&main_module).await).unwrap();
            (worker.run_event_loop(false).await).unwrap();
        });
    });

    // WINIT
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
