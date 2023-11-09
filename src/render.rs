use crate::core::load_map;
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
                        MapLayer::Base => { render_map_layer(x1, y1, x2, y2, MapLayer::Base, 0, 0); break; },
                        MapLayer::Color => { render_map_layer(x1, y1, x2, y2, MapLayer::Color, 2, 5); break; },
                        MapLayer::Trigger => { render_map_layer(x1, y1, x2, y2, MapLayer::Trigger, 0, 0); break; },
                        MapLayer::Wall => { render_map_layer(x1, y1, x2, y2, MapLayer::Wall, 0, 0); break; },
                        MapLayer::SumObj(ref object) => {
                            match object {
                                Summon::Static => printch(i, j, &'!'),
                                Summon::Dynamic => printch(i, j, &'@'),
                                Summon::NPC => printch(i, j, &'#'),
                                Summon::Enemy => printch(i, j, &'$')
                            }   
                        }
                        MapLayer::Explore => { render_map_layer(x1, y1, x2, y2, MapLayer::Explore, 0, 0); break; }
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

/// Render Map Layer (only map layer) use load_map (will be rewrite, after this use RML in loop)
pub fn render_map_layer(x1: u16, y1: u16, x2: u16, y2: u16, map: MapLayer, x_offset: u16, y_offset: u16) {

    let loaded_map_pack = load_map(map);   // Need to move to loop (for 1 (or same) call load_map function)
                                                        //       |
    let loaded_map: Vec<String> = loaded_map_pack;      // ------+

    {// Print layer
        let x1 = x1 + x_offset;
        let x2 = x2 + x_offset;
        let y1 = y1;
        let y2 = y2;

        let mut row_counter = 0;
        let mut y_offset_counter = 0;

    for map_row in &loaded_map {

        if y_offset_counter >= y_offset && row_counter <= y2-y1-1 {
            
        let end_x: u16 = if x2-x1+x_offset < map_row.len() as u16 {x2-x1+x_offset} else {map_row.len() as u16};
        let end_y: u16 = if y1 + row_counter <= y2 {y1+row_counter} else {y2};
        printmsg(x1-x_offset, end_y, &map_row[x_offset as usize..end_x as usize]);

        row_counter = row_counter + 1;

        } else { y_offset_counter = y_offset_counter + 1; }
    }

    }

    //printmsg(x1, y1, &("x1: ".to_owned() + x1.to_string().as_str()));
    //printmsg(x1, y1+1, &("x2: ".to_owned() + x2.to_string().as_str()));
    
    //printmsg(x1, y1+3, &("loaded_vector: ".to_owned() + loaded_map.len().to_string().as_str()));

    //printmsg(x1, y1+5, &("loaded_info_0: ".to_owned() + loaded_info.0.to_string().as_str()));
    //printmsg(x1, y1+6, &("loaded_info_1: ".to_owned() + loaded_info.1.to_string().as_str()));
    
}
