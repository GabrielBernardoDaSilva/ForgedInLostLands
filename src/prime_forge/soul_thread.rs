#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct TemporalPause {
    pub amount_in_seconds: f32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum EssenceAspect {
    Running,
    Yielded(TemporalPause),
    Finished,
}

pub struct SoulThread {
    name: String,
    state: EssenceAspect,
    generator: Box<dyn FnMut() -> EssenceAspect + 'static>,
    is_waiting: bool,
    amount_to_wait: f32,
}

impl SoulThread {
    // Constructor to create a new coroutine
    pub fn new(name: &str, generator: impl FnMut() -> EssenceAspect + 'static) -> Self {
        Self {
            name: name.to_owned(),
            state: EssenceAspect::Running,
            generator: Box::new(generator),
            is_waiting: false,
            amount_to_wait: 0.0,
        }
    }

    // Function to resume execution of the coroutine
    fn resume(&mut self) -> Option<TemporalPause> {
        match self.state {
            EssenceAspect::Running => {
                let next_state = (self.generator)();
                self.state = next_state;
                self.resume()
            }
            EssenceAspect::Yielded(value) => {
                self.state = EssenceAspect::Running;
                Some(value)
            }
            EssenceAspect::Finished => {
                self.state = EssenceAspect::Finished;
                None
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_waiting {
            self.amount_to_wait -= delta_time;

            if self.amount_to_wait > 0.0 {
                return;
            }
            self.is_waiting = false;
        }

        if let Some(res) = self.resume() {
            self.is_waiting = true;
            self.amount_to_wait = res.amount_in_seconds;
        }
    }

    pub fn stop(&mut self) {
        self.state = EssenceAspect::Finished;
    }
}

pub struct SoulThreadManager {
    soul_threads: Vec<SoulThread>,
}

impl SoulThreadManager {
    pub fn new() -> Self {
        Self {
            soul_threads: Vec::new(),
        }
    }

    pub fn add_thread(&mut self, thread: SoulThread) {
        self.soul_threads.push(thread);
    }

    pub fn update(&mut self, delta_time: f32) {
        for thread in self.soul_threads.iter_mut() {
            if thread.state == EssenceAspect::Finished {
                continue;
            }
            thread.update(delta_time);
        }

        self.soul_threads
            .retain(|thread| thread.state != EssenceAspect::Finished);
    }

    pub fn stop_all(&mut self) {
        self.soul_threads
            .iter_mut()
            .for_each(|thread| thread.stop());
    }

    pub fn stop_by_name(&mut self, name: &str) {
        let soul_thread = self
            .soul_threads
            .iter_mut()
            .find(|thread| thread.name == name);
        if let Some(soul_thread) = soul_thread {
            soul_thread.stop();
        }
    }
}
