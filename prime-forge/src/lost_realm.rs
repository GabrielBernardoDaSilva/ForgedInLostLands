use crate::{arcane_weft::ArcaneWeft, forged_object::TraitBundle, forged_trait::ForgedTrait};

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

    fn add_object(&mut self, object: ForgedObject) {
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
    /// alias for adding a coroutine
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
    /// alias for adding event
    pub fn add_destiny_rift_event(&mut self, event: impl DestinyRift + 'static) {
        self.destiny_rift_manager.add_event(Box::new(event));
    }

    pub fn consume_destiny_rift_event<T: 'static + DestinyRift>(&mut self) -> Option<&T> {
        self.destiny_rift_manager.consume_event()
    }

    /// Arcane Weft functions
    /// alias for plugging
    pub fn arcane_weft_craft(&mut self, arcane_weft: impl ArcaneWeft) {
        arcane_weft.craft(self);
    }

    /// Forged Object functions
    /// alias for adding object
    pub fn forge_new_object_mut(
        &mut self,
        name: &str,
        traits: impl TraitBundle,
    ) -> Option<&mut ForgedObject> {
        let mut forged_object = ForgedObject::new(name.to_string());
        traits.craft_trait_bundle(&mut forged_object);
        self.add_object(forged_object);
        self.get_mut_forged_object(name)
    }

    pub fn forge_new_object(
        &mut self,
        name: &str,
        traits: impl TraitBundle,
    ) -> Option<&ForgedObject> {
        let mut forged_object = ForgedObject::new(name.to_string());
        traits.craft_trait_bundle(&mut forged_object);
        self.add_object(forged_object);
        self.get_forged_object(name)
    }

    pub fn get_forged_object(&self, name: &str) -> Option<&ForgedObject> {
        self.forged_objects
            .iter()
            .find(|object| object.name == name)
    }

    pub fn get_mut_forged_object(&mut self, name: &str) -> Option<&mut ForgedObject> {
        self.forged_objects
            .iter_mut()
            .find(|object| object.name == name)
    }

    pub fn get_forged_object_by_trait<T: 'static + ForgedTrait>(&self) -> Option<&ForgedObject> {
        self.forged_objects
            .iter()
            .find(|object| object.get_trait::<T>().is_ok())
    }

    pub fn get_trait_by_type<T: 'static + ForgedTrait>(&self) -> Option<&T> {
        self.forged_objects
            .iter()
            .find(|object| object.get_trait::<T>().is_ok())
            .and_then(|object| object.get_trait::<T>().ok())
    }
    pub fn get_mut_trait_by_type<T: 'static + ForgedTrait>(&mut self) -> Option<&mut T> {
        self.forged_objects
            .iter_mut()
            .find(|object| object.get_trait::<T>().is_ok())
            .and_then(|object| object.get_trait_mut::<T>().ok())
    }

    pub fn destroy_forged_object(&mut self, name: &str) -> Option<ForgedObject> {
        let index = self
            .forged_objects
            .iter()
            .position(|object| object.name == name);
        if let Some(index) = index {
            Some(self.forged_objects.remove(index))
        } else {
            None
        }
    }

    pub fn get_parent_forged_object(&self, trait_: &impl ForgedTrait) -> Option<&ForgedObject> {
        let father_id = trait_.get_father();
        if let Some(id) = father_id {
            self.forged_objects
                .iter()
                .find(|object| object.id == uuid::Uuid::parse_str(id.as_str()).unwrap())
        } else {
            None
        }
    }
}
