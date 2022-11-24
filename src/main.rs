//! This will be a very stupid project.

#[derive(Debug)]
enum StupidError {
    ToLessArgs,
    ToMuchArgs,
}

type StupidResult<T> = Result<T, StupidError>;

#[derive(Debug)]
struct StupidArgs {
    msg: String,
}

impl StupidArgs {
    fn parse_args(args: &mut dyn Iterator<Item = String>) -> StupidResult<Self> {
        let args: Vec<String> = args.collect();

        if args.len() < 2 {
            return Err(StupidError::ToLessArgs)
        }

        if args.len() > 2 {
            return Err(StupidError::ToMuchArgs)
        }

        let arg = args.get(1).unwrap();

        Ok(Self { msg: arg.to_owned() })
    }

    fn msg(&self) -> &String { &self.msg }
}

fn main() {
    let args: StupidArgs = match StupidArgs::parse_args(&mut std::env::args()) {
        Ok(args) => args,
        Err(StupidError::ToLessArgs) => { 
            eprintln!("To less arguments btw.");
            return;
        }
        Err(StupidError::ToMuchArgs) => {
            eprintln!("To much arguments btw.");
            return;
        }
    };

    println!("Your message was: {}", args.msg())
}
