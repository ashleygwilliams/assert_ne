# assert_ne
> assert not equals

this is a rust crate that affords you a macro, `assert_ne!`.

## usage

```
assert_ne!(3, 3);
// panicked at 'assertion failed: 
// `(left !== right)` (left: `3`, right: `3`)

assert_ne!(3, 4);
// :)
```

## prerequisites

this a rust crate. to use and work on this project, you'll
need to have rust installed. to install rust, i recommend
using [rustup].

[rustup]: https://www.rustup.rs/

## up and running

1. fork and clone this repository
2. `cd assert_ne`
3. `cargo build`

## testing

`cargo test`
