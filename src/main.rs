extern crate castlecore;

use castlecore::*;
use crate::screen::*;
use crate::render::*;
use crate::border::*;

fn main() { 
    let _ = initscr();
    let _ = write_full_window(Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::SumObj(Summon::NPC))));
    //let _ = write_full_window(Screen::Empty);
    let _ = usescr();
    println!("");
    let _ = endscr();
}

