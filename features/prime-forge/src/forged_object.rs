use std::{cell::RefCell, rc::Rc};

use crate::{forged_trait::TransformSpecialTrait, lost_realm::LostRealm};

use super::{forged_trait::ForgedTrait, lost_lands_fault::LostLostLandsFaultForgedObject};

pub struct ForgedObject {
    pub name: String,
    pub id: uuid::Uuid,
    pub forged_traits: Vec<Box<RefCell<dyn ForgedTrait>>>,
    pub transform: Rc<RefCell<TransformSpecialTrait>>,
}

impl ForgedObject {
    pub fn new(name: String) -> ForgedObject {
        ForgedObject {
            name,
            id: uuid::Uuid::new_v4(),
            forged_traits: Vec::new(),
            transform: Rc::new(RefCell::new(TransformSpecialTrait::new())),
        }
    }

    pub fn add_trait(
        &mut self,
        new_trait: Box<RefCell<dyn ForgedTrait>>,
    ) -> Result<(), LostLostLandsFaultForgedObject> {
        // check if trait already exists
        let trait_exists = self.forged_traits.iter().any(|trait_| {
            trait_.borrow().as_any().type_id() == new_trait.borrow().as_any().type_id()
        });
        if trait_exists {
            return Err(LostLostLandsFaultForgedObject::TraitAlreadyExists(
                std::any::type_name::<dyn ForgedTrait>().to_string(),
            ));
        }

        new_trait.borrow_mut().set_father(self.id.to_string());
        self.forged_traits.push(new_trait);
        Ok(())
    }

    pub fn add_traits<T: TraitBundle>(&mut self, traits: T) {
        traits.craft_trait_bundle(self);
    }

    pub fn start(&self, lost_realm: &mut LostRealm) {
        for trait_ in &self.forged_traits {
            trait_.borrow_mut().start(lost_realm);
        }
    }

    pub fn update(&self, lost_realm: &mut LostRealm, dt: f32) {
        for trait_ in &self.forged_traits {
            trait_.borrow_mut().update(lost_realm, dt);
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
            Ok(unsafe { std::mem::transmute(forged_trait) })
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
            Ok(unsafe { std::mem::transmute(forged_trait) })
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

    // Transform Trait Special
    pub fn set_transform_parent(&self, parent: Rc<RefCell<TransformSpecialTrait>>) {
        TransformSpecialTrait::set_hierarchy( parent.clone(), self.transform.clone());
        parent.borrow_mut().update_self_and_children();
    }
}

pub trait TraitBundle {
    fn craft_trait_bundle(self, forged_object: &mut ForgedObject);
}

macro_rules! impl_trait_bundle {
    ($(($name: ident, $index: tt)),*) => {
        impl<$($name: ForgedTrait + 'static),*> TraitBundle for ($($name,)*) {
            fn craft_trait_bundle(self, forged_object: &mut ForgedObject) {
                $(forged_object.add_trait(Box::new(RefCell::new(self.$index))).unwrap();)*
            }
        }
    };
}
impl_trait_bundle!((A, 0));
impl_trait_bundle!((A, 0), (B, 1));
impl_trait_bundle!((A, 0), (B, 1), (C, 2));
impl_trait_bundle!((A, 0), (B, 1), (C, 2), (D, 3));
impl_trait_bundle!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
impl_trait_bundle!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
impl_trait_bundle!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
impl_trait_bundle!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7)
);
impl_trait_bundle!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8)
);
impl_trait_bundle!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8),
    (J, 9)
);
impl_trait_bundle!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8),
    (J, 9),
    (K, 10)
);
