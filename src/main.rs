extern crate chrono;
use std::io;
use chrono::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let period: i64 = args[1].parse().unwrap();
    let mut counter: u64 = 0;
    let mut line = String::new();
    let mut prev_time_slot: i64 = 0;
    loop{
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                counter += 1;
                let time_slot = Utc::now().timestamp()/period;
                if time_slot!=prev_time_slot {
                    println!("{} {}", NaiveDateTime::from_timestamp(time_slot*period, 0), counter);
                    counter = 0;
                    prev_time_slot = time_slot;
                }
            },
            Err(_) => {
                println!("{} {}", NaiveDateTime::from_timestamp(prev_time_slot*period, 0), counter);
                break;
            },
        };
    }
}
