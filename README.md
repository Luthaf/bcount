# bcount

[![Build Status](https://travis-ci.org/Luthaf/bcount.svg?branch=master)](https://travis-ci.org/Luthaf/bcount)
[![code coverage](https://codecov.io/github/Luthaf/bcount/coverage.svg?branch=master)](https://codecov.io/github/Luthaf/bcount?branch=master)

This crate provide type for counting mutable borrow of a value. The `Bc<T>`
type is a small wrapper on top of a value of type `T` which count the number
of time the value has been mutably borrowed since it's creation.

[Documentation is here!](http://luthaf.github.io/bcount/bcount/index.html)

## Installation

`bcount` is on crates.io, so just add the following to your `Cargo.toml`
```
[dependencies]
bcount = "*"
```

## Why?

If you want to cache an expensive computation result, you need to have
information about wether the parameters of the computation have changed or
not. You can use a hash for that, but this have two shortcomings:

 * You need to have values which implement the `Hash` trait. Some useful
   types like `f64` do not;
 * You need to have a cheap way to compute the hash.

If computing the hash is harder or more expensive than doing the
computation, you are doomed. Or you can use `Bc<T>` which will give you
information about the number of borrow since the last computation. If this
number have changed, then it is very likely that the value have changed,
and that you need to redo your computation.

## Limitations

This can not be used a real hash algorithm, because the number of borrow can
change even if the value do not.

If more than `usize::MAX` borrow occurs, the borrow counter will be wrapped
around to 0, and will not `panic` because of the overflow.

## Performances

The `Bc` type do not introduce notable overhead when borrowing. Here are the
benchmark results comparing a raw value and a borrow counted value:

```text
running 2 tests
   test counted ... bench:       1,061 ns/iter (+/- 12)
   test raw     ... bench:       1,059 ns/iter (+/- 16)
```

Using the number of borrow as hash value is way faster than doing the real
hash. Here is a benchmark for `[usize; 10000]` values:

```text
running 2 tests
   test counted ... bench:          22 ns/iter (+/- 1)
   test raw     ... bench:      49,011 ns/iter (+/- 755)
```

You can see the code for these benchmarks on [Github](https://github.com/Luthaf/bcount/tree/master/benches).

## Example

```rust
extern crate bcount;
use bcount::Bc;

fn main() {
    let mut a = Bc::new(vec![63, 67, 42]);
    assert_eq!(a.count(), 0);

    do_work(&mut a);
    do_work(&mut a);
    do_work(&mut a);
    do_work(&mut a);

    assert_eq!(a.count(), 4);

    *a = vec![3, 4, 5];
    assert_eq!(a.count(), 5);
}

fn do_work(_: &mut [usize]) {
    // Whatever, nobody cares
}

```

## Licence

MIT
