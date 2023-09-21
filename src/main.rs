extern crate castlecore;

use castlecore::functions;

fn main() {
    { let _ = functions::print_hello(); }
    castlecore::cc_ver();
}
