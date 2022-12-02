use std::error::Error;
use std::io::prelude::*;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;
