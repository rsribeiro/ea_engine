use crate::scene::{Scene, SceneConfig};
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
    let settings = Settings {
        icon_path: Some("icone.png"),
        show_cursor: false,
        ..Settings::default()
    };
    quicksilver::lifecycle::run::<Game>("Evil Alligator", Vector::new(800, 600), settings);
}

fn create_scene(path: impl AsRef<Path>) -> impl Future<Item = Scene, Error = Error> {
    load_file(PathBuf::from(path.as_ref())).map(move |data| {
        let cfg: SceneConfig = serde_json::from_slice(data.as_slice()).unwrap();
        Scene::new(cfg).unwrap()
    })
}
