mod command;
mod grep;
use grep::Grep;
use command::Command;


pub fn todo(command: &str) -> bool {
    let vec: Vec<&str> = command.split_whitespace().collect();
    match vec[0] {
        "grep" => match Grep::new(vec) {
            Ok(g) => if let Err(e) = g.run() {
                eprintln!("error: {}",  e);
                return false;
            }
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        },
        _=> {
            println!("shu: unknown command: {}", vec[0]);
            return false;
        }
        
    }
    true
}
