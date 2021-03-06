extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate image;

pub mod math;
pub mod rendering;

use crate::rendering::colored::ColoredVertex;
use crate::rendering::textured::TexturedVertex;

implement_vertex!(ColoredVertex, position, color);
implement_vertex!(TexturedVertex, position, tc, color);
