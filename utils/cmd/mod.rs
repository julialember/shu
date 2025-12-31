use std::{fmt, path::{Path, PathBuf}};

mod command;
mod ls;
mod grep;
mod cat;
mod head_tail;

use grep::{Grep, GrepError};
use cat::{Cat, CatError};
use head_tail::{HeadTail, HeadTailError};
use ls::{Ls, LsError};
    
use command::{
    CommandBuild, CommandBackPack
};

fn run<'a, E, B>(vec: Vec<&'a str>, path: &'a Path, pipe_mode: bool) -> bool 
    where 
        B: CommandBuild<'a, E>,
        E: fmt::Display,
{
    let (mut str, args, pipe_part) = 
            match CommandBackPack::new(vec, path) {
        Ok(args) => {
            args
        },
        Err(e) => {
            eprintln!("{}", e);
            return false;
        }
    };
    match B::new_obj(args, path, pipe_mode) {
        Ok(command) => 
            match command.run(&mut str) {
                Err(e) => {
                    if let Err(e) = writeln!(str.stderr, "{}", e) {
                        println!("error with write in stderr, so here the error: {}", e);
                    }
                    false
                }
                Ok(code) => {
                    match pipe_part {
                        Some(args) => set(args, path, true),
                        None => code
                    }
                }

            }
        Err(e) => {
            if let Err(e) = writeln!(str.stderr, "{}", e) {
                println!("error with write into stderr, so error: {}", e);
            }
            false
        }
    }
}

pub fn set(vec: Vec<&str>, path: &Path, pipe_mode: bool) -> bool {
    match vec[0] {
        "grep" => run::<'_, GrepError, Grep>(vec, path, pipe_mode),
        "cat" =>  run::<'_, CatError, Cat>(vec, path, pipe_mode),
        "head-tail" => run::<'_, HeadTailError, HeadTail>(vec, path, pipe_mode),
        "ls" => run::<'_, LsError, Ls>(vec, path, pipe_mode),
        _=> {
            eprintln!("shu: unknown command: {}", vec[0]);
            false
        }
        
    }
}

pub fn todo(command: &str, path: PathBuf) -> bool {
    let vec: Vec<&str> = split_args(command);
    set(vec, &path,false) 
}

fn split_args(command: &str) -> Vec<&str> {
    let mut vec = Vec::new();
    let mut start_arg = 0;
    let mut end_arg = 0;
    let mut chars = command.chars().peekable();
    let mut was_blank = false;
    while let Some(ch) = chars.next() {
        match ch {
            '#' => break,
            '>' => {
                if !was_blank {
                    vec.push(&command[start_arg..end_arg]);
                    was_blank=true;
                }
                if let Some(&nex) = chars.peek()
                    && nex == '>' {
                    chars.next();  
                    end_arg+=2;
                    vec.push(">>");
                } else {
                    end_arg+=1;
                    vec.push(">");
                }
                start_arg=end_arg;
            }
            '\'' | '"' => {
                start_arg+=1;
                while let Some(c) = chars.next()
                && c != ch {
                    end_arg+=1;
                }
                if end_arg < command.len() {
                    vec.push(&command[start_arg..=end_arg]);
                    end_arg+=2;
                    start_arg=end_arg;
                }
            } 
            ' ' => {
                if !was_blank {
                    vec.push(&command[start_arg..end_arg]);
                    was_blank=true;
                    end_arg+=1;
                    start_arg=end_arg;
                } else {
                    start_arg+=1;
                    end_arg+=1;
                }
            }
            _=> {
                end_arg+=1;
                was_blank=false;
            }
        } 
    }
    if start_arg != end_arg && start_arg < command.len() {
        vec.push(&command[start_arg..]);
    }
    vec
}
