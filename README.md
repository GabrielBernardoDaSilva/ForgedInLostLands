# Forged in Lost Lands
![alt text](https://github.com/GabrielBernardoDaSilva/forged-in-lost-lands/blob/master/forged-in-lost-lands.png)
Forged in Lost Lands is a simple game engine built in Rust that focuses on data control and ease of use. It provides a straightforward API for managing game objects, traits, events, and coroutines.

Features
Data Control: Manage game data with ease using Rust's strong typing and data ownership model.

Ethereal Flow: Utilize an Ethereal Flow system to manage game object hierarchies and behavior.
Event Handling: Handle game events through Destiny Rifts.
Thread Management: Manage game logic and coroutine tasks using Soul Threads.
ForgetTrait: Set behavior to your traits that will be execute in every update.
EonForge: Has every thing related to time control of tour application.
ArcaneWeft: Provide a way to separate your logic across multiple files like plugin system.
ForgedObject: The core of each entity that need to update their traits.

Simple Usage: Designed to be simple to use and integrate into your Rust game projects.

## Important

This engine does not have rendering by default,
You need to pick one you prefer and use with in!

Installation
To use Forged in Lost Lands in your Rust project, simply add the following line to your Cargo.toml file:

```toml
[dependencies]
bifrost-ecs = "0.1.0"

```

Example Usage
rust

<pre><code>
// Important imports
use prime_derived::{hierarchy_ethereal_flow, DestinyRiftArcaneScript, EtherealFlowArcaneScript};
use prime_forge::{
    arcane_weft::ArcaneWeft, forged_trait::ForgedTrait, lost_realm::LostRealm, soul_thread::{EssenceAspect, SoulThread, TemporalPause}
};


// Use of proc_macro for some internals and attribute macro for some hierarchical features. 
#[hierarchy_ethereal_flow]
#[derive(Default)]
struct Player {
    name: String,
    health: i32,
}

// Give behavior to your forged object 
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

// Define some custom events to your game or application
#[derive(DestinyRiftArcaneScript, EtherealFlowArcaneScript)]
pub struct Collision(bool);


pub struct ArcaneWeftCreation;
impl ArcaneWeft for ArcaneWeftCreation{
    fn craft(self, lost_realm: &mut LostRealm) {
        lost_realm.add_destiny_rift_event(Collision(true));
        lost_realm.forge_new_object("Player", (Player::default(), Health::default())).unwrap();
        println!("Arcane Weft Creation")
    }
}

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

    let h = f.get_trait::<Health>().unwrap();
    let father = lost_realm.get_mut_parent_forged_object(h).unwrap();
    println!("Father: {:?}", father.name);

    let p = lost_realm.get_mut_trait_by_type::<Player>().unwrap();
    println!("Player: {:?}", p.name);
    let father = lost_realm.get_mut_parent_forged_object(p).unwrap();
    println!("Father: {:?}", father.name);

    let all_forged_object_by_health_traits = lost_realm.get_mut_all_forged_objects_by_trait::<Health>();
    for fo in all_forged_object_by_health_traits {
        println!("Forged Object: {:?}", fo.name);
        fo.name += "!";
    }

    let all_health_traits = lost_realm.get_mut_all_trait_by_type::<Health>();
    for health in all_health_traits {
        println!("Health: {:?}", health.health);
        health.health += 100;
    }

    let all_forged_object_by_health_traits = lost_realm.get_all_forged_objects_by_trait::<Health>();
    for fo in all_forged_object_by_health_traits {
        println!("Forged Object: {:?}", fo.name);
    }

    let all_health_traits = lost_realm.get_all_trait_by_type::<Health>();
    for health in all_health_traits {
        println!("Health: {:?}", health.health);
    }

    

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

    lost_realm.arcane_weft_craft(ArcaneWeftCreation);
    let dt = lost_realm.get_delta_time();
    let time_since_start = lost_realm.get_time_elapsed();
    println!("Delta Time: {:?}", dt);
    println!("Time Since Start: {:?}", time_since_start);
    lost_realm.start();
    lost_realm.debug_update();
}

</code></pre>

License
Forged in Lost Lands is licensed under the MIT License. See LICENSE for details.
