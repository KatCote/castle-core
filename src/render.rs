use crate::functions::*;

/// Enum for use in match in functions as param
pub enum RenderInterface {
    Default,
    MapFull,
    MapLayer(MapLayer),
    MiniMap,
    InvPage(InvPage),
    PlayerParams,
    InfoWorld,
    InfoItem
}

/// Map layers enum for RenderInterface
#[derive(Debug)]
pub enum MapLayer {
    Base,
    Color,
    Trigger,
    Wall,
    SumObj(Summon),
    Explore // bool (visible map for minimap)
}

/// Inventory Pages enum for RenderInterface
#[derive(Debug)]
pub enum InvPage {
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
                    match ml {
                        MapLayer::Base => printch(i, j, &'A'),
                        MapLayer::Color => printch(i, j, &'B'),
                        MapLayer::Trigger => printch(i, j, &'C'),
                        MapLayer::Wall => printch(i, j, &'D'),
                        MapLayer::SumObj(ref object) => {
                            match object {
                                Summon::Static => printch(i, j, &'!'),
                                Summon::Dynamic => printch(i, j, &'@'),
                                Summon::NPC => printch(i, j, &'#'),
                                Summon::Enemy => printch(i, j, &'$')
                            }   
                        }
                        MapLayer::Explore => printch(i, j, &'E')
                    };
                },
                RenderInterface::MiniMap => { let _ = printch(i, j, &'3'); },
                RenderInterface::InvPage(ref page) => {
                    match page {
                        InvPage::Page1 => printch(i, j, &'F'),
                        InvPage::Page2 => printch(i, j, &'H')
                    };
                },
                RenderInterface::PlayerParams => { let _ = printch(i, j, &'5'); },
                RenderInterface::InfoWorld => { let _ = printch(i, j, &'6'); },
                RenderInterface::InfoItem => { let _ = printch(i, j, &'7'); }
            };
        }
    }
}
