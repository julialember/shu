use std::{
    fs::OpenOptions, 
    io
};
use crate::build::{
    BuildError,
};


pub struct FilterArgs<'a> {
    args_left: Vec<&'a str>,
    outfile: Box<dyn io::Write + 'a>,
    errfile: Box<dyn io::Write + 'a>
}

impl<'a> FilterArgs<'a> {
     pub fn filter(args: Vec<&'a str>) -> Result<FilterArgs<'a>, BuildError<'a>> {
        let mut args = args.into_iter();

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
        Ok(Self {
            args_left,
            outfile: match outfile {
                Some(file) => file,
                None => Box::new(io::stdout())
            },
            errfile: match errfile {
                Some(err_file) => err_file,
                None => Box::new(io::stderr())
            }
        })

    }
 
    fn next_write(args: &mut impl Iterator<Item = &'a str>, last: &'a str, add_mode: bool)
            -> Result<Box<dyn io::Write+'a>, BuildError<'a>>{
        if let Some(arg) = args.next() {
            let file = Self::write_source(arg, add_mode)?;
            Ok(file)
        }
        else {
            Err(BuildError::NoArgument(last))
        }
    }
    
    fn write_source(filename: &'a str, add_mode: bool) 
            -> Result<Box<dyn io::Write + 'a>, BuildError<'a>> {
        match OpenOptions::new()
            .create(true)
            .append(add_mode) 
            .write(true)
            .open(filename) {

            Ok(file) => Ok(Box::new(file)),
            Err(e) => Err(BuildError::OpenError(filename, e))
        }
    }

}
