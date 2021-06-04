use crate::{read_line, print};

pub fn main() {
    let n : usize = read_line().parse().unwrap();
    let fake = (0..n).filter_map(|_| {
        let line = read_line();
        let (hour, _minute) = line.split_once(':').unwrap();
        let hour : usize = hour.parse().unwrap();
        if hour < 8 || hour >= 20 {
            Some(())
        } else {
            None
        }
    }).count();
    if fake * 2 > n {
        print("SUSPICIOUS");
    } else {
        print("OK");
    }
}