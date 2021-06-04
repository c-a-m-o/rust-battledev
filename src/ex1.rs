use crate::{read_line, print};

pub fn main() {
    let n : usize = read_line().parse().unwrap();
    let mut count = 0;
    for _ in 0..n {
        let line = read_line();
        let name : Vec<u8> = line.into();
        let l = name.len();
        if l >= 5 {
            if name[l-5..l].iter().all(|&c| char::from(c).is_ascii_digit()) {
                count += 1;
            }
        }
    }
    print(&format!("{}", count));
}