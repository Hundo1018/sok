use std::{
    sync::{Arc, Mutex},
    vec,
};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::*,
    event_loop::{self, ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct WgpuApp {
    /// 避免窗口被释放
    #[allow(unused)]
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

/// WGPU's Instance will be in here
impl WgpuApp {
    async fn new(window: Arc<Window>) -> Self {
        //WGPU instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        //create a surface for window
        let surface = instance.create_surface(window.clone()).unwrap();

        //adapter is for choosing WebGPU API instance
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        //real device and command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let caps = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: 640,
            height: 480,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        //window to canvas
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-sok")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                log::debug!("{:?}", dst);
                Some(())
            })
            .expect("Couldn't append canvas to document body.");

        Self {
            window,
            surface,
            device,
            queue,
            config,
        }
    }
        /// 记录窗口大小已发生变化
    ///
    /// # NOTE:
    /// 当缩放浏览器窗口时, 窗口大小会以高于渲染帧率的频率发生变化，
    /// 如果窗口 size 发生变化就立即调整 surface 大小, 会导致缩放浏览器窗口大小时渲染画面闪烁。
    fn set_window_resized(&mut self, new_size: PhysicalSize<u32>) {
        todo!()
    }

    /// 必要的时候调整 surface 大小
    fn resize_surface_if_needed(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}

#[derive(Default)]
struct WgpuAppHandler {
    app: Arc<Mutex<Option<WgpuApp>>>,
    /// 错失的窗口大小变化
    ///
    /// # NOTE：
    /// 在 web 端，app 的初始化是异步的，当收到 resized 事件时，初始化可能还没有完成从而错过窗口 resized 事件，
    /// 当 app 初始化完成后会调用 `set_window_resized` 方法来补上错失的窗口大小变化事件。
    #[allow(dead_code)]
    missed_resize: Arc<Mutex<Option<PhysicalSize<u32>>>>,
}
impl ApplicationHandler for WgpuAppHandler {
    /// 恢复事件
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 如果 app 已经初始化完成，则直接返回
        if self.app.as_ref().lock().unwrap().is_some() {
            return;
        }

        let window_attributes = Window::default_attributes().with_title("tutorial2-surface");

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let app = self.app.clone();
        let missed_resize = self.missed_resize.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let window_cloned = window.clone();

            let wgpu_app = WgpuApp::new(window).await;
            let mut app = app.lock();
            // *app = Some(wgpu_app);

            // 如果错失了窗口大小变化事件，则补上
            // if let Some(resize) = *missed_resize.lock().unwrap() {
            //     app.as_mut().unwrap().unwrap().set_window_resized(resize);
            //     window_cloned.request_redraw();
            // }
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        log::debug!("event!");
        todo!()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
    let event_loop = EventLoop::new().unwrap();
    let mut app = WgpuAppHandler::default();
    event_loop.run_app(&mut app);
}
