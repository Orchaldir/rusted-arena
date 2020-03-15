#[macro_use]
extern crate glium;

pub mod rendering;

use crate::rendering::colored::ColoredVertex;

implement_vertex!(ColoredVertex, position, color);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
