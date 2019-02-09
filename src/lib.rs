extern crate quicksilver;
extern crate rand;
extern crate specs;
#[macro_use]
extern crate specs_derive;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

pub mod component;
pub mod enemy;
pub mod healing;
pub mod hero;
pub mod instant;
pub mod music;
pub mod resources;
pub mod scene;
pub mod system;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
