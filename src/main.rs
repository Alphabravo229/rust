use std::process::{Command, exit};
use std::io::{self, Write};

fn main() {
    print!("Enter the site you want to open: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    match input.trim() {
        "yt" => {Command::new("zen-twilight").arg("https://www.youtube.com/").spawn().expect("Failed to open");}
        "skl" => {
            Command::new("zen-twilight").arg("https://classroom.google.com/u/1/h/st").spawn().expect("Failed to open");
            Command::new("zen-twilight").arg("https://web.spaggiari.eu/home/app/default/login.php").spawn().expect("Failed to open");
        }
        _ => exit(0),
    }
    Command::new("pkill").arg("foot").spawn().expect("Failed to open");
}
