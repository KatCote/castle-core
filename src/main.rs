extern crate castlecore;

use castlecore::functions;
use crossterm::terminal::size;

fn main() { 
    let _ = functions::initscr();
    let _ = functions::border_full_window();
    let Ok((_cols, _rows)) = size() else { todo!() };
    let _ = functions::print_hello(_cols/2 - 12, _rows);
    let _ = functions::usescr();
    println!("");
    let _ = functions::endscr();
}
