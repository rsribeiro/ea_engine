use crate::component::{CalculateOutOfBounds, Healing, Position, Render, Velocity};
use quicksilver::geom::Vector;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use specs::{Builder, Entity, World};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct HealingConfig {
    pub sprite: String,
    pub position: Vector,
    pub velocity: Vector,
    pub score: i32,
}

impl Default for HealingConfig {
    fn default() -> HealingConfig {
        HealingConfig {
            sprite: "potion".to_string(),
            position: Vector::new(thread_rng().gen_range(50.0, 700.0), -100.0),
            velocity: Vector::new(0.0, 250.0),
            score: 50,
        }
    }
}

pub fn create_healing_potion(world: &mut World, config: HealingConfig) -> Entity {
    world
        .create_entity()
        .with(CalculateOutOfBounds)
        .with(Position {
            position: config.position,
        })
        .with(Velocity {
            velocity: config.velocity,
        })
        .with(Render {
            sprite: config.sprite,
            bounding_box: None,
        })
        .with(Healing {
            score: config.score,
        })
        .build()
}
