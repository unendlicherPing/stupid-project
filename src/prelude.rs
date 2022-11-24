pub use crate::error::Error;
pub use crate::parser::StupidArgs;

pub type Result<T> = core::result::Result<T, Error>;
