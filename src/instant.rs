use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant as StdInstant;

#[cfg(target_arch = "wasm32")]
use stdweb::{unstable::TryInto, Value};

#[derive(Debug, Clone)]
pub struct Instant {
    #[cfg(not(target_arch = "wasm32"))]
    instant: StdInstant,
    #[cfg(target_arch = "wasm32")]
    instant: Value,
}

impl Instant {
    pub fn now() -> Instant {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Instant {
                instant: StdInstant::now(),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let instant: Value = js! {
                return performance.now();
            };
            Instant { instant }
        }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.instant.duration_since(earlier.instant)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let elapsed: Value = js! {
                return parseInt(@{&self.instant} - @{&earlier.instant});//milliseconds
            };
            Duration::from_millis(elapsed.try_into().unwrap())
        }
    }
}
