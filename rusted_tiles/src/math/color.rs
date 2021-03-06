#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
};
pub const CYAN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
};
pub const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
};
pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
};
pub const PINK: Color = Color {
    r: 1.0,
    g: 0.08,
    b: 0.58,
};
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
pub const YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
};
