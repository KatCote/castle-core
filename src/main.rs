extern crate castlecore;

use castlecore::*;

fn main() { 
    let _ = screen::initscr();
    let _ = border::write_full_window();
    let _ = screen::usescr();
    println!("");
    let _ = screen::endscr();
}
