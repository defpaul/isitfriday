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
    let arg1: String = String::from("-m");
    let arg2: String = String::from("--more");
    let arg3: String = String::from("-d");
    let arg4: String = String::from("--day");
    for i in args {
        if *i == arg1 || *i == arg2 {
            print_time(); 
        } 
        else if *i == arg3 || *i == arg4{
            println!("\nDay:\n{:?}", offset::Local::now().date().weekday());
        }
   } 
}

fn print_time() { 
    println!("\nLockal:\n{:?}", offset::Local::now());
    println!("Utc:\n{:?}", offset::Utc::now());
}

fn nop() {
    print!("\n \x1b[38;5;200m
     _______   ___     ______         _________
    /  _   |  /  /    /  __  \\       /  ___   / 
   /  / |  | /  /    /  /  \\  \\     /  /__/  / 
  /  /  |  |/  /    |  |    |  |   /  ______/ 
 /  /   |     /      \\  \\__/  /   /  / 
/__/    |____/        \\______/   /__/  
");
}

fn yup() {
    print!(" \x1b[38;5;50m
___      ___   ___      ___    ________
\\  \\    /  /  |   |    |   |  |   __   |
 \\  \\  /  /   |   |    |   |  |  |__|  |
  \\  \\/  /    |   |    |   |  |  ______|
   \\    /     |    \\___/   |  |  | 
    /  /       \\          /   |  |
   /__/         \\________/    |__|
   ");
}
