use std::{fs::{File, self}, io::{BufReader, BufRead}};

use crate::render::{self, *};

/// Load map (return this map, then use this not at loop!)
pub fn load_map(map: render::MapLayer) -> Vec<String> {
    let mut map_path: String = String::from("map/default.cc_map");

    match map {
        MapLayer::Base => map_path = String::from("map/base.cc_map"),
        MapLayer::Color => map_path = String::from("map/color.cc_map"),
        MapLayer::Trigger => map_path = String::from("map/trigger.cc_map"),
        MapLayer::Wall => map_path = String::from("map/wall.cc_map"),
        MapLayer::SumObj(ref object) => {
            match object {
                Summon::Static => map_path = String::from("map/summon/static.cc_map"),
                Summon::Dynamic => map_path = String::from("map/summon/dynamic.cc_map"),
                Summon::NPC => map_path = String::from("map/summon/npc.cc_map"),
                Summon::Enemy => map_path = String::from("map/summon/enemy.cc_map")
            }   
        }
        MapLayer::Explore => map_path = String::from("map/explore.cc_map")
    }

    let map_path_to_file = File::open(&map_path);

    let map_file = match map_path_to_file {
        Ok(mfile) => mfile,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create(&map_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };

    let reader = BufReader::new(&map_file);


    let mut loaded_map: Vec<String> = Vec::new();

    let mut line_num = 1;

    for line in reader.lines() {

        match line {
            Ok(line) => loaded_map.push(line),
            Err(_) => break
        }

        line_num = line_num + 1;
    };

    return loaded_map;

}

/// Create all dirs for correct engine work
pub fn initpath() {
    match fs::create_dir("map") {
        Err(error) => match error.kind() {
            std::io::ErrorKind::AlreadyExists => {},
            e => panic!("Problem create dir {:?}", e)
        },
        Ok(_) => {},
    }
    match fs::create_dir("map/summon") {
        Err(error) => match error.kind() {
            std::io::ErrorKind::AlreadyExists => {},
            e => panic!("Problem create dir {:?}", e)
        },
        Ok(_) => {},
    }
}
