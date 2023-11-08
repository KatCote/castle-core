use std::{fs::File, io::{BufReader, BufRead}};

use crate::render::{self, *};

pub fn load_map(map: render::MapLayer) -> (Vec<String>, (u16, u16)) {
    let mut map_path: String = String::from("maps/default.cc_map");

    match map {
        MapLayer::Base => map_path = String::from("maps/base.cc_map"),
        MapLayer::Color => map_path = String::from("maps/color.cc_map"),
        MapLayer::Trigger => map_path = String::from("maps/trigger.cc_map"),
        MapLayer::Wall => map_path = String::from("maps/wall.cc_map"),
        MapLayer::SumObj(ref object) => {
            match object {
                Summon::Static => map_path = String::from("maps/summon/static.cc_map"),
                Summon::Dynamic => map_path = String::from("maps/summon/dynamic.cc_map"),
                Summon::NPC => map_path = String::from("maps/summon/npc.cc_map"),
                Summon::Enemy => map_path = String::from("maps/summon/enemy.cc_map")
            }   
        }
        MapLayer::Explore => map_path = String::from("maps/explore.cc_map")
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

    let mut loaded_info: (u16, u16) = (1, 1); // 1 - X; 2 - Y; 3 - Need to remember :P

    let mut line_num = 1;

    for line in reader.lines() {

        match line {
            Ok(line) => {

                //loaded_map[line_num as usize] = line.as_str();
                
                loaded_info.0 = line_num;
                loaded_info.1 = if loaded_info.1 > (&line).len() as u16 {loaded_info.1} else {line.len() as u16};

                loaded_map.push(line);
            }
            Err(_) => break
        }

        line_num = line_num + 1;
    };

    return (loaded_map, loaded_info);

}
