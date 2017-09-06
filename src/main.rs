extern crate qkx;

use std::env;
use std::process;
use qkx::Extractor;

fn main() {
    let extr = match Extractor::from(env::args().collect()) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = extr.run() {
        println!("{}", e);
        process::exit(1);
    }
}
