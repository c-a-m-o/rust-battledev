use crate::{read_line, print};

fn print_vec<E : ToString>(v : &Vec<E>) {
    let res = v.into_iter().fold(String::new(), |mut s, e| {
        let e_str = e.to_string();
        if s.is_empty() {
            s.reserve(e_str.len());
            s.push_str(&e_str);
        } else {
            s.reserve(e_str.len()+1);
            s.push_str(" ");
            s.push_str(&e_str);
        }
        s
    });
    print(&res);
}

pub fn main() {
    let line = read_line();
    let (n,m) = line.split_once(' ').unwrap();
    let n : usize = n.parse().unwrap();
    let m : usize = m.parse().unwrap();
    let key_str = read_line();
    let mut key : Vec<usize> = key_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for i in 1..n {
        key[i] = key[i-1] ^ key[i];
    }
    let mut counts = vec![0;256];
    for _ in 0..m {
        let line = read_line();
        let (l, r) = line.split_once(' ').unwrap();
        let l : usize = l.parse().unwrap();
        let r : usize = r.parse().unwrap();
        let val = if l == 0 {
            key[r]
        } else {
            key[r] ^ key[l-1]
        };
        counts[val] += 1;
    }
    print_vec(&counts);
}