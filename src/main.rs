use std::env;
use std::process;

use bckup::Config;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprint!("Problem parsing aguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = bckup::run(config){
        eprint!("Application error: {}", e);

        process::exit(1);
    }   
}
  