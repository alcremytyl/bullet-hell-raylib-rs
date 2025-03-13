use macroquad::color::{Color, BLUE, RED};

pub enum Target {
    PLAYER,
    ENEMY,
}

impl Into<f32> for Target {
    fn into(self) -> f32 {
        match self {
            Self::PLAYER => (0 << 0) as u32 as f32,
            Self::ENEMY => (0 << 1) as u32 as f32,
        }
    }
}

impl From<f32> for Target {
    fn from(value: f32) -> Self {
        let a = match value {
            x if x == (0 << 0) as f32 => Self::PLAYER,
            x if x == (0 << 1) as f32 => Self::ENEMY,
            _ => Self::ENEMY,
        };
        a
    }
}

impl Target {
    pub fn as_color(self) -> Color {
        match self {
            Self::PLAYER => BLUE,
            Self::ENEMY => RED,
        }
    }
}

impl Into<Color> for f32 {
    fn into(self) -> Color {
        Target(self).as_color()
    }
}
