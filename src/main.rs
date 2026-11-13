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
            "search" => {
                println!("Enter your search query:");
                let mut query = String::new();
                query.push_str("https://searxng.tail3da965.ts.net/search?q=");
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut query).expect("error");
                Command::new("zen-twilight").arg(query); 
                break;
            }
            "gh" => {Command::new("zen-twilight").arg("https://github.com").spawn().expect("error"); break;}
            "custom" => {
                println!("Enter the website to search, assumed https://");
                let mut website = String::new();
                website.push_str("https://");
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut website).expect("error");
                Command::new("zen-twilight").arg(website).spawn().expect("Failed to open");
                break;
            }
            "exit" => break,
            _ => {println!("Unknown command"); continue;}
        }
    }
    let _ = Command::new("pkill").arg("foot").spawn();
}