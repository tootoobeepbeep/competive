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
fn sevens(xs: Vec<i32>) -> bool {
    for i in (0..xs.len()) {
        for j in ((i+1)..xs.len()) {
            if xs[i] != xs[j] && xs[i]+xs[j]==7777 {
                return true;
            }
        }
    }
    return false;
}

fn simulate(x:i32, d1c:i32, d1a:i32) -> bool { 

    let mut angry = true; 
    let mut xx = x;
    let mut tick = 1;
    let mut calm_tick = 1;
    let mut angry_tick = 1;
    while tick != x {
    println!("angry:{}calm:{}tick:{}", angry_tick, calm_tick, tick);
    match angry {
        true => {
            tick += 1;
            angry_tick += 1;
            if (angry_tick == d1a) {
                angry_tick = 1;
                angry = false;
            }
        }
        false => {
            tick += 1;
            calm_tick +=1;
            if (calm_tick == d1c) {
                calm_tick = 1;
                angry = true;
                }
            }
        }
    }
    angry
}


fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {

    println!("does this work? {}", simulate(3, 2, 2));

    




}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
