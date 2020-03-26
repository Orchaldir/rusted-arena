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

pub fn get_position(body: &Body) -> usize {
    match body {
        Body::Simple(index) => *index,
        Body::Big(index, _) => *index,
        Body::Snake(indices) => indices[0],
    }
}

pub fn update_position(body: &Body, index: usize) -> Body {
    match body {
        Body::Simple(_) => Body::Simple(index),
        Body::Big(_, size) => Body::Big(index, *size),
        Body::Snake(ref indices) => {
            let mut new_indices = vec![index];
            new_indices.extend_from_slice(indices);
            new_indices.pop();
            Body::Snake(new_indices)
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
    use super::*;

    #[test]
    fn test_get_position_simple() {
        assert_eq!(get_position(&Body::Simple(3)), 3);
    }

    #[test]
    fn test_get_position_big() {
        assert_eq!(get_position(&Body::Big(4, 6)), 4);
    }

    #[test]
    fn test_get_position_snake() {
        assert_eq!(get_position(&Body::Snake(vec![1, 2, 3, 4])), 1);
    }

    #[test]
    fn test_update_position_simple() {
        assert_eq!(update_position(&Body::Simple(3), 4), Body::Simple(4));
    }

    #[test]
    fn test_update_position_big() {
        assert_eq!(update_position(&Body::Big(3, 4), 5), Body::Big(5, 4));
    }

    #[test]
    fn test_update_position_snake() {
        assert_eq!(
            update_position(&Body::Snake(vec![3, 4, 5]), 2),
            Body::Snake(vec![2, 3, 4])
        );
    }
}
