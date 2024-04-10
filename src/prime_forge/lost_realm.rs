use super::{
    destiny_rift::{DestinyRift, DestinyRiftManager},
    eonforge::EonForge,
    forged_object::ForgedObject,
    soul_thread::{SoulThread, SoulThreadManager},
};

pub struct LostRealm {
    forged_objects: Vec<ForgedObject>,
    pub eonforge: EonForge,
    destiny_rift_manager: DestinyRiftManager,
    soul_threads_manager: SoulThreadManager,
}

impl LostRealm {
    pub fn new() -> LostRealm {
        LostRealm {
            forged_objects: Vec::new(),
            eonforge: EonForge::new(),
            destiny_rift_manager: DestinyRiftManager::new(),
            soul_threads_manager: SoulThreadManager::new(),
        }
    }

    pub fn add_object(&mut self, object: ForgedObject) {
        self.forged_objects.push(object);
    }

    pub fn start(&mut self) {
        self.eonforge.start();
        for object in &self.forged_objects {
            object.start();
        }
    }

    pub fn update(&mut self) {
        for object in &self.forged_objects {
            object.update();
        }
        self.destiny_rift_manager.remove_event();
        self.soul_threads_manager
            .update(self.eonforge.get_delta_time());
    }

    pub fn debug_update(&mut self) {
        loop {
            let dt = std::time::Duration::from_millis(1000 / 60);
            
            self.eonforge.update(dt.as_secs_f32());
            self.update();

            std::thread::sleep(dt);
        }
    }

    //// Soul thread functions
    pub fn add_soul_thread(&mut self, thread: SoulThread) {
        self.soul_threads_manager.add_thread(thread);
    }

    pub fn stop_all_soul_threads(&mut self) {
        self.soul_threads_manager.stop_all();
    }

    pub fn stop_soul_thread_by_name(&mut self, name: &str) {
        self.soul_threads_manager.stop_by_name(name);
    }

    //// Destiny rift functions
    pub fn add_destiny_rift_event(&mut self, event: impl DestinyRift + 'static) {
        self.destiny_rift_manager.add_event(Box::new(event));
    }

    pub fn consume_destiny_rift_event<T: 'static + DestinyRift>(&mut self) -> Option<&T> {
        self.destiny_rift_manager.consume_event()
    }
}
