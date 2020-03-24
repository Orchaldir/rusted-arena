use rusted_tiles::math::color::*;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::tile::TileRenderer;

#[derive(PartialEq, Clone, Debug)]
pub enum Body {
    Simple(Point),
    Big(Point, u32),
    Snake(Vec<Point>),
}

pub fn get_position(body: &Body) -> Point {
    match body {
        Body::Simple(pos) => *pos,
        Body::Big(pos, _) => *pos,
        Body::Snake(ref parts) => parts[0],
    }
}

pub fn render_body(renderer: &mut TileRenderer, body: &Body) {
    match body {
        Body::Simple(pos) => renderer.add_ascii(*pos, b'@', RED),
        Body::Big(pos, size) => renderer.add_big_ascii(*pos, *size, b'D', RED),
        Body::Snake(ref parts) => {
            for i in (0..parts.len()).rev() {
                let color = if i == 0 { RED } else { WHITE };
                renderer.add_ascii(parts[i], b'S', color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POS: Point = Point { x: 1, y: 2 };

    #[test]
    fn test_get_position_simple() {
        assert_eq!(get_position(&Body::Simple(POS)), POS);
    }

    #[test]
    fn test_get_position_big() {
        assert_eq!(get_position(&Body::Big(POS, 6)), POS);
    }

    #[test]
    fn test_get_position_snake() {
        let points = vec![0, 1, 2, 3]
            .into_iter()
            .map(|x| Point {
                x: POS.x + x,
                y: POS.y,
            })
            .collect();

        assert_eq!(get_position(&Body::Snake(points)), POS);
    }
}
