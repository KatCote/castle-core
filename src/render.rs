use std::fs::File;
use std::io::{BufReader, BufRead};

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
                        MapLayer::Base => { render_map_layer(x1, y1, x2, y2, MapLayer::Base, 10, 5); break; },
                        MapLayer::Color => { render_map_layer(x1, y1, x2, y2, MapLayer::Color, 0, 0); break; },
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

pub fn render_map_layer(x1: u16, y1: u16, x2: u16, y2: u16, map: MapLayer, x_offset: i16, y_offset: i16) {

    let loaded_map_pack = load_map(map);    // Need to move to loop (for 1 (or same) call load_map function)
                                                                        //       |
    let loaded_map: Vec<String> = loaded_map_pack.0;                   // ------+
    let loaded_info: (u16, u16) = loaded_map_pack.1;                    // ------+

    //let map_x_border = if x2-x1 < line.len() as u16 {x2-x1} else {line.len() as u16};
    //let map_y = if y1+line_num <= y2 {y1+line_num} else {/*break*/};

    //printmsg(x1, map_y, &line.as_str()[..map_x_border as usize]);

    printmsg(x1, y1, &("x1: ".to_owned() + x1.to_string().as_str()));
    printmsg(x1, y1+1, &("x2: ".to_owned() + x2.to_string().as_str()));
    
    printmsg(x1, y1+3, &("loaded_vector: ".to_owned() + loaded_map.len().to_string().as_str()));

    printmsg(x1, y1+5, &("loaded_info_0: ".to_owned() + loaded_info.0.to_string().as_str()));
    printmsg(x1, y1+6, &("loaded_info_1: ".to_owned() + loaded_info.1.to_string().as_str()));
    
}
