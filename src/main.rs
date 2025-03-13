/** TODO **
*
* mass rewrite to counter the len crash & organize better
* manually make weapons via closures
* find some profiler
*
*/
use macroquad::prelude::*;
use raylib_rs::game::Game;

fn conf() -> Conf {
    Conf {
        window_title: "bullet hell".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(WHITE);
        next_frame().await;
    }
}
