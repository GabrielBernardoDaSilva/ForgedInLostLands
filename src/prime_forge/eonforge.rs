use std::time::Duration;

pub struct EonForge {
    delta_time: f32,
    time_elapsed_since_start: Duration,
}

impl EonForge {
    pub fn new() -> EonForge {
        EonForge {
            delta_time: 0.0,
            time_elapsed_since_start: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.time_elapsed_since_start = self.time_elapsed_since_start + Duration::new(1, 0);
        self.delta_time = 1.0;
    }

    pub fn update(&mut self) {
        self.time_elapsed_since_start = self.time_elapsed_since_start + Duration::new(1, 0);
        self.delta_time = 1.0;
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_time_elapsed_since_start(&self) -> Duration {
        self.time_elapsed_since_start
    }
}
