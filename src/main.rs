use std::cell::RefCell;

use prime_forge::{
    destiny_rift::DestinyRift, forged_object::ForgedObject, forged_trait::ForgedTrait,
    lost_realm::LostRealm, EtherealFlow,
};

use crate::prime_forge::soul_thread::{EssenceAspect, SoulThread, TemporalPause};

pub mod prime_forge;

pub struct Health(i32);

impl ForgedTrait for Health {
    fn start(&self) {
        println!("Health started");
    }

    fn update(&self) {}
}

impl EtherealFlow for Health {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct Collision(bool);

impl DestinyRift for Collision {}
impl EtherealFlow for Collision {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    let mut lost_realm = LostRealm::new();
    let mut f_object = ForgedObject::new("Player".to_string(), 0);
    f_object.add_trait(Box::new(RefCell::new(Health(100))));
    let h = f_object.get_trait::<Health>().unwrap();
    println!("Health: {:?}", h.0);
    let h = f_object.get_trait_mut::<Health>().unwrap();
    println!("Health: {:?}", h.0);
    h.0 = 200;
    lost_realm.add_object(f_object);

    lost_realm.add_destiny_rift_event(Collision(true));
    let rs = lost_realm.consume_destiny_rift_event::<Collision>();
    if let Some(rs) = rs {
        println!("Collision: {:?}", rs.0);
    }
    let mut counter = 10;
    lost_realm.add_soul_thread(SoulThread::new("Soul", move || {
        println!("Soul thread");
        counter -= 1;
        if counter == 0 {
            println!("Soul thread finished");
            return EssenceAspect::Finished;
        }
        return EssenceAspect::Yielded(TemporalPause {
            amount_in_seconds: 1.0,
        });
    }));

    lost_realm.start();
    lost_realm.debug_update();
}
