use std::process::{Command};
use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter the site you want to open: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
        match input.trim() {
            "yt" => {Command::new("zen-twilight").arg("https://www.youtube.com/").spawn().expect("Failed to open"); break;}
            "skl" => {
                Command::new("zen-twilight").arg("https://classroom.google.com/u/1/h/st").spawn().expect("Failed to open");
                Command::new("zen-twilight").arg("https://web.spaggiari.eu/home/app/default/login.php").spawn().expect("Failed to open");
                break;
            }
            "custom" => {
                println!("Enter the website to search, assumed https://");
                let mut website = String::new();
                website.push_str("https://");
                io::stdin().read_line(&mut website).expect("error");
                Command::new("zen-twilight").arg(website).spawn().expect("Failed to open");
                break;
            }
            "exit" => break,
            _ => {println!("Unknown command"); continue;}
        }
    }
}