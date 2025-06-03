use instant::{Duration, Instant};
pub struct Profiler {
    last_frame_time: Instant,
    frame_count: u32,
    last_fps_update: Instant,
    current_fps: f32,
}

impl Profiler {
    pub fn update(&mut self) -> f32{
       let now = Instant::now();
        self.frame_count += 1;

        // 每秒更新一次FPS
        if now.duration_since(self.last_fps_update) >= Duration::from_secs(1) {
            self.current_fps = self.frame_count as f32 / 
                now.duration_since(self.last_fps_update).as_secs_f32();
            self.frame_count = 0;
            self.last_fps_update = now;
        }

        self.last_frame_time = now;
        self.current_fps
    }
}

impl Default for Profiler {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            last_frame_time: now,
            frame_count: 0,
            last_fps_update: now,
            current_fps: 0.0,
        }
    }
}
