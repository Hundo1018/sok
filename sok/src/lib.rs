mod profiler;
use profiler::Profiler;
use wasm_bindgen::convert::IntoWasmAbi;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use parking_lot::Mutex;
use std::sync::Arc;
use web_sys::{js_sys::Function, HtmlCanvasElement};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct WgpuApp {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    size_changed: bool,
    render_pipeline: wgpu::RenderPipeline,
}

/// WGPU's Instance will be in here
impl WgpuApp {
    async fn new(window: Arc<Window>) -> Self {
        let size: PhysicalSize<u32> = PhysicalSize {
            width: 800,
            height: 600,
        };
        #[cfg(target_arch = "wasm32")]
        {
            //window to canvas
            use winit::platform::web::WindowExtWebSys;
            let canvas = window.canvas().unwrap();

            web_sys::window()
                .and_then(|win| win.document())
                .map(|doc| {
                    let _ = canvas.set_attribute("id", "wasm-sok");
                    match doc.get_element_by_id("wasm-container") {
                        Some(dst) => {
                            let _ = dst.append_child(canvas.as_ref());
                        }
                        None => {
                            let container = doc.create_element("div").unwrap();

                            let _ = container.set_attribute("id", "wasm-container");
                            let _ = container.append_child(canvas.as_ref());
                            doc.body().map(|body| body.append_child(container.as_ref()));
                        }
                    };
                })
                .expect("Couldn't append canvas to document body.");

            // make sure forcus
            canvas.set_tab_index(0);

            let style = canvas.style();
            let _ = style.set_property("width", "100vw");
            let _ = style.set_property("height", "100vh");
            style.set_property("outline", "none").unwrap();
            canvas.focus().expect("canvs cannot get focus");
        }

        //WGPU instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        log::debug!("instanced");

        //create a surface for window
        let surface = instance.create_surface(window.clone()).unwrap();
        log::debug!("surfaced");

        //adapter is for choosing WebGPU API instance
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        log::debug!("adaptered");

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
        log::debug!("device & queue");

        let caps = surface.get_capabilities(&adapter);
        log::debug!("caps ed");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        // let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                //vertex shaer
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                //fragment shader
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState { 
                count: 1,
                 mask: !0,
                  alpha_to_coverage_enabled: false },
            multiview: None,
            cache: None,
        });
        surface.configure(&device, &config);
        Self {
            window,
            surface,
            _adapter: adapter,
            device,
            queue,
            config,
            size,
            size_changed: false,
            render_pipeline,
        }
    }

    /// 记录窗口大小已发生变化
    ///
    /// # NOTE:
    /// 当缩放浏览器窗口时, 窗口大小会以高于渲染帧率的频率发生变化，
    /// 如果窗口 size 发生变化就立即调整 surface 大小, 会导致缩放浏览器窗口大小时渲染画面闪烁。
    fn set_window_resized(&mut self, new_size: PhysicalSize<u32>) {
        if new_size == self.size {
            return;
        }
        self.size = new_size;
        self.size_changed = true;
    }

    /// 必要的时候调整 surface 大小
    fn resize_surface_if_needed(&mut self) {
        if self.size_changed {
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.surface.configure(&self.device, &self.config);
            self.size_changed = false;
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }
        self.resize_surface_if_needed();
        // wait for surface to provide a new surfaceTexture
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }
        // submit can accept any parameter impl trait IntoIter
        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }

    fn keyboard_input(&mut self, _event: &KeyEvent) -> bool {
        false
    }

    fn mouse_click(&mut self, _state: ElementState, _button: MouseButton) -> bool {
        false
    }

    fn mouse_wheel(&mut self, _delta: MouseScrollDelta, _phase: TouchPhase) -> bool {
        false
    }

    fn cursor_move(&mut self, position: PhysicalPosition<f64>) -> bool {
        true
    }

    /// mouse move/touch
    fn device_input(&mut self, _event: &DeviceEvent) -> bool {
        false
    }

    fn update(&mut self) {
        //
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
    profiler: Profiler,
}
impl ApplicationHandler for WgpuAppHandler {
    /// 恢复事件
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 如果 app 已经初始化完成，则直接返回
        if self.app.as_ref().lock().is_some() {
            return;
        }
        log::debug!("resumed! in applicationHandler");
        let window_attributes = Window::default_attributes().with_title("wasm-sok");

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        cfg_if::cfg_if! {
            if #[cfg(target_arch="wasm32")]{
                let app = self.app.clone();
                let missed_resize = self.missed_resize.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let window_cloned = window.clone();

                    let wgpu_app = WgpuApp::new(window).await;

                    log::debug!("wgpu_app created!");


                    let mut app = app.lock();
                    *app = Some(wgpu_app);

                    //如果错失了窗口大小变化事件，则补上
                    if let Some(resize) = *missed_resize.lock() {
                        app.as_mut().unwrap().set_window_resized(resize);
                        window_cloned.request_redraw();
                    }
                    log::debug!("end");

                });
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut app = self.app.lock();
        if app.as_ref().is_none() {
            if let WindowEvent::Resized(physical_size) = event {
                if physical_size.width > 0 && physical_size.height > 0 {
                    let mut missed_resize = self.missed_resize.lock();
                    *missed_resize = Some(physical_size);
                }
            }
            return;
        }
        let app = app.as_mut().unwrap();

        match event {
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    log::debug!("window minized");
                } else {
                    log::debug!("window resized {:?}", physical_size);
                    app.set_window_resized(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                app.window.pre_present_notify();
                match app.render() {
                    Ok(_) => {}
                    // when lost context of suface
                    Err(wgpu::SurfaceError::Lost) => eprintln!("Surface is lost"),
                    // other error like exp, should be resolve in next frame
                    Err(e) => eprintln!("{e:?}"),
                }
                let fps = self.profiler.update();
                log::debug!("fps:{:?}",fps);
                app.window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                // log::debug!("{:?},{}", event.logical_key, is_synthetic);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                // log::debug!("{:?}", position);
            }
            _ => {}
        }
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
