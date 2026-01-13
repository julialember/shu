use std::{
    fmt, 
    io,
};
use crate::build::{SplitError, SplitArgs, FilterArgs};

pub enum BuildError<'a> {
    OpenError(&'a str, io::Error),
    SplitError(SplitError),
    UnExpectedArg(&'a str),    
    NoArgument(&'a str),
    Other(&'a str, Box<dyn fmt::Display + 'a>),
}

impl fmt::Display for BuildError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SplitError(e) => write!(f, "error with args split: {}", e),
            Self::NoArgument(arg) => write!(f, "no argument agter: {}", arg),
            Self::OpenError(name, e) => write!(f, "error with read({}): {}", name, e),
            Self::UnExpectedArg(arg) => write!(f, "unknown argument: {}", arg),
            Self::Other(name, e) => write!(f, "{}: {}", name, e),
        } 
    }
}

struct Build<'a> {
    filtered: FilterArgs<'a>,
}

impl<'a> Build<'a> {
    fn build(command: &'a str) -> Result<Build<'a>, BuildError<'a>>{
        let args = SplitArgs::split(command)?.rebuild();
        let filtered = FilterArgs::filter(args)?;
        Ok(Build{filtered})
        
    } 
}

pub trait CommandBuild {
    fn new(args: Vec<&str>) -> Result<Self, BuildError<'_>>
        where 
            Self: Sized;
}




