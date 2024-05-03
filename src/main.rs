use chrono::{self, Datelike, Weekday};

fn main() {
    let day = chrono::offset::Local::now().date().weekday();
    let fri = Weekday::Fri;
    if day == fri {
       println!("Yes");
    }
    else {
        println!("Nope");
    }
}
