use std::{
    fmt, 
    io,
};
use crate::build::{ split::split, FilterArgs};

pub fn build<'a>(command: &'a str) -> Result<(), BuildError>{
    let args = split(command);
    let filtered = FilterArgs::filter(args)?;
    Ok(())
}

pub enum BuildError {
    OpenError(String, io::Error),
    UnExpectedArg(String),    
    NoArgument(String),
    Other(String, Box<dyn fmt::Display>),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArgument(arg) => write!(f, "no argument agter: {}", arg),
            Self::OpenError(name, e) => write!(f, "error with read({}): {}", name, e),
            Self::UnExpectedArg(arg) => write!(f, "unknown argument: {}", arg),
            Self::Other(name, e) => write!(f, "{}: {}", name, e),
        } 
    }
}

pub trait CommandBuild {
    fn new(args: Vec<String>) -> Result<Self, BuildError>
        where 
            Self: Sized;
}




