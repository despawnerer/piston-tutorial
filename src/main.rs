extern crate piston_window;

use piston_window::*;


fn main() {
    let window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
    }
}
