use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::ComponentMap;
use rusted_tiles::math::color::*;
use rusted_tiles::math::get_point;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::tile::TileRenderer;

#[derive(PartialEq, Clone, Debug)]
pub enum Body {
    Simple(usize),
    Big(usize, u32),
    Snake(Vec<usize>),
}

impl Component for Body {
    type Storage = ComponentMap<Self>;

    fn get_component_type() -> &'static str {
        "Body"
    }
}

pub fn get_position(body: &Body) -> usize {
    match body {
        Body::Simple(index) => *index,
        Body::Big(index, _) => *index,
        Body::Snake(indices) => indices[0],
    }
}

pub fn update_position(body: &mut Body, new_index: usize) {
    match body {
        Body::Simple(index) => *index = new_index,
        Body::Big(index, _) => *index = new_index,
        Body::Snake(indices) => {
            let mut new_indices = vec![new_index];
            new_indices.extend_from_slice(indices);
            new_indices.pop();

            *indices = new_indices;
        }
    }
}

pub fn render_body(renderer: &mut TileRenderer, size: Point, body: &Body) {
    match body {
        Body::Simple(index) => renderer.add_ascii(get_point(*index, size), b'@', RED),
        Body::Big(index, s) => renderer.add_big_ascii(get_point(*index, size), *s, b'D', RED),
        Body::Snake(indices) => {
            for i in (0..indices.len()).rev() {
                let color = if i == 0 { RED } else { WHITE };
                renderer.add_ascii(get_point(indices[i], size), b'S', color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Body::*;
    use super::*;

    #[test]
    fn test_get_component_type() {
        assert_eq!(Body::get_component_type(), "Body");
    }

    #[test]
    fn test_get_position_simple() {
        assert_eq!(get_position(&Simple(3)), 3);
    }

    #[test]
    fn test_get_position_big() {
        assert_eq!(get_position(&Big(4, 6)), 4);
    }

    #[test]
    fn test_get_position_snake() {
        assert_eq!(get_position(&Snake(vec![1, 2, 3, 4])), 1);
    }

    #[test]
    fn test_update_position_simple() {
        let mut body = Simple(3);
        update_position(&mut body, 4);

        assert_eq!(body, Simple(4));
    }

    #[test]
    fn test_update_position_big() {
        let mut body = Big(3, 4);
        update_position(&mut body, 5);

        assert_eq!(body, Big(5, 4));
    }

    #[test]
    fn test_update_position_snake() {
        let mut body = Snake(vec![3, 4, 5]);
        update_position(&mut body, 2);

        assert_eq!(body, Snake(vec![2, 3, 4]));
    }
}
