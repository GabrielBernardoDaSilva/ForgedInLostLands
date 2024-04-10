use std::cell::RefCell;

use super::{forged_trait::ForgedTrait, lost_lands_fault::LostLostLandsFaultForgedObject};

pub struct ForgedObject {
    pub name: String,
    pub id: u32,
    pub forged_traits: Vec<Box<RefCell<dyn ForgedTrait>>>,
}

impl ForgedObject {
    pub fn new(name: String, id: u32) -> ForgedObject {
        ForgedObject {
            name,
            id,
            forged_traits: Vec::new(),
        }
    }

    pub fn add_trait(&mut self, trait_: Box<RefCell<dyn ForgedTrait>>) {
        self.forged_traits.push(trait_);
    }

    pub fn start(&self) {
        for trait_ in &self.forged_traits {
            trait_.borrow().start();
        }
    }

    pub fn update(&self) {
        for trait_ in &self.forged_traits {
            trait_.borrow().update();
        }
    }

    pub fn get_trait<T: 'static + ForgedTrait>(
        &self,
    ) -> Result<&T, LostLostLandsFaultForgedObject> {
        let forged_trait = self.forged_traits.iter().find(|forged_trait| {
            forged_trait.borrow().as_any().type_id() == std::any::TypeId::of::<T>()
        });

        if let Some(forged_trait) = forged_trait {
            let forged_trait = forged_trait.borrow();
            let forged_trait = forged_trait.as_any().downcast_ref::<T>().unwrap();
            let forged_trait = unsafe { std::mem::transmute::<&T, &'static T>(forged_trait) };
            return Ok(forged_trait);
        } else {
            Err(LostLostLandsFaultForgedObject::TraitNotFound(
                std::any::type_name::<T>().to_string(),
            ))
        }
    }

    pub fn get_trait_mut<T: 'static + ForgedTrait>(
        &self,
    ) -> Result<&mut T, LostLostLandsFaultForgedObject> {
        let forged_trait = self.forged_traits.iter().find(|forged_trait| {
            forged_trait.borrow().as_any().type_id() == std::any::TypeId::of::<T>()
        });

        if let Some(forged_trait) = forged_trait {
            let mut forged_trait = forged_trait.borrow_mut();
            let forged_trait = forged_trait.as_any_mut().downcast_mut::<T>().unwrap();
            let forged_trait =
                unsafe { std::mem::transmute::<&mut T, &'static mut T>(forged_trait) };
            return Ok(forged_trait);
        } else {
            Err(LostLostLandsFaultForgedObject::TraitNotFound(
                std::any::type_name::<T>().to_string(),
            ))
        }
    }

    pub fn remove_trait<T: 'static + ForgedTrait>(
        &mut self,
    ) -> Result<(), LostLostLandsFaultForgedObject> {
        let forged_trait = self.forged_traits.iter().position(|forged_trait| {
            println!("{:?}", forged_trait.borrow().as_any().type_id());
            forged_trait.borrow().as_any().type_id() == std::any::TypeId::of::<T>()
        });

        if let Some(forged_trait) = forged_trait {
            self.forged_traits.remove(forged_trait);
            Ok(())
        } else {
            Err(LostLostLandsFaultForgedObject::TraitNotFound(
                std::any::type_name::<T>().to_string(),
            ))
        }
    }
}
