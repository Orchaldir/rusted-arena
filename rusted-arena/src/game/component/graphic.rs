use crate::utils::ecs::component::Component;
use crate::utils::ecs::storage::ComponentMap;
use rusted_tiles::math::color::*;

#[derive(PartialEq, Clone, Debug)]
pub enum GraphicData {
    Ascii(u8, Color),
}

const UNKNOWN_GRAPHIC_DATA: GraphicData = GraphicData::Ascii(b'?', PINK);

#[derive(PartialEq, Clone, Debug)]
pub struct Graphic {
    data: Vec<GraphicData>,
}

impl Component for Graphic {
    type Storage = ComponentMap<Self>;

    fn get_component_type() -> &'static str {
        "Graphic"
    }
}

impl Graphic {
    pub fn ascii(ascii: u8, color: Color) -> Graphic {
        Graphic {
            data: vec![GraphicData::Ascii(ascii, color)],
        }
    }

    pub fn two_ascii(ascii0: u8, color0: Color, ascii1: u8, color1: Color) -> Graphic {
        Graphic {
            data: vec![
                GraphicData::Ascii(ascii0, color0),
                GraphicData::Ascii(ascii1, color1),
            ],
        }
    }

    pub fn get(&self, index: usize) -> &GraphicData {
        self.data.get(index).unwrap_or(&UNKNOWN_GRAPHIC_DATA)
    }
}
