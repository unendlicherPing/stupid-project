//! This will be a very stupid project.

use std::env::Args;

enum StupidError {
    ToMuchArgs
}

type StupidResult<T> = Result<T, StupidError>;

struct StupidArgs {
    msg: String,
}

impl StupidArgs {
    fn parse_args(args: Args) -> StupidResult<Self> {
        let args: Vec<String> = args.collect();
        
        if args.len() >= 1 {
            return Err(StupidError::ToMuchArgs)
        }

        let arg = args.get(0).unwrap();

        Ok(Self { msg: arg.to_owned() })
    }

    fn msg(&self) -> &String { &self.msg }
}

fn main() {
    let args: StupidArgs = match StupidArgs::parse_args(std::env::args()) {
        Ok(args) => args,
        Err(_) => panic!("To much arguments btw."),
    };

    println!("Your message was: {}", args.msg())
}
