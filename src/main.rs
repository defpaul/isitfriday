use std::env;

use chrono::{self, offset, Datelike, Weekday};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = chrono::offset::Local::now().date().weekday();
    let fri = Weekday::Fri;
    let set = Weekday::Sat;
    let sun = Weekday::Sun;
    if day == fri {
       yup();
    }
    else if day == set || day == sun{
        weekend();
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
    let arg_h0: String = String::from("-h");
    let arg_h1: String = String::from("--help");
    for i in args {
        if *i == arg_m0 || *i == arg_m1 {
            print_time(); 
        } 
        else if *i == arg_d0 || *i == arg_d1{
            println!("\nDay:\n{:?}", offset::Local::now().date().weekday());
        }
        else if *i == arg_h0 || *i == arg_h1{
            help();
        }

   } 
}

fn print_time() { 
    println!("\nLockal:\n{:?}", offset::Local::now());
    println!("\nUtc:\n{:?}", offset::Utc::now());
}

fn help(){
    print!(r#"
        This is the help text to this programm.
            -m (--more) displais time
            -d (--day) displayes the day
            -h (--help) displais this masage
"#)
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

fn weekend(){
    colore("gold");
    print!(r#"

  _ _   _             _                    _       
 (_) |_( )___    __ _| |_ __ ___  __ _  __| |_   _ 
 | | __|// __|  / _` | | '__/ _ \/ _` |/ _` | | | |
 | | |_  \__ \ | (_| | | | |  __/ (_| | (_| | |_| |
 |_|\__| |___/  \__,_|_|_|  \___|\__,_|\__,_|\__, |
 __      _____  ___| | _____ _ __   __| |    |___/ 
 \ \ /\ / / _ \/ _ \ |/ / _ \ '_ \ / _` |          
  \ V  V /  __/  __/   <  __/ | | | (_| |          
   \_/\_/ \___|\___|_|\_\___|_| |_|\__,_|          
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
   \    /     |    \__/    |  |  | 
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
        "gold" => print!("\x1b[38;5;46m"),
       _ => ()
   }
}
