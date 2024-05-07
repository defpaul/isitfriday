use std::env;

use chrono::{self, offset, Datelike, Weekday};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = chrono::offset::Local::now().date().weekday();
    let fri = Weekday::Fri;
    if day == fri {
       yup();
    }
    else {
        nop();
    }
    match_args(&args)
}

fn match_args(args: &[String]) {
    let arg_m0: String = String::from("-m");
    let arg_m1: String = String::from("--more");
    let arg_d0: String = String::from("-d");
    let arg_d1: String = String::from("--day");
    for i in args {
        if *i == arg_m0 || *i == arg_m1 {
            print_time(); 
        } 
        else if *i == arg_d0 || *i == arg_d1{
            println!("\nDay:\n{:?}", offset::Local::now().date().weekday());
        }
   } 
}

fn print_time() { 
    println!("\nLockal:\n{:?}", offset::Local::now());
    println!("\nUtc:\n{:?}", offset::Utc::now());
}

fn nop() {
    colore("red");
    print!(r#"
     _______   ___     ______         _________
    /  _   |  /  /    /  __  \       /  ___   / 
   /  / |  | /  /    /  /  \  \     /  /__/  / 
  /  /  |  |/  /    |  |    |  |   /  ______/ 
 /  /   |     /      \  \__/  /   /  / 
/__/    |____/        \______/   /__/  
"#);
    colore("clear");
}

fn yup() {
    colore("grean");
    print!(r#"
___      ___   ___      ___    ________
\  \    /  /  |   |    |   |  |   __   |
 \  \  /  /   |   |    |   |  |  |__|  |
  \  \/  /    |   |    |   |  |  ______|
   \    /     |    \___/   |  |  | 
    /  /       \          /   |  |
   /__/         \________/    |__|
"#);
    colore("clear");
}


fn colore(color: &str) {
   match color {
       "red" => print!("\x1b[38;5;196m"),
        "grean" => print!("\x1b[38;5;82m"), 
        "clear" => print!("\x1b[0m"),
       _ => ()
   }
}
