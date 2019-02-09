use quicksilver::{
    lifecycle::Asset,
    sound::{Sound, StopHandle},
    Future, Result,
};

const NORMAL_MUSIC: &str = "music/normal.ogg";
const BOSS_MUSIC: &str = "music/boss.ogg";
const GAME_OVER_MUSIC: &str = "music/gameover.ogg";
const VICTORY_MUSIC: &str = "music/victory.ogg";

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

impl MusicPlayer {
    pub fn new() -> Result<MusicPlayer> {
        let assets = Asset::new(Sound::load(NORMAL_MUSIC).join4(
            Sound::load(BOSS_MUSIC),
            Sound::load(GAME_OVER_MUSIC),
            Sound::load(VICTORY_MUSIC),
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
