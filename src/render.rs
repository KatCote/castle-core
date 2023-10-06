use crate::functions::*;

/// Enum for use in match in functions as param
pub enum RenderInterface {
    DEFAULT,
    MapFull,
    MapLayer(Map),
    InvPage(Inventory),
    PlayerParams,
    InfoWorld,
    InfoItem
}

/// Map layers enum for RenderInterface
#[derive(Debug)]
pub enum Map {
    Base,
    Color,
    Trigger,
    Wall
}

/// Inventory Pages enum for RenderInterface
#[derive(Debug)]
pub enum Inventory {
    Page1,
    Page2
}

/// Render selected layer on current ractangle
pub fn render_layer(x1: u16, y1: u16, x2: u16, y2: u16, layer: RenderInterface) {

    for i in x1..x2 {
        for j in y1..y2 {
            match layer {
                RenderInterface::DEFAULT => { let _ = printch(i, j, &'.'); },
                RenderInterface::MapFull => { let _ = printch(i, j, &'0'); },
                RenderInterface::MapLayer(ref ml) => {  
                    let _ = match ml {
                        Map::Base => printch(i, j, &'A'),
                        Map::Color => printch(i, j, &'B'),
                        Map::Trigger => printch(i, j, &'C'),
                        Map::Wall => printch(i, j, &'D')
                    };
                },
                RenderInterface::InvPage(ref page) => {
                    let _ =match page {
                        Inventory::Page1 => printch(i, j, &'E'),
                        Inventory::Page2 => printch(i, j, &'F')
                    };
                },
                RenderInterface::PlayerParams => { let _ = printch(i, j, &'7'); },
                RenderInterface::InfoWorld => { let _ = printch(i, j, &'8'); },
                RenderInterface::InfoItem => { let _ = printch(i, j, &'9'); }
            };
        }
    }
}
