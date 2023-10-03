extern crate castlecore;

use castlecore::functions;

fn main() { 
    { let _ = functions::initscr(); } 
    //let a: char = 'X';
    //functions::mvaddch(0, 0, &a);
    { let _ = functions::usescr(); }
    println!("");
    { let _ = functions::endscr(); }
}
