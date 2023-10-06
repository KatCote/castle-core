extern crate castlecore;

use castlecore::*;

fn main() { 
    let _ = screen::initscr();
    let _ = border::write_full_window(border::Screen::RenderLayer(render::RenderInterface::MapLayer(render::Map::Base)));
    //let _ = border::write_full_window(border::Screen::Empty);
    let _ = screen::usescr();
    println!("");
    let _ = screen::endscr();
}
