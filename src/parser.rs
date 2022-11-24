//! # Parser
//!
//! `parser` parses an `Iterator` of `String` into the `StupidArgs` struct.

use crate::prelude::*;

/// # StupidArgs
///
/// This struct parses the first element of an Iterator of Strings and parses it into a message.
///
/// The intention behind all this is that it will take the commandline arguments and it will
/// return a struct with the first argument, the message, or if the argument size is 0 or more
/// than 1 it will return an error.
///
/// # Returns
///
/// ## StrupidArgs
/// If the iterator got exactly 2 arguments which means one commandline argument.
///
/// ## `Error::ToLessArgs`
/// If the iterator got less than 2 arguments which means no commandline argument.
///
/// ## `Error::ToMuchArgs`
/// If the iterator got more than 2 arguments which means more than 1 commandline argument.
///
/// # Example
///
/// ```
/// let argument = std::env::args();
/// # let argument = [String::from("bar"), String::from("foo"), String::from("foobar")];
/// # let argument = &mut test_data.iter().map(|s| s.to_owned());
///
/// let to_stupid_args = StrupidArgs::parse(&mut arguments).unwrap();
///
/// assert_eq!(to_stupid_args.msg(), "foo");
/// ```
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
