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

#[cfg(test)]
mod test {
    use crate::{StupidArgs, StupidResult, StupidError};

    #[test]
    fn test_parser_result() {
        let test_data = [String::from("foo"), String::from("bar")];
        let test_data = &mut test_data.iter().map(|s| s.to_owned());

        let args: StupidResult<StupidArgs> = StupidArgs::parse_args(test_data);

        assert_eq!(args.unwrap().msg(), "bar")
    }

    #[test]
    fn test_parser_to_low() {
        let test_data = [String::from("foo")];
        let test_data = &mut test_data.iter().map(|s| s.to_owned());

        let args: StupidResult<StupidArgs> = StupidArgs::parse_args(test_data);

        let Err(StupidError::ToLessArgs) = args else {
            panic!()
        };
    }

    #[test]
    fn test_parser_to_large() {
        let test_data = [String::from("foo"), String::from("bar"), String::from("foobar")];
        let test_data = &mut test_data.iter().map(|s| s.to_owned());

        let args: StupidResult<StupidArgs> = StupidArgs::parse_args(test_data);

        let Err(StupidError::ToMuchArgs) = args else {
            panic!()
        };
    }
}
