#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::io::{self, prelude::*};
use std::str;
use std::f64::consts::PI;
struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>,
}
impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }
    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
    fn read_str(&mut self) -> String {
        let mut s = String::new();
        self.reader.read_line(&mut s);
        return s; 
   }
}
//fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
   //let n:i32 = scan.token();
   //let m:i32 = scan.token();

   //let some_str = scan.read_str();
   //let mut set:HashSet<&str> = some_str.split(' ').collect();
   //for i in 1..n {
       //let other_str = scan.read_str();
       //let other:HashSet<&str> = other_str.rsplit(' ').collect();
       //set = set.intersection(&other).copied().collect();
   //}
//}

fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let n:i32 = scan.token();
    let mut cap:i32 = scan.token();
    let mut count = 0;
    let mut flag = true;
    let numbers = (0..n).map(|_| scan.token()).collect::<Vec<i32>>();
    for num in numbers {
        cap = cap - num;
        if 0 >= cap {
            writeln!(w, "{}", count);
            flag = false;
            break;
        } 
        count += 1;
    }
    if flag {
        writeln!(w, "{}", count);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
