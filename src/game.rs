use crate::scene::{Scene, SceneConfig};
use log::Level;
use quicksilver::prelude::*;
use std::path::{Path, PathBuf};

struct Game {
    scene: Asset<Scene>,
}

impl State for Game {
    fn new() -> Result<Self> {
        Ok(Game {
            scene: Asset::new(create_scene("scene.json")),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.scene.execute(|s| s.update(window))
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.scene.execute(|s| s.draw(window))
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.scene.execute(|s| s.event(event, window))
    }
}

pub fn run() {
    init_logger(Level::Info);
    let settings = Settings {
        icon_path: Some("icone.png"),
        show_cursor: false,
        ..Settings::default()
    };
    quicksilver::lifecycle::run::<Game>("Evil Alligator", Vector::new(800, 600), settings);
}

fn init_logger(level: Level) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        simple_logger::init_with_level(level).unwrap();
    }
    //TODO make this work
    // #[cfg(target_arch = "wasm32")]
    // {
    //     // wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    //     // console_log::init_with_level(level).expect("error initializing log");
    // }
}

fn create_scene(path: impl AsRef<Path>) -> impl Future<Item = Scene, Error = Error> {
    load_file(PathBuf::from(path.as_ref())).map(move |data| {
        let cfg: SceneConfig = serde_json::from_slice(data.as_slice()).unwrap();
        Scene::new(cfg).unwrap()
    })
}
