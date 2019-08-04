use quicksilver::{
    lifecycle::Asset,
    sound::{Sound, StopHandle},
    Result,
};

pub struct MusicPlayer {
    current_music: Option<Asset<Sound>>,
    stop_handle: Option<StopHandle>,
    volume: f32,
}

impl MusicPlayer {
    pub fn new() -> Result<Self> {
        Ok(MusicPlayer {
            current_music: None,
            stop_handle: None,
            volume: 0.75,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        if self.stop_handle.is_none() {
            if let Some(music) = &mut self.current_music {
                let mut handle: Option<StopHandle> = None;
                let vol = self.volume;
                music.execute(|music| {
                    music.set_volume(vol);
                    handle = Some(music.play()?);
                    Ok(())
                })?;
                self.stop_handle = handle;
            }
        }
        Ok(())
    }

    pub fn play_music(&mut self, music: String) -> Result<()> {
        self.stop_music()?;
        self.current_music = Some(Asset::new(Sound::load(music)));
        Ok(())
    }

    fn stop_music(&mut self) -> Result<()> {
        match self.stop_handle.take() {
            Some(x) => {
                self.current_music = None;
                x.stop()
            }
            None => Ok(()),
        }
    }
}
