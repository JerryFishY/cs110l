/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for i in v.iter() {
        res.push(i+n);
    }
    res
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in 0..(n-1) {
        v[i as usize]=v[i as usize]+n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut n : usize = v.len();
    let mut i = 0;
    let mut record :HashSet<i32> = HashSet::new();
    println!("len: {} .", n);
    loop {
        println!("len: {} .", n);
        if i > n-1 {
            break;
        }
        println!("idx: {} .", i);
        if record.contains(&v[i]){
            println!("{} exists in the HashSet.", v[i]);
            v.remove(i as usize);
            n=n-1;
        }else{
            println!("{} not exists in the HashSet.", v[i]);
            record.insert(v[i as usize]);
            i=i+1;
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
