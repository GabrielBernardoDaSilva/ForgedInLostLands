use super::{destiny_rift::DestinyRiftManager, eonforge::EonForge, forged_object::ForgedObject};

pub struct LostRealm {
    pub forged_objects: Vec<ForgedObject>,
    pub eonforge: EonForge,
    pub destiny_rift_manager: DestinyRiftManager,
}

impl LostRealm {
    pub fn new() -> LostRealm {
        LostRealm {
            forged_objects: Vec::new(),
            eonforge: EonForge::new(),
            destiny_rift_manager: DestinyRiftManager::new(),
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
        self.eonforge.update();
        for object in &self.forged_objects {
            object.update();
        }
        self.destiny_rift_manager.remove_event();
    }
}
