use std::{rc::Rc, sync::Arc};

use parking_lot::Mutex;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

struct WgpuApp {
    // 用Arc來在執行緒之間共享對Window的引用計數
    // 允許未使用，這可以避免視窗被釋放
    #[allow(unused)]
    window: Arc<Window>,
}

impl WgpuApp {
    async fn new(window: Arc<Window>) -> Self {
        Self { window }
    }
}

#[derive(Default)]
struct WgpuAppHandler {
    app: Rc<Mutex<Option<WgpuApp>>>,
}

impl ApplicationHandler for WgpuAppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 當恢復時

        // 如果app的引用計數不為0，則不創建新的app, 提前離開
        // 這樣可以避免在應用程序恢復時創建新的app
        if self.app.as_ref().lock().is_some() {
            return;
        }

        // 建立視窗跟他的屬性
        let window_attributes = Window::default_attributes().with_title("tutorial1-window");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let wgpu_app = pollster::block_on(WgpuApp::new(window));
        self.app.lock().replace(wgpu_app);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // 當暫停時
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(_physical_size) => {
                // 當視窗大小改變時
                // 這裡可以獲取新的視窗大小
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { .. } => {
                // 鍵盤事件
            }
            WindowEvent::RedrawRequested => {
                // 重新繪製
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), impl std::error::Error> {
    env_logger::init();

    let events_loop = EventLoop::new().unwrap();
    let mut app = WgpuAppHandler::default();
    events_loop.run_app(&mut app)
}
