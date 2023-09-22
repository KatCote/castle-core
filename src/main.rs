extern crate castlecore;

use castlecore::functions;

fn main() { 
    { let _ = functions::initscr(true); }
    { let _ = functions::print_hello(); }
    println!("");
}
