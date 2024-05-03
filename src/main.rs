use chrono::{self, Datelike};

fn main() {
    println!("{:?}", chrono::offset::Local::now());

    println!("{:?}", chrono::offset::Utc::now());

    println!("{}", chrono::offset::Local::now().date().weekday());
}
