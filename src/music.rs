use quicksilver::{
    lifecycle::Asset,
    sound::{Sound, StopHandle},
    Future, Result,
};
use serde::{Deserialize, Serialize};

pub enum Music {
    NormalMusic,
    BossMusic,
    GameOverMusic,
    VictoryMusic,
}

pub struct MusicPlayer {
    assets: Asset<(Sound, Sound, Sound, Sound)>,
    current_music: Option<Music>,
    stop_handle: Option<StopHandle>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct MusicPlayerConfig {
    pub normal_music: String,
    pub boss_music: String,
    pub game_over_music: String,
    pub victory_music: String,
}

impl Default for MusicPlayerConfig {
    fn default() -> MusicPlayerConfig {
        MusicPlayerConfig {
            normal_music: "music/normal.ogg".to_string(),
            boss_music: "music/boss.ogg".to_string(),
            game_over_music: "music/gameover.ogg".to_string(),
            victory_music: "music/victory.ogg".to_string(),
        }
    }
}

impl MusicPlayer {
    pub fn new(config: MusicPlayerConfig) -> Result<Self> {
        let assets = Asset::new(Sound::load(config.normal_music).join4(
            Sound::load(config.boss_music),
            Sound::load(config.game_over_music),
            Sound::load(config.victory_music),
        ));

        Ok(MusicPlayer {
            assets,
            current_music: None,
            stop_handle: None,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        if self.stop_handle.is_none() {
            if let Some(music) = &self.current_music {
                let mut handle: Option<StopHandle> = None;
                self.assets.execute(|music_list| {
                    handle = match music {
                        Music::NormalMusic => {
                            music_list.0.set_volume(0.75);
                            Some(music_list.0.play()?)
                        }
                        Music::BossMusic => {
                            music_list.1.set_volume(0.75);
                            Some(music_list.1.play()?)
                        }
                        Music::GameOverMusic => {
                            music_list.2.set_volume(0.75);
                            Some(music_list.2.play()?)
                        }
                        Music::VictoryMusic => {
                            music_list.3.set_volume(0.75);
                            Some(music_list.3.play()?)
                        }
                    };
                    Ok(())
                })?;
                self.stop_handle = handle;
            }
        }
        Ok(())
    }

    pub fn play_music(&mut self, music: Music) -> Result<()> {
        self.stop_music()?;
        self.current_music = Some(music);
        Ok(())
    }

    fn stop_music(&mut self) -> Result<()> {
        match self.stop_handle.take() {
            Some(x) => {
                self.current_music = None;
                x.stop()
            },
            None => Ok(()),
        }
    }
}
