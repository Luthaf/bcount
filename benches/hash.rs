#![feature(test)]
extern crate test;
use test::Bencher;

extern crate bcount;
use bcount::Bc;

use std::hash::{Hash, SipHasher, Hasher};

#[derive(Hash)]
struct VeryBigStruct {
    vec: Vec<usize>,
}

impl VeryBigStruct {
    fn new() -> VeryBigStruct {
        VeryBigStruct{
            vec: vec![42; 10000],
        }
    }
}

#[bench]
fn raw(b: &mut Bencher) {
    let raw = VeryBigStruct::new();
    b.iter(|| hash(&raw));
}

struct Counted(Bc<VeryBigStruct>);
impl Hash for Counted {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.count().hash(state);
    }
}

#[bench]
fn counted(b: &mut Bencher) {
    let counted = Counted(Bc::new(VeryBigStruct::new()));
    b.iter(|| hash(&counted));
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}
