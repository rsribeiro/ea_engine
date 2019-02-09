use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    component::{
        Background, Boss, CalculateOutOfBounds, ChangeSprite, Enemy, Fireball, Healing, Hero,
        Label, Position, Render, Shooter, Velocity,
    },
    enemy::BossConfig,
    healing::HealingConfig,
    hero::HeroConfig,
    instant::Instant,
    music::{Music, MusicPlayer},
    resources::{
        DeltaTime, GameStateFlag, GameStateFlagRes, KeyboardKeys, LabelVariable, PressedKeys,
        VariableDictionary,
    },
    system::{
        CollisionSystem, FireballSystem, HeroBlinkingSystem, HeroControlSystem, LabelRenderSystem,
        OutOfBoundsSystem, RenderSystem, WalkSystem,
    },
};

use quicksilver::{
    geom::Vector,
    graphics::{Atlas, Color, Font, FontStyle},
    input::{ButtonState, Key},
    lifecycle::{Asset, Event, State, Window},
    Result,
};

use specs::{BitSet, Builder, Entity, RunNow, World};

#[derive(PartialEq)]
enum GameState {
    Initialiazing,
    Running,
    GameOver,
}

#[derive(Debug, Clone)]
pub struct SceneConfig {
    pub atlas: String,
    pub font: String,
    pub main_background: String,
    pub defeat_background: String,
    pub victory_background: String,
    pub hero_config: HeroConfig,
    pub boss_config: BossConfig,
    pub boss_cycle: u32,
    pub new_body_cycle: u64,
}

impl Default for SceneConfig {
    fn default() -> SceneConfig {
        SceneConfig {
            atlas: "evil_alligator.atlas".to_string(),
            font: "cmunrm.ttf".to_string(),
            main_background: "cenario".to_string(),
            defeat_background: "inferno".to_string(),
            victory_background: "ceu".to_string(),
            hero_config: HeroConfig::default(),
            boss_config: BossConfig::default(),
            boss_cycle: 11,
            new_body_cycle: 210,
        }
    }
}

pub struct Scene {
    world: World,
    atlas: Rc<RefCell<Asset<Atlas>>>,
    font: Rc<RefCell<Asset<Font>>>,
    hero: Entity,
    state: GameState,
    cycle_timer: u64,
    cycle_counter: u32,
    music_player: MusicPlayer,
    last_instant: Instant,
    entity_factory: Box<Fn(&mut World, u32) -> Result<()>>,
    config: SceneConfig,
}

impl State for Scene {
    fn new() -> Result<Scene> {
        let config = SceneConfig::default();

        let entity_factory = |world: &mut World, cycle_counter: u32| {
            let healing_config = HealingConfig {
                sprite: "potion".to_string(),
                position: Vector::new(50.0 + rand::random::<f32>() * 700.0, -100.0),
                velocity: Vector::new(0.0, 250.0),
                score: 50,
            };

            if cycle_counter % 2 == 1 {
                crate::enemy::create_walker(world);
            } else {
                crate::enemy::create_shooter(world);
            }
            if cycle_counter % 3 == 0 {
                crate::healing::create_healing_potion(world, healing_config);
            }
            Ok(())
        };
        Scene::new(Box::new(entity_factory), config)
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if self.state == GameState::Running {
            self.update_time_step()?;
            self.entity_factory()?;

            HeroControlSystem.run_now(&self.world.res);
            WalkSystem.run_now(&self.world.res);
            FireballSystem.run_now(&self.world.res);
            CollisionSystem.run_now(&self.world.res);
            OutOfBoundsSystem.run_now(&self.world.res);
            HeroBlinkingSystem.run_now(&self.world.res);

            let flag = self.world.read_resource::<GameStateFlagRes>().flag;
            if let Some(f) = flag {
                match f {
                    GameStateFlag::Victory => self.victory(),
                    GameStateFlag::Defeat => self.defeat(),
                }?;
            }
        }
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let mut running = self.state == GameState::Running;
        if !running {
            self.atlas.borrow_mut().execute(|_| {
                running = true;
                Ok(())
            })?;

            if self.state == GameState::Initialiazing && running {
                self.state = GameState::Running;
            } else if self.state == GameState::Initialiazing && !running {
                return Ok(());
            }
        }

        RenderSystem::new(window, Rc::clone(&self.atlas))?.run_now(&self.world.res);
        if self.state == GameState::Running {
            self.update_labels(window)?;
            LabelRenderSystem::new(window, Rc::clone(&self.font))?.run_now(&self.world.res);
        }
        self.world.maintain();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match self.state {
            GameState::Running => {
                let mut pressed_keys = self.world.write_resource::<PressedKeys>();
                let pressed_keys = &mut pressed_keys.pressed_keys;
                match event {
                    Event::Key(Key::Up, ButtonState::Pressed) => {
                        pressed_keys.add(KeyboardKeys::KeyUp as u32);
                    }
                    Event::Key(Key::Up, ButtonState::Released) => {
                        pressed_keys.remove(KeyboardKeys::KeyUp as u32);
                    }
                    Event::Key(Key::Left, ButtonState::Pressed) => {
                        pressed_keys.add(KeyboardKeys::KeyLeft as u32);
                    }
                    Event::Key(Key::Left, ButtonState::Released) => {
                        pressed_keys.remove(KeyboardKeys::KeyLeft as u32);
                    }
                    Event::Key(Key::Right, ButtonState::Pressed) => {
                        pressed_keys.add(KeyboardKeys::KeyRight as u32);
                    }
                    Event::Key(Key::Right, ButtonState::Released) => {
                        pressed_keys.remove(KeyboardKeys::KeyRight as u32);
                    }
                    _ => {}
                };

                if let Event::Key(Key::Escape, ButtonState::Pressed) = event {
                    let mut flag = self.world.write_resource::<GameStateFlagRes>();
                    *flag = GameStateFlagRes {
                        flag: Some(GameStateFlag::Defeat),
                    };
                }
            }
            GameState::GameOver => {
                if let Event::Key(Key::Escape, ButtonState::Pressed) = event {
                    window.close();
                }
                if let Event::Key(Key::Return, ButtonState::Pressed) = event {
                    window.close();
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Scene {
    pub fn new(
        entity_factory: Box<Fn(&mut World, u32) -> Result<()>>,
        config: SceneConfig,
    ) -> Result<Scene> {
        let atlas = Rc::new(RefCell::new(Asset::new(Atlas::load(config.atlas.clone()))));
        let font = Rc::new(RefCell::new(Asset::new(Font::load(config.font.clone()))));
        let music_player = MusicPlayer::new()?;

        let mut world = World::new();
        register_components(&mut world);
        add_resorces(&mut world);

        create_background(&mut world, config.main_background.clone());
        create_label(
            &mut world,
            LabelVariable::FramesPerSecond,
            FontStyle::new(48.0, Color::BLACK),
            Vector::new(20, 587),
        );
        create_label(
            &mut world,
            LabelVariable::HeroLives,
            FontStyle::new(48.0, Color::BLACK),
            Vector::new(10, 20),
        );
        create_label(
            &mut world,
            LabelVariable::Score,
            FontStyle::new(48.0, Color::BLACK),
            Vector::new(730, 20),
        );
        let hero = crate::hero::create_hero(&mut world, config.hero_config.clone());

        Ok(Scene {
            world,
            atlas,
            font,
            hero,
            state: GameState::Initialiazing,
            cycle_timer: 0,
            cycle_counter: 0,
            music_player,
            last_instant: Instant::now(),
            entity_factory,
            config,
        })
    }

    fn entity_factory(&mut self) -> Result<()> {
        if self.cycle_counter < self.config.boss_cycle {
            if self.cycle_timer == 0 {
                self.music_player.play_music(Music::NormalMusic)?;
            }
            self.cycle_timer += 1;
            if self.cycle_timer % self.config.new_body_cycle == 0 {
                self.cycle_counter += 1;
                if self.cycle_counter == self.config.boss_cycle {
                    self.music_player.play_music(Music::BossMusic)?;
                    crate::enemy::create_boss(&mut self.world, self.config.boss_config.clone());
                } else {
                    (self.entity_factory)(&mut self.world, self.cycle_counter)?;
                }
            }
        }
        Ok(())
    }

    fn update_time_step(&mut self) -> Result<()> {
        let now = Instant::now();
        let time_step = now.duration_since(self.last_instant.clone());
        self.last_instant = now;
        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime {
                duration: time_step,
            };
        }
        Ok(())
    }

    fn defeat(&mut self) -> Result<()> {
        self.end_game()?;
        create_background(&mut self.world, self.config.defeat_background.clone());
        self.music_player.play_music(Music::GameOverMusic)?;
        Ok(())
    }

    fn victory(&mut self) -> Result<()> {
        self.end_game()?;
        create_background(&mut self.world, self.config.victory_background.clone());
        self.music_player.play_music(Music::VictoryMusic)?;
        Ok(())
    }

    fn end_game(&mut self) -> Result<()> {
        self.world.delete_all();
        self.state = GameState::GameOver;
        Ok(())
    }

    fn update_labels(&mut self, window: &Window) -> Result<()> {
        let hero_storage = self.world.read_storage::<Hero>();
        if let Some(hero) = hero_storage.get(self.hero) {
            let mut dict = self.world.write_resource::<VariableDictionary>();
            *dict = VariableDictionary {
                dictionary: [
                    (
                        LabelVariable::FramesPerSecond,
                        format!("{:.0}", window.average_fps()),
                    ),
                    (LabelVariable::HeroLives, format!("{}", hero.lives)),
                    (LabelVariable::Score, format!("{}", hero.score)),
                ]
                .iter()
                .cloned()
                .collect(),
            }
        }
        Ok(())
    }
}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Render>();
    world.register::<Shooter>();
    world.register::<Label>();
    world.register::<Hero>();
    world.register::<Boss>();
    world.register::<ChangeSprite>();
    world.register::<Enemy>();
    world.register::<Healing>();
    world.register::<Background>();
    world.register::<CalculateOutOfBounds>();
    world.register::<Fireball>();
}

fn add_resorces(world: &mut World) {
    world.add_resource(GameStateFlagRes { flag: None });
    world.add_resource(DeltaTime {
        duration: Duration::new(0, 0),
    });
    world.add_resource(VariableDictionary {
        dictionary: [
            (LabelVariable::FramesPerSecond, "60".to_string()),
            (LabelVariable::HeroLives, "5".to_string()),
            (LabelVariable::Score, "0".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
    });
    world.add_resource(PressedKeys {
        pressed_keys: BitSet::new(),
    });
}

fn create_background(world: &mut World, sprite: String) -> Entity {
    world
        .create_entity()
        .with(Background)
        .with(Position {
            position: Vector::new(400, 300),
        })
        .with(Render {
            sprite,
            bounding_box: None,
        })
        .build()
}

fn create_label(
    world: &mut World,
    variable: LabelVariable,
    font_style: FontStyle,
    position: Vector,
) -> Entity {
    world
        .create_entity()
        .with(Label {
            bind_variable: variable,
            font_style,
        })
        .with(Position { position })
        .build()
}
