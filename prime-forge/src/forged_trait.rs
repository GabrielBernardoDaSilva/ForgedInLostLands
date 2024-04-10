use crate::forged_object::ForgedObject;

use super::EtherealFlow;


pub trait ForgedTrait: ForgedHierarchy {
    fn start(&mut self) {}
    fn update(&mut self) {}
}

pub trait ForgedHierarchy: EtherealFlow {
    fn set_father(&mut self, father: &ForgedObject);
    fn get_father(&self) -> Option<&ForgedObject>;
}

