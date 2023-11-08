extern crate castlecore;

use castlecore::*;
use castlecore::core::initpath;
use crate::screen::*;
use crate::render::*;
use crate::window::*;

fn main() { 

    initpath();
    initscr();

    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Base)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.8,
        0.97, // >0.02 && <0.098
        0.5,
        0.5);

    usescr();
    endscr();
}
