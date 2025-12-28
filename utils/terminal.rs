use std::{
    env, fs::{OpenOptions}, 
    io::{self, Write, stdin, stdout}
};
mod cmd;

fn main() -> io::Result<()> { 
    let mut command = String::new();
    let mut shu_his = OpenOptions::new()
        .create(true).truncate(false).append(true).open(".shu_history")?;
    let now_dir = env::current_dir()?;
    loop {
        if let Some(name) = now_dir.file_stem() {
            print!("[{}]~$ ", name.display());
        } else {
            print!("[???]~$ ");
        }
        stdout().flush().expect("can't flush stdout");
        stdin().read_line(&mut command).expect("can't read line");
        if command.trim().len() == 0 {continue;}
        match command.trim() {
            "exit"=> {
                process_terminated();
                writeln!(shu_his, "exit")?;
                break;
            }
            "pwd" => {
                println!("{}", now_dir.display());
                writeln!(shu_his, "pwd")?; 
            }
            _ => {
                let code = cmd::todo(&command);
                println!("exit code: {}", if code {1} else {0});
                writeln!(shu_his, "{} {}", &command, if code {"ERROR"} else {""})?;
            }
        }
        command.clear();
    }
    
    Ok(())
}

fn process_terminated() {
    println!("tranks for using our terminal!");
}
