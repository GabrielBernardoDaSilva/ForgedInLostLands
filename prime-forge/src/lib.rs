pub mod forged_trait;
pub mod forged_object;
pub mod lost_lands_fault;
pub mod lost_realm;
pub mod eonforge;
pub mod destiny_rift;
pub mod soul_thread;
pub mod arcane_weft;

pub trait EtherealFlow {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}