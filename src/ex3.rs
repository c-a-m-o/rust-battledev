use crate::{read_line, print};

pub fn main() {
    let n : usize = read_line().parse().unwrap();
    let mut res = vec![0;10];
    let mut sup = vec![0;n];
    for _ in 1..n {
        let line = read_line();
        let (a, b) = line.split_once(" ").unwrap();
        let a : usize = a.parse().unwrap();
        let b : usize = b.parse().unwrap();
        sup[a] = b;
    }

    let mut current = vec![0];
    for i in 0..10 {
        res[i] = current.len();
        let mut next = Vec::new();
        for agent in current {
            for j in 1..n { // Skip 0 : that's Dolan
                if sup[j] == agent {
                    next.push(j);
                }
            }
        }
        current = next;
    }

    let mut result = format!("{}", res[0]);
    for i in 1..10 {
        result = format!("{} {}", result, res[i]);
    }

    print(&result);
    
}