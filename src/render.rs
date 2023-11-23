use crossterm::style::Color::Rgb;

use crate::core::load_map;
use crate::functions::*;

/// Enum for use in match in functions as param
pub enum RenderInterface {
    Default, // Plug                                (Dev)
    MapFull, // Regular                             (User)
    MapLayer(MapLayer), // Onlu for development     (Dev)
    MiniMap, // Non-regular                         (User)
    InvPage(InvPage), // Regular                    (User)
    PlayerParams, // Regular                        (User)
    InfoWorld, // Regular                           (User)
    InfoItem // Regular                             (User)
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
    let j = 1;
    let i = 1;

    match layer {
        RenderInterface::Default => { let _ = printch(i, j, &'.'); },
        RenderInterface::MapFull => { render_full_map(x1, y1, x2, y2, 80, 60); }, // 80, 80 
        RenderInterface::MapLayer(ref ml) => {  
            match ml {
                MapLayer::Base => { render_map_layer(x1, y1, x2, y2, MapLayer::Base, 0, 0); },
                MapLayer::Color => { render_map_layer(x1, y1, x2, y2, MapLayer::Color, 0, 0); },
                MapLayer::Trigger => { render_map_layer(x1, y1, x2, y2, MapLayer::Trigger, 0, 0); },
                MapLayer::Wall => { render_map_layer(x1, y1, x2, y2, MapLayer::Wall, 0, 0); },
                MapLayer::SumObj(ref object) => {
                    match object {
                        Summon::Static => printch(i, j, &'!'),
                        Summon::Dynamic => printch(i, j, &'@'),
                        Summon::NPC => printch(i, j, &'#'),
                        Summon::Enemy => printch(i, j, &'$')
                    }   
                }
                MapLayer::Explore => { render_map_layer(x1, y1, x2, y2, MapLayer::Explore, 0, 0); }
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

pub fn render_full_map(x1: u16, y1: u16, x2: u16, y2: u16, x_offset: u16, y_offset: u16) {
    let loaded_base = load_map(MapLayer::Base);
    let loaded_color = load_map(MapLayer::Color);

    let mut x_offset_counter: u16 = 0;
    let mut y_offset_counter: u16 = 0;

    let mut base_row_counter = 1;

    for base_row in &loaded_base {

        if y_offset_counter < y_offset { y_offset_counter = y_offset_counter + 1; continue; }

        let mut base_col_counter = 1;
        let temp_index_row = base_row_counter; // IDK

        let color_row = match loaded_color.get(temp_index_row - 1 + y_offset_counter as usize) { // NEED TO REWRITE // TODO
            Some(some_string) => some_string,
            None => ""
        };

        let color_chars_vec: Vec<char> = color_row.chars().collect();
        let base_chars_vec: Vec<char> = base_row.chars().collect();
        
        for base_char in base_chars_vec {

            if x_offset_counter < x_offset { x_offset_counter = x_offset_counter + 1; continue; }

            let temp_index_col = base_col_counter; // IDK

            let color_char = match color_chars_vec.get(temp_index_col - 1 + x_offset as usize) {
                Some(some_char) => some_char,
                None => &' '
            };

            match color_char { // Need to rewrite with any map types
                '.' => set_color(Rgb { r: 0, g: 119, b: 16 }, Rgb { r: 179, g: 255, b: 94 }),
                '#' => set_color(Rgb { r: 108, g: 174, b: 0 }, Rgb { r: 153, g: 90, b: 0 }),
                ',' => set_color(Rgb { r: 85, g: 56, b: 0 }, Rgb { r: 108, g: 174, b: 0 }),
                'H' => set_color(Rgb { r: 168, g: 168, b: 168 }, Rgb { r: 156, g: 66, b: 0 }),
                '=' => set_color(Rgb { r: 150, g: 107, b: 76 }, Rgb { r: 56, g: 24, b: 0 }),
                _ => ()
            }
            
            //printch(base_col_counter as u16, base_row_counter as u16, &base_char);
            printch(base_col_counter as u16 + x1 - 1, base_row_counter as u16 + y1 - 1, &base_char);

            reset_color();

            if base_col_counter >= x2 as usize - 1 { break; } else { base_col_counter = base_col_counter + 1; }
        }

        x_offset_counter = 0;

        if base_row_counter >= y2 as usize - 1 { break; } else { base_row_counter = base_row_counter + 1; } 
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
