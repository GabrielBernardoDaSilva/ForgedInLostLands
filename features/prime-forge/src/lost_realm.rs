use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{arcane_weft::ArcaneWeft, forged_object::TraitBundle, forged_trait::ForgedTrait};

use super::{
    destiny_rift::{DestinyRift, DestinyRiftManager},
    eonforge::EonForge,
    forged_object::ForgedObject,
    soul_thread::{SoulThread, SoulThreadManager},
};

pub struct LostRealm {
    forged_objects: Rc<RefCell<Vec<ForgedObject>>>,
    pub eonforge: Rc<RefCell<EonForge>>,
    destiny_rift_manager: Rc<RefCell<DestinyRiftManager>>,
    soul_threads_manager: Rc<RefCell<SoulThreadManager>>,
}

impl LostRealm {
    pub fn new() -> LostRealm {
        LostRealm {
            forged_objects: Rc::new(RefCell::new(Vec::new())),
            eonforge: Rc::new(RefCell::new(EonForge::new())),
            destiny_rift_manager: Rc::new(RefCell::new(DestinyRiftManager::new())),
            soul_threads_manager: Rc::new(RefCell::new(SoulThreadManager::new())),
        }
    }

    fn add_object(&self, object: ForgedObject) {
        self.forged_objects.borrow_mut().push(object);
    }

    pub fn start(&self) {
        self.eonforge.borrow_mut().start();
        for object in self.forged_objects.borrow_mut().iter() {
            object.start();
        }
    }

    pub fn update(&self) {
        for object in self.forged_objects.borrow_mut().iter() {
            object.update();
        }
        self.destiny_rift_manager.borrow_mut().remove_event();
        self.soul_threads_manager
            .borrow_mut()
            .update(self.eonforge.borrow().get_delta_time());
    }

    pub fn debug_update(&self) {
        loop {
            let dt = std::time::Duration::from_millis(1000 / 60);

            self.eonforge.borrow_mut().update(dt.as_secs_f32());
            self.update();

            std::thread::sleep(dt);
        }
    }

    //// Soul thread functions
    /// alias for adding a coroutine
    pub fn add_soul_thread(&self, thread: SoulThread) {
        self.soul_threads_manager.borrow_mut().add_thread(thread);
    }

    pub fn stop_all_soul_threads(&self) {
        self.soul_threads_manager.borrow_mut().stop_all();
    }

    pub fn stop_soul_thread_by_name(&self, name: &str) {
        self.soul_threads_manager.borrow_mut().stop_by_name(name);
    }

    //// Destiny rift functions
    /// alias for adding event
    pub fn add_destiny_rift_event(&self, event: impl DestinyRift + 'static) {
        self.destiny_rift_manager
            .borrow_mut()
            .add_event(Box::new(event));
    }

    pub fn consume_destiny_rift_event<T: 'static + DestinyRift>(&self) -> Option<&T> {
        let mut borrow_mut = self.destiny_rift_manager.borrow_mut();
        let rc = borrow_mut.consume_event();
        if let Some(rc) = rc {
            let rc = unsafe {
                let ptr = rc as *const T;
                &*ptr
            };
            Some(rc)
        } else {
            None
        }
    }

    /// Arcane Weft functions
    /// alias for plugging
    pub fn arcane_weft_craft(&self, arcane_weft: impl ArcaneWeft) {
        let mut_self = unsafe {
            let ptr = self as *const LostRealm;
            std::mem::transmute::<*const LostRealm, &mut LostRealm>(ptr)
        };
        arcane_weft.craft(mut_self);
    }

    /// Forged Object functions
    /// alias for adding object
    pub fn forge_new_object_mut(
        &self,
        name: &str,
        traits: impl TraitBundle,
    ) -> Option<&mut ForgedObject> {
        let mut forged_object = ForgedObject::new(name.to_string());
        traits.craft_trait_bundle(&mut forged_object);
        self.add_object(forged_object);
        self.get_mut_forged_object(name)
    }

    pub fn forge_new_object(&self, name: &str, traits: impl TraitBundle) -> Option<&ForgedObject> {
        let mut forged_object = ForgedObject::new(name.to_string());
        traits.craft_trait_bundle(&mut forged_object);
        self.add_object(forged_object);
        self.get_forged_object(name)
    }

    pub fn get_forged_object(&self, name: &str) -> Option<&ForgedObject> {
        let borrow = self.forged_objects.borrow();
        let rc = borrow.iter().find(|object| object.name == name);
        if let Some(rc) = rc {
            let rc = unsafe {
                let ptr = rc as *const ForgedObject;
                &*ptr
            };
            Some(rc)
        } else {
            None
        }
    }

    pub fn get_mut_forged_object(&self, name: &str) -> Option<&mut ForgedObject> {
        let mut borrow_mut = self.forged_objects.borrow_mut();
        let rc = borrow_mut.iter_mut().find(|object| object.name == name);
        if let Some(rc) = rc {
            let rc = unsafe {
                let ptr = rc as *const ForgedObject;
                &mut *(ptr as *mut ForgedObject)
            };
            Some(rc)
        } else {
            None
        }
    }

    pub fn get_forged_object_by_trait<T: 'static + ForgedTrait>(&self) -> Option<&ForgedObject> {
        let borrow = self.forged_objects.borrow();
        let rc = borrow.iter().find(|object| object.get_trait::<T>().is_ok());
        if let Some(rc) = rc {
            let rc = unsafe {
                let ptr = rc as *const ForgedObject;
                &*ptr
            };
            Some(rc)
        } else {
            None
        }
    }

    pub fn get_trait_by_type<T: 'static + ForgedTrait>(&self) -> Option<&T> {
        let borrow = self.forged_objects.borrow();
        let rc = borrow
            .iter()
            .find(|object| object.get_trait::<T>().is_ok())
            .and_then(|object| object.get_trait::<T>().ok());
        if let Some(rc) = rc {
            let rc = unsafe {
                let ptr = rc as *const T;
                &*ptr
            };
            Some(rc)
        } else {
            None
        }
    }
    pub fn get_mut_trait_by_type<T: 'static + ForgedTrait>(&self) -> Option<&mut T> {
        let mut borrow_mut = self.forged_objects.borrow_mut();
        let rc = borrow_mut
            .iter_mut()
            .find(|object| object.get_trait::<T>().is_ok())
            .and_then(|object| object.get_trait_mut::<T>().ok());
        if let Some(rc) = rc {
            let rc = unsafe {
                let ptr = rc as *const T;
                &mut *(ptr as *mut T)
            };
            Some(rc)
        } else {
            None
        }
    }

    pub fn destroy_forged_object(&self, name: &str) -> Option<ForgedObject> {
        let index = self
            .forged_objects
            .borrow()
            .iter()
            .position(|object| object.name == name);
        if let Some(index) = index {
            Some(self.forged_objects.borrow_mut().remove(index))
        } else {
            None
        }
    }

    pub fn destroy_forged_object_by_ref(
        &self,
        forged_object: &ForgedObject,
    ) -> Option<ForgedObject> {
        let index = self
            .forged_objects
            .borrow()
            .iter()
            .position(|object| object.id == forged_object.id);
        if let Some(index) = index {
            Some(self.forged_objects.borrow_mut().remove(index))
        } else {
            None
        }
    }

    pub fn get_parent_forged_object(&self, trait_: &impl ForgedTrait) -> Option<&ForgedObject> {
        let father_id = trait_.get_father();
        if let Some(id) = father_id {
            let borrow = self.forged_objects.borrow();
            let rc = borrow
                .iter()
                .find(|object| object.id == uuid::Uuid::parse_str(id.as_str()).unwrap());
            if let Some(rc) = rc {
                let rc = unsafe {
                    let ptr = rc as *const ForgedObject;
                    &*ptr
                };
                Some(rc)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_mut_parent_forged_object(
        &self,
        trait_: &impl ForgedTrait,
    ) -> Option<&mut ForgedObject> {
        let father_id = trait_.get_father();
        if let Some(id) = father_id {
            let mut borrow_mut = self.forged_objects.borrow_mut();
            let rc = borrow_mut
                .iter_mut()
                .find(|object| object.id == uuid::Uuid::parse_str(id.as_str()).unwrap());
            if let Some(rc) = rc {
                let rc = unsafe {
                    let ptr = rc as *const ForgedObject;
                    &mut *(ptr as *mut ForgedObject)
                };
                Some(rc)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_all_forged_objects_by_trait<T: 'static + ForgedTrait>(&self) -> Vec<&ForgedObject> {
        let borrow = self.forged_objects.borrow();
        let rc = borrow
            .iter()
            .filter(|object| object.get_trait::<T>().is_ok())
            .map(|object| unsafe {
                let ptr = object as *const ForgedObject;
                &*ptr
            })
            .collect();
        rc
    }

    pub fn get_all_trait_by_type<T: 'static + ForgedTrait>(&self) -> Vec<&T> {
        let borrow = self.forged_objects.borrow();
        let rc = borrow
            .iter()
            .filter(|object| object.get_trait::<T>().is_ok())
            .map(|object| object.get_trait::<T>().unwrap())
            .map(|trait_| unsafe {
                let ptr = trait_ as *const T;
                &*ptr
            })
            .collect();
        rc
    }

    pub fn get_mut_all_forged_objects_by_trait<T: 'static + ForgedTrait>(
        &self,
    ) -> Vec<&mut ForgedObject> {
        let mut borrow_mut = self.forged_objects.borrow_mut();
        let rc = borrow_mut
            .iter_mut()
            .filter(|object| object.get_trait::<T>().is_ok())
            .map(|object| unsafe {
                let ptr = object as *const ForgedObject;
                &mut *(ptr as *mut ForgedObject)
            })
            .collect();
        rc
    }

    pub fn get_mut_all_trait_by_type<T: 'static + ForgedTrait>(&self) -> Vec<&mut T> {
        let mut borrow_mut = self.forged_objects.borrow_mut();
        let rc = borrow_mut
            .iter_mut()
            .filter(|object| object.get_trait::<T>().is_ok())
            .map(|object| object.get_trait_mut::<T>().unwrap())
            .map(|trait_| unsafe {
                let ptr = trait_ as *const T;
                &mut *(ptr as *mut T)
            })
            .collect();
        rc
    }

    // EonForge
    pub fn get_delta_time(&self) -> f32 {
        self.eonforge.borrow().get_delta_time()
    }

    pub fn get_time_elapsed(&self) -> Duration {
        self.eonforge.borrow().get_time_elapsed_since_start()
    }
}
