use std::env;

use chrono::{self, offset, Datelike, Local, Weekday};

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
    let arg1: String = String::from("-m");
    let arg2: String = String::from("--more");
    for i in args {
        if *i == arg1 || *i == arg2 {
            print_time(); 
        } 
   } 
}

fn print_time() { 
    println!("\nLockal:\n{:?}", offset::Local::now());
    println!("Utc:\n{:?}", offset::Utc::now());
}

fn nop() {
    print!("\n \x1b[38;5;50m
     _______   ___     ______         _________
    /  _   |  /  /    /  __  \\       /  ___   / 
   /  / |  | /  /    /  /  \\  \\     /  /__/  / 
  /  /  |  |/  /    |  |    |  |   /  ______/ 
 /  /   |     /      \\  \\__/  /   /  / 
/__/    |____/        \\______/   /__/  ");
}

fn yup() {
    print!(" \x1b[38;5;200m
___      ___   ___      ___    ________
\\  \\    /  /  |   |    |   |  |   __   |
 \\  \\  /  /   |   |    |   |  |  |__|  |
  \\  \\/  /    |   |    |   |  |  ______|
   \\    /     |    \\___/   |  |  | 
    /  /       \\          /   |  |
   /__/         \\________/    |__|
   ");
}
