use crate::component::{
    Boss, CalculateOutOfBounds, ChangeSprite, Enemy, Fireball, Position, Render, Shooter, Velocity,
};
use serde::{Deserialize, Serialize};

use specs::{
    world::{Builder, Index},
    World,
};

use quicksilver::geom::{Rectangle, Shape, Vector};

use rand::{thread_rng, Rng};

const ENEMY_HEAD_HEIGHT: f32 = 10.;

pub fn get_enemy_head_body_area(self_area: Rectangle, position: Vector) -> (Rectangle, Rectangle) {
    let self_area = self_area.with_center(position);
    (
        Rectangle::new(
            self_area.top_left(),
            Vector::new(self_area.width(), ENEMY_HEAD_HEIGHT),
        ),
        Rectangle::new(
            self_area.top_left() + Vector::new(0., 10),
            Vector::new(self_area.width(), self_area.height() - ENEMY_HEAD_HEIGHT),
        ),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnemyConfig {
    pub sprite: String,
    pub position: Vector,
    pub velocity: Vector,
    pub score: i32,
    pub shooter_config: Option<ShooterConfig>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum EnemyType {
    Walker,
    Shooter,
    Flyer,
    FireballShower,
}

pub fn create_enemy(world: &mut World, config: EnemyConfig) {
    let mut builder = world
        .create_entity()
        .with(CalculateOutOfBounds)
        .with(Position {
            position: config.position,
        })
        .with(Velocity {
            velocity: config.velocity,
        })
        .with(Render {
            sprite: config.sprite.clone(),
            bounding_box: None,
        })
        .with(Enemy {
            score: config.score,
        });
    if let Some(shooter_config) = config.shooter_config {
        builder = builder.with(Shooter {
            projectile_sprite: shooter_config.projectile_sprite.clone(),
            maximum_fireballs: shooter_config.maximum_projectiles,
            fireball_amount: 0,
            coefficient: shooter_config.projectile_coefficient,
        });
    }
    builder.build();
}

pub fn create_walker(world: &mut World) {
    let config = if rand::random() {
        EnemyConfig {
            sprite: "andador".to_string(),
            position: Vector::new(850.0, 432.0),
            velocity: Vector::new(-125.0, 0.0),
            score: 100,
            shooter_config: None,
        }
    } else {
        EnemyConfig {
            sprite: "andador_flipped".to_string(),
            position: Vector::new(-50.0, 432.0),
            velocity: Vector::new(125.0, 0.0),
            score: 100,
            shooter_config: None,
        }
    };
    create_enemy(world, config);
}

pub fn create_shooter(world: &mut World) {
    let config = EnemyConfig {
        sprite: "atirador".to_string(),
        position: Vector::new(850.0, 433.5),
        velocity: Vector::new(-125.0, 0.0),
        score: 200,
        shooter_config: Some(ShooterConfig {
            projectile_sprite: "tiro".to_string(),
            maximum_projectiles: 2,
            projectile_coefficient: (0.175, 0.0),
        }),
    };
    create_enemy(world, config);
}

pub fn create_flyer(world: &mut World) {
    let config = EnemyConfig {
        sprite: "alma".to_string(),
        position: Vector::new(850.0, 400.0),
        velocity: Vector::new(-150.0, 0.0),
        score: 200,
        shooter_config: Some(ShooterConfig {
            projectile_sprite: "tiro".to_string(),
            maximum_projectiles: 1,
            projectile_coefficient: (0.250, 0.0),
        }),
    };
    create_enemy(world, config);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct BossConfig {
    pub sprite: String,
    pub angry_sprite: String,
    pub position: Vector,
    pub lives: i32,
    pub normal_lives: i32,
    pub shooter_config: ShooterConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShooterConfig {
    pub projectile_sprite: String,
    pub maximum_projectiles: i32,
    pub projectile_coefficient: (f32, f32),
}

impl Default for BossConfig {
    fn default() -> BossConfig {
        BossConfig {
            sprite: "chefe".to_string(),
            angry_sprite: "chefeapelao".to_string(),
            position: Vector::new(748.5, 428.0),
            lives: 10,
            normal_lives: 5,
            shooter_config: ShooterConfig {
                projectile_sprite: "tiro".to_string(),
                maximum_projectiles: 2,
                projectile_coefficient: (0.075, -0.05),
            },
        }
    }
}

pub fn create_boss(world: &mut World, config: BossConfig) {
    world
        .create_entity()
        .with(Boss {
            lives: config.lives,
            normal_lives: config.normal_lives,
        })
        .with(Position {
            position: config.position,
        })
        .with(Render {
            sprite: config.sprite.clone(),
            bounding_box: None,
        })
        .with(Enemy { score: 300 })
        .with(ChangeSprite {
            new_sprite: config.angry_sprite.clone(),
            do_change: false,
        })
        .with(Shooter {
            projectile_sprite: config.shooter_config.projectile_sprite.clone(),
            maximum_fireballs: config.shooter_config.maximum_projectiles,
            fireball_amount: 0,
            coefficient: config.shooter_config.projectile_coefficient,
        })
        .build();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FireballConfig {
    pub sprite: String,
    pub position: Vector,
    pub velocity: Vector,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct FireballShowerConfig {
    pub sprite: String,
    pub y_velocity: f32,
}

impl Default for FireballShowerConfig {
    fn default() -> FireballShowerConfig {
        FireballShowerConfig {
            sprite: "fogo".to_string(),
            y_velocity: -100.0,
        }
    }
}

pub fn create_fireball_shower(world: &mut World, config: FireballShowerConfig) {
    let mut rng = thread_rng();
    let x_init: i32 = rng.gen_range(0, 100);
    let x_end: i32 = rng.gen_range(810, 900);
    let step: usize = rng.gen_range(90, 120);
    for x in (x_init..x_end).step_by(step) {
        let fireball_config = FireballConfig {
            sprite: config.sprite.clone(),
            position: Vector::new(x as f32, config.y_velocity),
            velocity: Vector::new(0.0, 250.0 + rng.gen_range(-10.0, 10.0)),
        };
        create_fireball(world.create_entity(), None, fireball_config);
    }
}

pub fn create_fireball<T: Builder>(builder: T, owner_id: Option<Index>, config: FireballConfig) {
    builder
        .with(Fireball { owner_id })
        .with(CalculateOutOfBounds)
        .with(Render {
            sprite: config.sprite,
            bounding_box: None,
        })
        .with(Position {
            position: config.position,
        })
        .with(Velocity {
            velocity: config.velocity,
        })
        .build();
}
