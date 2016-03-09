//! This crate provide type for counting mutable borrow of a value. The `Bc<T>`
//! type is a small wrapper on top of a value of type `T` which count the number
//! of time the value has been mutably borrowed since it's creation.
//!
//! # Why?
//!
//! If you want to cache an expensive computation result, you need to have
//! information about wether the parameters of the computation have changed or
//! not. You can use a hash for that, but this have two shortcomings:
//!
//!  * You need to have values which implement the `Hash` trait. Some useful
//!    types like `f64` do not;
//!  * You need to have a cheap way to compute the hash.
//!
//! If computing the hash is harder or more expensive than doing the
//! computation, you are doomed. Or you can use `Bc<T>` which will give you
//! information about the number of borrow since the last computation. If this
//! number have changed, then it is very likely that the value have changed,
//! and that you need to redo your computation.
//!
//! # Limitations
//!
//! This can not be used a real hash algorithm, because the number of borrow can
//! change even if the value do not.
//!
//! If more than `usize::MAX` borrow occurs, the borrow counter will be wrapped
//! around to 0, and will not `panic` because of the overflow.
//!
//! # Performances
//!
//! The `Bc` type do not introduce notable overhead when borrowing. Here are the
//! benchmark results comparing a raw value and a borrow counted value:
//!
//! ```text
//! running 2 tests
//!    test counted ... bench:       1,061 ns/iter (+/- 12)
//!    test raw     ... bench:       1,059 ns/iter (+/- 16)
//! ```
//!
//! Using the number of borrow as hash value is way faster than doing the real
//! hash. Here is a benchmark for `[usize; 10000]` values:
//!
//! ```text
//! running 2 tests
//!    test counted ... bench:          22 ns/iter (+/- 1)
//!    test raw     ... bench:      49,011 ns/iter (+/- 755)
//! ```
//!
//! You can see the code for these benchmarks on [Github](https://github.com/Luthaf/bcount/tree/master/benches).
//!
//! # Example
//!
//! ```rust
//! extern crate bcount;
//! use bcount::Bc;
//!
//! fn main() {
//!     let mut a = Bc::new(vec![63, 67, 42]);
//!     assert_eq!(a.count(), 0);
//!
//!     do_work(&mut a);
//!     do_work(&mut a);
//!     do_work(&mut a);
//!     do_work(&mut a);
//!
//!     assert_eq!(a.count(), 4);
//!
//!     *a = vec![3, 4, 5];
//!     assert_eq!(a.count(), 5);
//! }
//!
//! fn do_work(_: &mut [usize]) {
//!     // Whatever, nobody cares
//! }
//!
//! ```
//!
//! # Caveats:
//!
//! This only count direct mutable borrow, nothing else. This means that
//! multiple borrow are only counted once. In this code, a call to
//! `borrow_me_twice` will only augment the borrow count by one.
//!
//! ```text
//! fn borrow_me(reference: &mut T) {
//!     // Do work
//! }
//!
//! fn borrow_me_twice(reference: &mut T) {
//!     // Do work
//!     borrow_me(reference)
//! }
//! ```
//!
//! Also, `Cell` and `RefCell` allow programmers to separate mutability from
//! mutable references, so with `Bc<Cell<T>>` the borrow count will never
//! change, even if the internal `T` is modified.

// TODO? Mbc (Mutable Borrow counter) & Cbc (*const* borrow counter) & Bc (*all* borrow counter)

#![deny(missing_docs)]
use std::ops::{Deref, DerefMut};

/// The borrow counter struct for type `T`.
pub struct Bc<T> {
    counter: usize,
    val: T
}

impl<T> Bc<T> {
    /// Create a new `Bc<T>` containing the value `val`.
    pub fn new(val: T) -> Bc<T> {
        Bc {
            val: val,
            counter: 0,
        }
    }

    /// Reset the borrow counter
    pub fn reset(&mut self) {
        self.counter = 0;
    }

    /// Get the number of time this structure has been mutably borrowed.
    pub fn count(&self) -> usize {
        self.counter
    }
}

impl<T> Deref for Bc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.val
    }
}

impl<T> DerefMut for Bc<T> {
     fn deref_mut(&mut self) -> &mut T {
         self.counter = self.counter.wrapping_add(1);
         &mut self.val
     }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::usize;

    fn do_nothing(_: &mut f64) {}

    #[test]
    fn count() {
        let mut a = Bc::new(5.0);
        assert_eq!(a.count(), 0);

        *a = 89.0;
        assert_eq!(a.count(), 1);

        do_nothing(&mut a);
        assert_eq!(a.count(), 2);
    }

    #[test]
    fn reset() {
        let mut a = Bc::new(3);

        assert_eq!(a.count(), 0);

        *a = 18;
        *a = 42;
        assert_eq!(a.count(), 2);

        a.reset();
        assert_eq!(a.count(), 0);
    }

    #[test]
    fn overflow() {
        let mut a = Bc::new(3);
        a.counter = usize::MAX - 1;

        *a = 18;
        *a = 18;
        assert_eq!(a.count(), 0);
    }

    #[test]
    fn non_mutable() {
        fn observe(_: &f64) {/* Do nothing */}

        let a = Bc::new(3.0);
        assert_eq!(a.count(), 0);

        observe(&a);
        observe(&a);
        observe(&a);
        assert_eq!(a.count(), 0);
    }
}
