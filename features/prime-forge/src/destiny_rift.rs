use std::collections::HashMap;

use super::EtherealFlow;

pub trait DestinyRift: EtherealFlow {}

// alias for events
pub struct DestinyRiftManager {
    pub events: HashMap<std::any::TypeId, Vec<Box<dyn DestinyRift>>>,
}

impl DestinyRiftManager {
    pub fn new() -> DestinyRiftManager {
        DestinyRiftManager {
            events: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, event: Box<dyn DestinyRift>) {
        if let Some(ev) = self.events.get_mut(&event.as_any().type_id()) {
            ev.push(event);
            return;
        }
        self.events.insert(event.as_any().type_id(), vec![event]);
    }

    pub fn remove_event(&mut self) {
        let keys_to_remove = self
            .events
            .iter()
            .filter(|(_, ev)| ev.is_empty())
            .map(|(key, _)| *key)
            .collect::<Vec<_>>();

        for key in keys_to_remove {
            println!("Removing event: {:?}", key);
            self.events.remove(&key);
        }
    }

    pub fn consume_event<T: 'static + DestinyRift>(&mut self) -> Option<&T> {
        // remove last ev
        if let Some(ev) = self.events.get_mut(&std::any::TypeId::of::<T>()) {
            let ev = ev.pop().unwrap();
            let ev = ev.as_any().downcast_ref::<T>().unwrap();
            return Some(unsafe { std::mem::transmute::<&T, &'static T>(ev) });
        }
        None
    }
}

impl Default for DestinyRiftManager {
    fn default() -> Self {
        DestinyRiftManager::new()
    }
}
