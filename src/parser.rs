use crate::prelude::*;

#[derive(Debug)]
pub struct StupidArgs {
    msg: String,
}

impl StupidArgs {
    pub fn parse_args(args: &mut dyn Iterator<Item = String>) -> Result<Self> {
        let args: Vec<String> = args.collect();

        if args.len() < 2 {
            return Err(Error::ToLessArgs)
        }

        if args.len() > 2 {
            return Err(Error::ToMuchArgs)
        }

        let arg = args.get(1).unwrap();

        Ok(Self { msg: arg.to_owned() })
    }

    pub fn msg(&self) -> &String { &self.msg }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_parser_result() {
        let test_data = [String::from("foo"), String::from("bar")];
        let test_data = &mut test_data.iter().map(|s| s.to_owned());

        let args: Result<StupidArgs> = StupidArgs::parse_args(test_data);

        assert_eq!(args.unwrap().msg(), "bar")
    }

    #[test]
    fn test_parser_to_low() {
        let test_data = [String::from("foo")];
        let test_data = &mut test_data.iter().map(|s| s.to_owned());

        let args: Result<StupidArgs> = StupidArgs::parse_args(test_data);

        let Err(Error::ToLessArgs) = args else {
            panic!()
        };
    }

    #[test]
    fn test_parser_to_large() {
        let test_data = [String::from("foo"), String::from("bar"), String::from("foobar")];
        let test_data = &mut test_data.iter().map(|s| s.to_owned());

        let args: Result<StupidArgs> = StupidArgs::parse_args(test_data);

        let Err(Error::ToMuchArgs) = args else {
            panic!()
        };
    }
}
