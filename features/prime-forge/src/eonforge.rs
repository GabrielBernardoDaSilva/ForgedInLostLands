use std::time::Duration;

pub struct EonForge {
    delta_time: f32,
    time_elapsed_since_start: Duration,
    last_time: f32,
}

impl EonForge {
    pub fn new() -> EonForge {
        EonForge {
            delta_time: 0.0,
            time_elapsed_since_start: Duration::new(0, 0),
            last_time: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.time_elapsed_since_start = Duration::from_secs(0);
    }

    pub fn update(&mut self, current_time: f32) {
        self.time_elapsed_since_start += Duration::from_secs(1);
        self.delta_time = current_time - self.last_time;
        self.last_time = current_time;
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_time_elapsed_since_start(&self) -> Duration {
        self.time_elapsed_since_start
    }
}


impl Default for EonForge {
    fn default() -> Self {
        Self::new()
    }
}