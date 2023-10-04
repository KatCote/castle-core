extern crate castlecore;

use castlecore::functions;

fn main() { 
    { let _ = functions::initscr(); }
    { let _ = functions::create_full_window(); }
    //{ let _ = functions::print_hello(); } 
    { let _ = functions::usescr(); }
    println!("");
    { let _ = functions::endscr(); }
}
