use quicksilver::{
    lifecycle::Asset,
    sound::{Sound, StopHandle},
    Future, Result,
};

pub enum Music {
    NormalMusic,
    BossMusic,
    GameOverMusic,
    VictoryMusic,
}

pub struct MusicPlayer {
    assets: Asset<(Sound, Sound, Sound, Sound)>,
    stop_handle: Option<StopHandle>,
}

#[derive(Debug, Clone)]
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
    pub fn new(config: MusicPlayerConfig) -> Result<MusicPlayer> {
        let assets = Asset::new(Sound::load(config.normal_music).join4(
            Sound::load(config.boss_music),
            Sound::load(config.game_over_music),
            Sound::load(config.victory_music),
        ));

        Ok(MusicPlayer {
            assets,
            stop_handle: None,
        })
    }

    pub fn play_music(&mut self, music: Music) -> Result<()> {
        self.stop_music()?;

        let mut handle: Option<StopHandle> = None;
        self.assets.execute(|music_list| {
            handle = match music {
                Music::NormalMusic => Some(music_list.0.play()?),
                Music::BossMusic => Some(music_list.1.play()?),
                Music::GameOverMusic => Some(music_list.2.play()?),
                Music::VictoryMusic => Some(music_list.3.play()?),
            };
            Ok(())
        })?;
        self.stop_handle = handle;
        Ok(())
    }

    fn stop_music(&mut self) -> Result<()> {
        let opt = self.stop_handle.take();
        match opt {
            Some(x) => x.stop(),
            None => Ok(()),
        }
    }
}
