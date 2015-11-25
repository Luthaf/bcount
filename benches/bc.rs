#![feature(test)]
extern crate test;
use test::Bencher;

extern crate bcount;
use bcount::Bc;

#[bench]
fn counted(b: &mut Bencher) {
    let mut counted = Bc::new(1000);
    b.iter(|| do_work(&mut counted));
}

#[bench]
fn raw(b: &mut Bencher) {
    let mut raw = 1000;
    b.iter(|| do_work(&mut raw));
}

#[inline(never)]
fn do_work(n: &mut usize) -> f64 {
    let mut s = 0.0;
    for i in 0..*n {
        s += i as f64;
    }
    return s;
}
