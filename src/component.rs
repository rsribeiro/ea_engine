use std::time::Duration;

use crate::resources::LabelVariable;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::FontStyle,
};

use specs::{world::Index, Component, NullStorage, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub position: Vector,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub velocity: Vector,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Render {
    pub sprite: String,
    pub bounding_box: Option<Rectangle>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Shooter {
    pub projectile_sprite: String,
    pub maximum_fireballs: i32,
    pub fireball_amount: i32,
    pub coefficient_1: f32,
    pub coefficient_2: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Label {
    pub bind_variable: LabelVariable,
    pub font_style: FontStyle,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Hero {
    pub lives: i32,
    pub score: i32,
    pub blinking: bool,
    pub render: bool,
    pub reset_position: bool,
    pub blink_timer: Duration,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Boss {
    pub lives: i32,
    pub normal_lives: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ChangeSprite {
    pub new_sprite: String,
    pub do_change: bool,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Enemy {
    pub score: i32,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Healing {
    pub score: i32,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Background;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct CalculateOutOfBounds;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Fireball {
    pub owner_id: Option<Index>,
}
