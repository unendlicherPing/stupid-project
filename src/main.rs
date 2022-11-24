//! This will be a very stupid project.

use prelude::*;

mod error;
mod prelude;
mod parser;


fn main() {
    let args: StupidArgs = match StupidArgs::parse_args(&mut std::env::args()) {
        Ok(args) => args,
        Err(Error::ToLessArgs) => { 
            eprintln!("To less arguments btw.");
            return;
        }
        Err(Error::ToMuchArgs) => {
            eprintln!("To much arguments btw.");
            return;
        }
    };

    println!("Your message was: {}", args.msg())
}

