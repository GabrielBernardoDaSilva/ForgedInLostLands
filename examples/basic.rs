use prime_derived::{hierarchy_ethereal_flow, DestinyRiftArcaneScript, EtherealFlowArcaneScript};
use prime_forge::{
    forged_trait::ForgedTrait,
    lost_realm::LostRealm,
    soul_thread::{EssenceAspect, SoulThread, TemporalPause},
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
    use nalgebra_glm as glm;
    let lost_realm = LostRealm::new();
    let health = Health {
        health: 100,
        ..Default::default()
    };
    let player = Player {
        name: "Player".to_string(),
        health: 100,
        ..Default::default()
    };

    let f2 = lost_realm.forge_new_object("Forged1", (health,)).unwrap();
    let health = Health {
        health: 100,
        ..Default::default()
    };
    let f = lost_realm
        .forge_new_object("Forged", (player, health))
        .unwrap();
    f.transform.borrow_mut().position += glm::vec3(1.0, 0.0, 0.0);
    f2.transform.borrow_mut().position += glm::vec3(1.0, 0.0, 0.0);

    f.set_transform_child(f2.transform.clone());
    f.transform.borrow_mut().update_self_and_children();

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
