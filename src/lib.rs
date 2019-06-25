
//! This is essentially a set of more fully-featured assert macros along the lines
//! of `assert_eq!`, `assert_ne!`, etc. It provides better output than just
//! performing a comparison in the `assert!(...)` body, in that it includes the
//! values of the arguments on output.  Unlike `assert_eq!`/`assert_ne!`, it also
//! will output the expression strings that produced the value.

#![deny(missing_docs)]

#![no_std]

// Need to access these from inside macros.
#[doc(hidden)]
pub use almost;

#[doc(hidden)]
#[macro_use]
pub mod __internal;

// These are separate modules for easier copy/paste...
// TODO: can we remove duplication without making the docs worse?

#[macro_use]
mod lt;
#[macro_use]
mod gt;
#[macro_use]
mod le;
#[macro_use]
mod ge;
#[macro_use]
mod eq;
#[macro_use]
mod ne;
#[macro_use]
mod almost_eq;
#[macro_use]
mod almost_zero;
#[macro_use]
mod not_almost_zero;
#[macro_use]
mod unreachable;
