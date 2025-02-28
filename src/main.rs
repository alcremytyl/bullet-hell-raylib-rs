use raylib::{
    color::Color,
    ffi::KeyboardKey,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use raylib_rs::{
    clamp, drawable::Drawable, player::Player, PLAYER_SIZE, PLAYER_SLOW, PLAYER_SPEED,
    PROJECTILE_CAP, SCREEN_H, SCREEN_W,
};

/*
TODO
- bullets (most likely multithread)
- enemies
- everything
*/

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W as i32, SCREEN_H as i32)
        .build();
    let mut player = Player::new();
    let bullet_pool = [(); PROJECTILE_CAP as usize];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY.alpha(0.1));

        player.draw(&mut d);

        handle_input(d, &mut player.pos);
    }
}

fn handle_input(handler: RaylibDrawHandle, player_pos: &mut Vector2) {
    let displace = Vector2 {
        x: (handler.is_key_down(KeyboardKey::KEY_A) as i32
            - handler.is_key_down(KeyboardKey::KEY_D) as i32) as f32,
        y: (handler.is_key_down(KeyboardKey::KEY_W) as i32
            - handler.is_key_down(KeyboardKey::KEY_S) as i32) as f32,
    } * PLAYER_SPEED
        * (1.0 - (PLAYER_SLOW * handler.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) as i32 as f32))
        * handler.get_frame_time();

    player_pos.x = clamp(player_pos.x - displace.x, 0.0, SCREEN_W);
    player_pos.y = clamp(player_pos.y - displace.y, 0.0, SCREEN_H);
}
