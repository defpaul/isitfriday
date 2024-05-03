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

    if &args[1] == "m" || &args[1] == "more" {
        println!("{:?}", offset::Local::now());
        println!("{:?}", offset::Utc::now());
    }
}
