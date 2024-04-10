use std::{cell::RefCell};

use prime_forge::{
    destiny_rift::DestinyRift, forged_object::ForgedObject, forged_trait::ForgedTrait,
    lost_realm::LostRealm, EtherealFlow,
};

pub mod prime_forge;

pub struct Health(i32);

impl ForgedTrait for Health {
    fn start(&self) {
        println!("Health started");
    }

    fn update(&self) {
        println!("Health updated");
    }
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

trait AmountToWait {
    fn wait(&self, amount: f32);
}

struct Wait {
    amount_in_seconds: f32,
}

impl Wait {
    fn new(amount_in_seconds: f32) -> Wait {
        Wait { amount_in_seconds }
    }
}

impl Iterator for Wait {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.amount_in_seconds > 0.0 {
            self.amount_in_seconds -= 1.0;
            Some(Wait::new(self.amount_in_seconds))
        } else {
            None
        }
    }
}

struct Coroutine {
    name: String,
    callback: Box<dyn Fn() -> Box<dyn Iterator<Item = Wait>>>,
}

impl Coroutine {
    fn new(name: String, callback: Box<dyn Fn() -> Box<dyn Iterator<Item = Wait>>>) -> Coroutine {
        Coroutine { name, callback }
    }

    fn update(&mut self) {
        let mut iter = (self.callback)();
        while let Some(wait) = iter.next() {
            println!("Coroutine: {:?}", wait.amount_in_seconds);
        }
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

    lost_realm
        .destiny_rift_manager
        .add_event(Box::new(Collision(true)));
    let rs = lost_realm.destiny_rift_manager.consume_event::<Collision>();
    if let Some(rs) = rs {
        println!("Collision: {:?}", rs.0);
    }

    lost_realm.start();
    lost_realm.update();

    fn test() ->  Box<dyn Iterator<Item = Wait>> {
        Box::new(Wait::new(6.0)) 
    }

    let mut coroutine = Coroutine::new("Coroutine".to_string(), Box::new(test));

    coroutine.update();
}
