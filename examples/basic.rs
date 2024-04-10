use prime_derived::{hierarchy_ethereal_flow, DestinyRiftArcaneScript, EtherealFlowArcaneScript};
use prime_forge::{
    forged_trait::{ForgedHierarchy, ForgedTrait},
    lost_realm::LostRealm,
    soul_thread::{EssenceAspect, SoulThread, TemporalPause},
    EtherealFlow,
};

#[hierarchy_ethereal_flow]
#[derive(Default)]
struct Player {
    name: String,
    health: i32,
}

impl ForgedTrait for Player {
    fn start(&mut self) {
        println!("Player started");
    }

    fn update(&mut self) {}
}

struct Player1 {
    name: String,
    health: i32,
    father: Option<*const prime_forge::forged_object::ForgedObject>,
}

impl EtherealFlow for Player1 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl prime_forge::forged_trait::ForgedHierarchy for Player1 {
    fn get_father(&self) -> Option<&prime_forge::forged_object::ForgedObject> {
        unsafe { self.father.unwrap().as_ref() }
    }
    fn set_father(&mut self, father: &prime_forge::forged_object::ForgedObject) {
        self.father = Some(father as *const prime_forge::forged_object::ForgedObject);
    }
}

#[hierarchy_ethereal_flow]
#[derive(Default)]
pub struct Health {
    pub health: i32,
}

impl ForgedTrait for Health {
    fn start(&mut self) {
        println!("Health started");
    }

    fn update(&mut self) {}
}

#[derive(DestinyRiftArcaneScript, EtherealFlowArcaneScript)]
pub struct Collision(bool);

fn main() {
    let mut lost_realm = LostRealm::new();
    let health = Health {
        health: 100,
        ..Default::default()
    };
    let player = Player {
        name: "Player".to_string(),
        health: 100,
        ..Default::default()
    };

    let mut ptr: Option<*const prime_forge::forged_object::ForgedObject> = None;
    {
        let forged_object = lost_realm
            .forge_new_object("Forged", (player, health))
            .unwrap();
        ptr = Some(forged_object as *const _);
    }


    // println!("Player: {:?}", father.name);

    // let mut f_object = ForgedObject::new("Player".to_string());
    // let health = Health {
    //     health: 100,
    //     ..Default::default()
    // };

    // f_object.add_trait(Box::new(RefCell::new(health)));
    // let h = f_object.get_trait::<Health>().unwrap();
    // println!("Health: {:?}", h.0);
    // let h = f_object.get_trait_mut::<Health>().unwrap();
    // println!("Health: {:?}", h.0);
    // h.0 = 200;
    // lost_realm.add_object(f_object);

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
