# [`chek`](https://crates.io/crates/chek): macros for checking things.

[![Docs](https://docs.rs/chek/badge.svg)](https://docs.rs/chek)

This is essentially a set of more fully-featured assert macros along the lines
of `assert_eq!`, `assert_ne!`, etc. It provides better output than just
performing a comparison in the `assert!(...)` body, in that it includes the
values of the arguments on output. Unlike `assert_eq!`/`assert_ne!`, it also
will log the expression strings that produced the value.

It's the successor to my `more-asserts` crate, which is somewhat unpleasant to use
in rust 2018 without a `#[macro_use] extern crate ...`, due to the long module name.

Additionally, it is no_std compatible in all configurations.

## Usage

### Cargo features

#### `inline_panics`
This changes the assertions to panic directly, instead of calling a separate
function which performs the panic. This results in better error reporting in the
panic itself (it will report the correct line number / file instead of reporting
that the panic happened inside the `chek` crate), at the cost of some additional
code bloat.

This is on by default.

### Assertions

The following macros all take optional formatting message args as well, e.g.
`check::lt!(a, b, "it should have been less because of: {}", thing)`. Note that
even if this is provided, `a` and `b` will still be logged, so you don't need to
do that manually.

- `chek::less!(a, b)`: Equivalent to `assert!(a < b)`, but with better output on failure
    - A debug_assertions-only version is available: `chek::debug_less!`.
    - The following aliases are provided: `chek::lt!` and `chek::debug_lt!` for the debug_assertions-only version.

- `chek::less_or_equal!(a, b)`: Equivalent to `assert!(a <= b)`, but with better output on failure
    - A debug_assertions-only version is available: `chek::less_or_equal!`.
    - The following aliases are provided: `chek::le!` and `chek::debug_le!` for the debug_assertions-only version.

- `chek::greater!(a, b)`: Equivalent to `assert!(a > b)`, but with better output on failure
    - A debug_assertions-only version is available: `chek::debug_greater!`.
    - The following aliases are provided: `chek::gt!` and `chek::debug_gt!` for the debug_assertions-only version.

- `chek::greater_or_equal!(a, b)`: Equivalent to `assert!(a >= b)`, but with better output on failure
    - A debug_assertions-only version is available: `chek::debug_greater_or_equal!`.
    - The following aliases are provided: `chek::ge!` and `chek::debug_ge!` for the debug_assertions-only version.

- `chek::equal!(a, b)`: Equivalent to `assert_eq!(a, b)`, but with better output on failure.
    - A debug_assertions-only version is available: `chek::debug_equal!`.
    - The following aliases are provided: `chek::eq!` and `chek::debug_eq!` for the debug_assertions-only version.

- `chek::not_equal!(a, b)`: Equivalent to `assert_ne!(a, b)`, but with better output on failure.
    - A debug_assertions-only version is available: `chek::debug_not_equal!`.
    - The following aliases are provided: `chek::ne!` and `chek::debug_ne!` for the debug_assertions-only version.

- `chek::almost_zero!(a)`: Similar to `assert!(almost::zero(a))`, but with better output on failure.
    - A debug_assertions-only version is available: `chek::debug_almost_zero!`.
    - Uses the [`almost` crate](https://crates.io/crates/almost). See [the `almost::zero` documentation](https://docs.rs/almost/%2a/almost/fn.zero.html) documentation for more details.

- `chek::not_almost_zero!(a)`: Similar to `assert!(!almost::zero(a))`, but with better output on failure.
    - A debug_assertions-only version is available: `chek::debug_not_almost_zero!`.
    - Uses the [`almost` crate](https://crates.io/crates/almost). See [the `almost::zero` documentation](https://docs.rs/almost/%2a/almost/fn.zero.html) documentation for more details.

- `chek::almost_equal!(a, b)`: Equivalent to `assert!(almost::equal(a, b))`, but with better output on failure.
    - **_Important_**: Do not use if `a` or `b` is a hard-coded constant zero! instead, use `chek::almost_zero!(v)`.
    - A debug_assertions-only version is available: `chek::debug_almost_equal!`.
    - Uses the [`almost` crate](https://crates.io/crates/almost). See [the `almost::equal` documentation](https://docs.rs/almost/%2a/almost/fn.equal.html) documentation for more details.

- `chek::not_almost_equal!(a, b)`: Equivalent to `assert!(!almost::equal(a, b))`, but with better output on failure.
    - **_Important_**: Do not use if `a` or `b` is a hard-coded constant zero! instead, use `chek::not_almost_zero!(v)`.
    - A debug_assertions-only version is available: `chek::debug_not_almost_equal!`.
    - Uses the [`almost` crate](https://crates.io/crates/almost). See [the `almost::equal` documentation](https://docs.rs/almost/%2a/almost/fn.equal.html) documentation for more details.

- `chek::almost_zero_with_tolerance!(a)`: Similar to `assert!(almost::zero_with_tolerance(a, b, tol))`, but with better output on failure.
    - A debug_assertions-only version is available: `chek::debug_almost_zero_with_tolerance!`.
    - Uses the [`almost` crate](https://crates.io/crates/almost). See [the `almost::zero_with_tolerance` documentation](https://docs.rs/almost/%2a/almost/fn.zero_with_tolerance.html) documentation for more details.

- `chek::not_almost_zero_with_tolerance!(a)`: Similar to `assert!(!almost::zero_with_tolerance(a, b, tol))`, but with better output on failure.
    - A debug_assertions-only version is available: `chek::debug_not_almost_zero_with_tolerance!`.
    - Uses the [`almost` crate](https://crates.io/crates/almost). See [the `almost::zero_with_tolerance` documentation](https://docs.rs/almost/%2a/almost/fn.zero_with_tolerance.html) documentation for more details.

- `chek::debug_unreachable_unchecked!()`: Unsafe. Similar to [`std::hint::unreachable_unchecked`](https://doc.rust-lang.org/stable/std/hint/fn.unreachable_unchecked.html), but panics in debug builds if it's hit.

- `chek::debug_unreachable!()`: Equivalent to the `unreachable!` macro but replaced with a no-op in release builds.

## License

[CC0 (public domain)](https://creativecommons.org/publicdomain/zero/1.0/).
