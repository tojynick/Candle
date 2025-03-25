use std::time::Instant;

pub struct FPSCounter {
    last_frame_time: Instant,
    fps_samples: [f32; 60],
    sample_index: usize,

    pub fps: f32
}

impl FPSCounter {

    pub fn new() -> Self {

        Self {
            last_frame_time: Instant::now(),
            fps_samples: [0.0; 60],
            sample_index: 0,
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {

        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        if delta_time > 0.0 {
            let current_fps = 1.0 / delta_time;
            self.fps_samples[self.sample_index] = current_fps;
            self.sample_index = (self.sample_index + 1) % self.fps_samples.len();
        }

        self.fps = self.fps_samples.iter().sum::<f32>() / self.fps_samples.len() as f32;

    }
}