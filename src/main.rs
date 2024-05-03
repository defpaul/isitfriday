use std::env;

use chrono::{self, offset, Datelike, Local, Weekday};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = chrono::offset::Local::now().date().weekday();
    let fri = Weekday::Fri;
    if day == fri {
       println!("Yes");
    }
    else {
        println!("Nope");
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
