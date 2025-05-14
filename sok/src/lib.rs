use std::{rc::Rc, sync::{Arc, Mutex}};

use wgpu::WasmNotSend;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-sok")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    event_loop
        .run(move |event, control_flow| match event {
            Event::Resumed => {
                log::debug!("Resumed");
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested //events
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}

//surface
struct WgpuApp {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    size_changed: bool,
}

impl WgpuApp {
    async fn new(window: Arc<Window>) -> Self{
        
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                // WebGL not supported all wgpu features
                // web 
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
                memory_hints: wgpu::MemoryHints::Performance,
                //trace API calls
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
            present_mode:wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size: PhysicalSize::new(640, 480),
            size_changed: false,
        }
        
    }
    //
    fn set_window_resized(&mut self, new_size: PhysicalSize<u32>) {
        todo!()
    }

    fn resize_surface_if_needed(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}

struct WgpuAppHandler{
    app:Rc<Mutex<Option<WgpuApp>>>,
    /// When not initialized, this may called
    #[allow(dead_code)]
    missed_resize: Rc<Mutex<Option<PhysicalSize<u32>>>>,

    /// When not initialized, this may called
    #[allow(dead_code)]
    missed_request_redraw: Rc<Mutex<bool>>,
}

impl ApplicationHandler for WgpuAppHandler{
//     fn resumed(&mut self, event_loop: &ActiveEventLoop){

//     })
}

//for user action when using wgpu
pub trait WgpuAppAction {
    fn new(window: Arc<Window>) -> impl core::future::Future<Output = Self> + WasmNotSend;
    /// record size of window when resized
    fn set_window_resized(&mut self, new_size: PhysicalSize<u32>);
    /// get size of window
    fn get_size(&self) -> PhysicalSize<u32>;
    /// keyboard event
    fn keyboard_input(&mut self, _event: &KeyEvent) -> bool;
    fn mouse_click(&mut self, _state: ElementState, _button: MouseButton) -> bool;
    fn mouse_wheel(&mut self, _delta: MouseScrollDelta, _phase: TouchPhase) -> bool;
    fn cursor_move(&mut self, _position: PhysicalPosition<f64>) -> bool;
    /// move/touch event
    fn device_input(&mut self, _event: &DeviceEvent) -> bool;
    /// rendering data event
    fn update(&mut self, _dt: instant::Duration) {}
    /// render
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}
