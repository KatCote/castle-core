extern crate castlecore;

use castlecore::functions;

fn main() { 
    { let _ = functions::initscr(); } 
    { let _ = functions::usescr(); }
    println!("");
    { let _ = functions::endscr(); }
}
