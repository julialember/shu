use std::{fmt, io::{BufRead, BufReader, Read}};
use crate::build::{};
/*

pub struct Grep<'a> {
    pattern: String,
    search_in: Vec<Box<dyn Read + 'a>>,
    ignore_case: bool,
    count: bool,
    number_lines: bool,
}

impl CommandRun for Grep<'_> {
    fn run(mut self) -> Result<(), RunError> where Self: Sized {
        if self.ignore_case {
            self.pattern = self.pattern.to_lowercase()
        }
        for read_source in self.search_in {
            let lines = 
                BufReader::new(read_source)
                .lines()
                .map_while(Result::ok);
            if self.count {
                let count = lines
                    .filter(|x| Self::match_pattern(&self.pattern, x, self.ignore_case))
                    .count();
                println!("{}", count) 
            } else {
                for (num, i) in lines
                    .enumerate()
                    .filter(|(_, x)| Self::match_pattern(&self.pattern, x, self.ignore_case)) {
                    if self.number_lines {
                        println!("{}. {}", num+1, i);
                    } else {
                        println!("{}", i);
                    }
                }
            }
        }
        Ok(()) 
    }
}

impl Grep<'_> {
    fn match_pattern(pattern: &str, search: &str, ignore_case: bool) -> bool {
        if ignore_case {
            search.to_lowercase().contains(pattern)
        } else {
            search.contains(pattern)
        }
    }
}

impl CommandBuild<GrepError> for Grep<'_> {
    fn init<'a>(args: Vec<&'a str>) -> Result<Self, BuildError<'a, GrepError>> {
        let mut pattern: Option<String> = None;
        let mut search_in = Vec::new();
        let mut ignore_case = false;
        let mut count = false;
        let mut number_lines = false;

        let mut index = 1;
        while index < args.len() {
            if args[index].starts_with('-') {
                match args[index] {
                    "-i" | "--ignore-case" => ignore_case = true,
                    "-c" | "--count" => count = true,
                    "-n" | "--number-lines" => number_lines = true,
                    unknown => return Err(BuildError::UnExpectedArg(unknown)),
                }
            } else if pattern.is_none() {
                pattern = Some(args[index].to_string())
            } else {
                let file = Self::read_source(args[index])?;
                search_in.push(file);
            }
            index+=1;
        }
        if search_in.is_empty() {
            let file = Self::read_source("-")?;
            search_in.push(file);
        }
        
        match pattern {
            Some(pattern) => 
                Ok(Self {
                    pattern,
                    ignore_case,
                    number_lines,
                    count,
                    search_in
                }),
            None => Err(BuildError::Other("grep", GrepError::NoPattern))
        }
    }
}

pub enum GrepError{
    NoPattern
}

impl fmt::Display for GrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPattern => writeln!(f, "no pattern to found")
        } 
    }
}
*/
