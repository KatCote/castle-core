use crate::functions::*;

/// Enum for use in match in functions as param
pub enum RenderInterface {
    Default,
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
    Wall,
    SumObj(Summon)
}

/// Inventory Pages enum for RenderInterface
#[derive(Debug)]
pub enum Inventory {
    Page1, // Like cells
    Page2 // Like shells
}

/// Summon variations for RenderInterface (not logic or triggers)
#[derive(Debug)]
pub enum Summon {
    Static, // Like chest (only render, not tringger or logic) // [][] of struct (x, y, type ... etc.)
    Dynamic, //Like fireplace // [][] of struct (x, y, type ... etc.)
    NPC, // Like trader in village // [][] of struct (x, y, type, hp, color ... etc.)
    Enemy // Like golem // [][] of Enemy struct (x, y, hp, color ... etc.)
}

/// Render selected layer on current ractangle
pub fn render_layer(x1: u16, y1: u16, x2: u16, y2: u16, layer: RenderInterface) {

    for i in x1..x2 {
        for j in y1..y2 {
            match layer {
                RenderInterface::Default => { let _ = printch(i, j, &'.'); },
                RenderInterface::MapFull => { let _ = printch(i, j, &'1'); },
                RenderInterface::MapLayer(ref ml) => {  
                    let _ = match ml {
                        Map::Base => printch(i, j, &'A'),
                        Map::Color => printch(i, j, &'B'),
                        Map::Trigger => printch(i, j, &'C'),
                        Map::Wall => printch(i, j, &'D'),
                        Map::SumObj(ref object) => {
                            match object {
                                Summon::Static => printch(i, j, &'!'),
                                Summon::Dynamic => printch(i, j, &'@'),
                                Summon::NPC => printch(i, j, &'#'),
                                Summon::Enemy => printch(i, j, &'$')
                            }   
                        }
                    };
                },
                RenderInterface::InvPage(ref page) => {
                    let _ = match page {
                        Inventory::Page1 => printch(i, j, &'E'),
                        Inventory::Page2 => printch(i, j, &'F')
                    };
                },
                RenderInterface::PlayerParams => { let _ = printch(i, j, &'4'); },
                RenderInterface::InfoWorld => { let _ = printch(i, j, &'5'); },
                RenderInterface::InfoItem => { let _ = printch(i, j, &'6'); }
            };
        }
    }
}
