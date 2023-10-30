extern crate castlecore;

use castlecore::*;
use crate::screen::*;
use crate::render::*;
use crate::window::*;

fn main() { 
    initscr();

    //let _ = write_full_window(Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::SumObj(Summon::NPC))));

    /*write_vertical_split_window(
        Screen::RenderLayer(RenderInterface::Default), 
        Screen::RenderLayer(RenderInterface::MapFull),
        0.17
    );*/

    write_default_game_window(
        Screen::RenderLayer(RenderInterface::Default),
        Screen::RenderLayer(RenderInterface::Default),
        Screen::RenderLayer(RenderInterface::Default),
        Screen::RenderLayer(RenderInterface::Default),
        Screen::RenderLayer(RenderInterface::Default),
        0.7,
        0.6,
        0.5,
        0.5);

    usescr();
    endscr();
}

