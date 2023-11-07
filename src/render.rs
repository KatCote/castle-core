use std::fs::File;
use std::io::Read;

use crossterm::style::Print;

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
                        MapLayer::Base => render_map_layer(x1, y1, x2, y2, MapLayer::Base),
                        MapLayer::Color => printch(i, j, &'B'),
                        MapLayer::Trigger => render_map_layer(x1, y1, x2, y2, MapLayer::Trigger),
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

pub fn render_map_layer(x1: u16, y1: u16, x2: u16, y2: u16, map: MapLayer) {

    let mut map_path: String = String::from("maps/default.map");

    match map {
        MapLayer::Base => map_path = String::from("maps/base.map"),
        MapLayer::Color => map_path = String::from("maps/color.map"),
        MapLayer::Trigger => map_path = String::from("maps/trigger.map"),
        MapLayer::Wall => map_path = String::from("maps/wall.map"),
        MapLayer::SumObj(ref object) => {
            match object {
                Summon::Static => map_path = String::from("maps/summon/static.map"),
                Summon::Dynamic => map_path = String::from("maps/summon/dynamic.map"),
                Summon::NPC => map_path = String::from("maps/summon/npc.map"),
                Summon::Enemy => map_path = String::from("maps/summon/enemy.map")
            }   
        }
        MapLayer::Explore => map_path = String::from("maps/explore.map")
    }

    let map_path_to_file = File::open(&map_path);

    let mut map_file = match map_path_to_file {
        Ok(mfile) => mfile,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create(&map_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => { panic!("Problem opening the file: {:?}", other_error); }
        }
    };

    let mut map_contents = String::new();

    map_file.read_to_string(&mut map_contents).unwrap();

    printmsg(x1, y1, &map_contents);

}
