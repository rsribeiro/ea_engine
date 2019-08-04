use crate::{
    enemy::{EnemyType, FireballShowerConfig},
    healing::HealingConfig,
};
use quicksilver::Result;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use specs::World;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum FactoryType {
    Fixed,
    Random,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct EntityFactoryConfig {
    pub factory_type: FactoryType,
    pub enemy_types: Vec<EnemyType>,
    pub healing_interval: Option<i32>,
}

impl Default for EntityFactoryConfig {
    fn default() -> EntityFactoryConfig {
        EntityFactoryConfig {
            factory_type: FactoryType::Fixed,
            enemy_types: vec![EnemyType::Walker, EnemyType::Shooter],
            healing_interval: Some(3),
        }
    }
}

pub struct EntityFactory {
    factory_type: FactoryType,
    enemy_types: Vec<EnemyType>,
    healing_interval: Option<i32>,
    counter: i32,
}

impl EntityFactory {
    pub fn new(config: EntityFactoryConfig) -> Result<Self> {
        Ok(EntityFactory {
            factory_type: config.factory_type,
            enemy_types: config.enemy_types,
            healing_interval: config.healing_interval,
            counter: 0,
        })
    }

    pub fn create_entity(&mut self, world: &mut World) -> Result<()> {
        let pos = match self.factory_type {
            FactoryType::Fixed => self.counter as usize % self.enemy_types.len(),
            FactoryType::Random => thread_rng().gen_range(0, self.enemy_types.len()),
        };
        match self.enemy_types[pos] {
            EnemyType::Walker => crate::enemy::create_walker(world),
            EnemyType::Shooter => crate::enemy::create_shooter(world),
            EnemyType::Flyer => crate::enemy::create_flyer(world),
            EnemyType::FireballShower => {
                crate::enemy::create_fireball_shower(world, FireballShowerConfig::default())
            }
        };
        if self.healing_interval.is_some() && self.counter % self.healing_interval.unwrap() == 0 {
            crate::healing::create_healing_potion(world, HealingConfig::default());
        }
        self.counter += 1;
        Ok(())
    }
}
