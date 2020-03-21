use crate::rendering::colored::ColoredVertex;
use crate::rendering::textured::TexturedVertex;

pub const SIZE: [f32; 2] = [0.5, 1.5];
pub const P00: [f32; 2] = [2.0, 4.0];
pub const P10: [f32; 2] = [2.5, 4.0];
pub const P01: [f32; 2] = [2.0, 5.5];
pub const P11: [f32; 2] = [2.5, 5.5];
pub const POS: [[f32; 2]; 4] = [P00, P10, P01, P11];

// char A has column 1 & row 11
pub const A00: [f32; 2] = [0.0625, 0.6875];
pub const A10: [f32; 2] = [0.125, 0.6875];
pub const A01: [f32; 2] = [0.0625, 0.75];
pub const A11: [f32; 2] = [0.125, 0.75];
pub const TC_A: [[f32; 2]; 4] = [A00, A10, A01, A11];

// char ? has column 15 & row 12
pub const Q00: [f32; 2] = [0.9375, 0.75];
pub const Q10: [f32; 2] = [1.0, 0.75];
pub const Q01: [f32; 2] = [0.9375, 0.8125];
pub const Q11: [f32; 2] = [1.0, 0.8125];
pub const TC_Q: [[f32; 2]; 4] = [Q00, Q10, Q01, Q11];

pub const COLOR: [f32; 3] = [0.4, 0.5, 0.6];

pub fn assert_tile(vertices: &[ColoredVertex], pos: [[f32; 2]; 4], color: [f32; 3]) {
    assert_eq!(vertices.len(), 6);
    assert_colored(&vertices[0], pos[0], color);
    assert_colored(&vertices[1], pos[1], color);
    assert_colored(&vertices[2], pos[3], color);
    assert_colored(&vertices[3], pos[0], color);
    assert_colored(&vertices[4], pos[3], color);
    assert_colored(&vertices[5], pos[2], color);
}

pub fn assert_textured_tile(
    vertices: &[TexturedVertex],
    pos: [[f32; 2]; 4],
    tcs: [[f32; 2]; 4],
    color: [f32; 3],
) {
    assert_eq!(vertices.len(), 6);
    assert_textured(&vertices[0], pos[0], tcs[0], color);
    assert_textured(&vertices[1], pos[1], tcs[1], color);
    assert_textured(&vertices[2], pos[3], tcs[3], color);
    assert_textured(&vertices[3], pos[0], tcs[0], color);
    assert_textured(&vertices[4], pos[3], tcs[3], color);
    assert_textured(&vertices[5], pos[2], tcs[2], color);
}

pub fn assert_colored(vertex: &ColoredVertex, pos: [f32; 2], color: [f32; 3]) {
    assert_eq!(vertex.position, pos);
    assert_eq!(vertex.color, color);
}

pub fn assert_textured(vertex: &TexturedVertex, pos: [f32; 2], tc: [f32; 2], color: [f32; 3]) {
    assert_eq!(vertex.position, pos);
    assert_eq!(vertex.tc, tc);
    assert_eq!(vertex.color, color);
}
