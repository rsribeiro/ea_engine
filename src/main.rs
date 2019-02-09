use eangine::scene::Scene;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

fn main() {
    run::<Scene>(
        "Evil Alligator",
        Vector::new(800, 600),
        Settings {
            icon_path: Some("icone.png"),
            show_cursor: false,
            ..Settings::default()
        },
    );
}
