extern crate castlecore;

use castlecore::*;
use crate::screen::*;
use crate::render::*;
use crate::window::*;

fn main() { 
    let _ = initscr();

    //let _ = write_full_window(Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::SumObj(Summon::NPC))));

    let _ = write_vertical_split_window(Screen::RenderLayer(RenderInterface::Default), 
    Screen::RenderLayer(RenderInterface::MapFull), 0.17);

    let _ = usescr();
    let _ = endscr();
}

