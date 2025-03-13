use crate::game::Game;

pub struct Weapon {
    pub cooldown: Option<f32>,
    pub pattern: dyn FnMut,
}

impl Weapon {
    pub fn shoot(game: &mut Game) {
        todo!()
    }
}
