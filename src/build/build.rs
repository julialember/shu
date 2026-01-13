use std::{
    borrow::Cow, fmt, fs::OpenOptions, io::{self, Write}
};
use crate::build::{SplitError, split::split};

pub struct Build<'a> {
    args_left: Vec<&'a str>,
    output: Box<dyn Write + 'a>,
    errout: Box<dyn Write + 'a>,
}

pub enum BuildError<'a> {
    ReadError(&'a str, io::Error),
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
            Self::ReadError(name, e) => write!(f, "error with read({}): {}", name, e),
            Self::UnExpectedArg(arg) => write!(f, "unknown argument: {}", arg),
            Self::Other(name, e) => write!(f, "{}: {}", name, e),
        } 
    }
}

pub trait CommandBuild {
    fn new(args: Vec<&str>) -> Result<Self, BuildError<'_>>
        where 
            Self: Sized;
}

impl<'a> Build<'a> {
    fn init(command: &'a str) -> Result<Build<'a>, BuildError<'_>> {
        let args = split(command)?
            .into_iter();
        
        let mut index = 1;
        let mut args_left = vec![];
        let mut outfile = None;
        let mut errfile = None;
        while let Some(arg) = args.next() {
            match arg.as_ref() {
                ">>" => outfile = Some(Self::next_write(&mut args, arg,true)?),
                ">" => outfile = Some(Self::next_write(&mut args, arg, false)?),
                "2>" => errfile = Some(Self::next_write(&mut args, arg, false)?),
                "2>>" => errfile = Some(Self::next_write(&mut args, arg, true)?),
                unknown => args_left.push(unknown),
            }
        } 
        Ok(Self{
            args_left,
            output: match outfile {
                Some(file) => file,
                None => Box::new(io::stdout())
            },
            errout: match errfile {
                Some(err_file) => err_file,
                None => Box::new(io::stderr())
            }
        })

    }
 
    pub fn next_write(args: &mut impl Iterator<Item = &'a str>, last: &'a str, add_mode: bool)
            -> Result<Box<dyn Write+'a>, BuildError<'a>>{
        if let Some(arg) = args.next() {
            let file = Self::write_source(arg, add_mode)?;
            Ok(file)
        }
        else {
            Err(BuildError::NoArgument(last))
        }
    }
    
    fn write_source(filename: &'a str, add_mode: bool) -> Result<Box<dyn Write + 'a>, BuildError<'a>> {
        match OpenOptions::new()
            .create(true)
            .append(add_mode) 
            .write(true)
            .open(filename) {

            Ok(file) => Ok(Box::new(file)),
            Err(e) => Err(BuildError::ReadError(filename, e))
        }
    }
}



