#[cold]
#[inline(never)]
#[doc(hidden)]
#[cfg(not(feature = "inline_panics"))]
pub fn cmp_assert_fail<A, B>(which: &str, left: A, right: B, left_str: &str, right_str: &str) -> !
where
    A: core::fmt::Debug,
    B: core::fmt::Debug,
{
    panic!("assertion failed: `chek::{}!(left, right)`\n  left: `{:?}` = `{}`,\n right: `{:?}` = `{}`", which, left, left_str, right, right_str);
}

#[cold]
#[inline(never)]
#[doc(hidden)]
#[cfg(not(feature = "inline_panics"))]
pub fn cmp_assert_fail_msg<A, B>(which: &str, left: &A, right: &B, left_str: &str, right_str: &str, msg: core::fmt::Arguments<'_>) -> !
where
    A: core::fmt::Debug,
    B: core::fmt::Debug,
{
    panic!("assertion failed: `chek::{}!(left, right): {}`\n  left: `{:?}` = `{}`,\n right: `{:?}` = `{}`", which, msg, left, left_str, right, right_str);
}

// Re-export so that we don't have to know which of `core`/`std` is available in the macro.
#[doc(hidden)]
pub use core::hint::unreachable_unchecked;

#[cold]
#[inline(never)]
#[doc(hidden)]
#[cfg(not(feature = "inline_panics"))]
pub fn value_assert_fail_msg<A>(which: &str, value: &A, value_str: &str, msg: core::fmt::Arguments<'_>) -> !
where
    A: core::fmt::Debug,
{
    panic!("assertion failed: `chek::{}!(value): {}`\n value: `{:?}` = `{}`", which, msg, value, value_str);
}

#[cold]
#[inline(never)]
#[doc(hidden)]
#[cfg(not(feature = "inline_panics"))]
pub fn value_assert_fail<A>(which: &str, value: &A, value_str: &str) -> !
where
    A: core::fmt::Debug,
{
    panic!("assertion failed: `chek::{}!(value)`\n value: `{:?}` = `{}`", which, value, value_str);
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "inline_panics"))]
macro_rules! __cmp_assert_fail {
    ($which:expr, $left:expr, $right:expr, $left_str:expr, $right_str:expr) => {
        $crate::__internal::cmp_assert_fail($which, $left, $right, $left_str, $right_str)
    };
    ($which:expr, $left:expr, $right:expr, $left_str:expr, $right_str:expr, $msg:expr) => {
        $crate::__internal::cmp_assert_fail_msg($which, $left, $right, $left_str, $right_str, $msg)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "inline_panics")]
macro_rules! __cmp_assert_fail {
    ($which:expr, $left:expr, $right:expr, $left_str:expr, $right_str:expr) => {
        panic!(concat!("assertion failed: `chek::", $which, "!(left, right)`\n  left: `{:?}` = `{}`,\n right: `{:?}` = `{}`"),
               $left, stringify!($left), $right, stringify!($right))
    };
    ($which:expr, $left:expr, $right:expr, $left_str:expr, $right_str:expr, $msg:expr) => {
        panic!(concat!("assertion failed: `chek::", $which, "!(left, right)`: {}\n  left: `{:?}` = `{}`,\n right: `{:?}` = `{}`"),
               $msg, $left, $left_str, $right, $right_str)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "inline_panics"))]
macro_rules! __value_assert_fail {
    ($which:expr, $value:expr, $value_str:expr) => {
        $crate::__internal::value_assert_fail($which, $value, $value_str)
    };
    ($which:expr, $value:expr, $value_str:expr, $msg:expr) => {
        $crate::__internal::value_assert_fail_msg($which, $value, $value_str, $msg)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "inline_panics")]
macro_rules! __value_assert_fail {
    ($which:expr, $value:expr, $value_str:expr) => {
        panic!(concat!("assertion failed: `chek::", $which, "!(value)`\n value: `{:?}` = `{}`"), $value, $value_str);
    };
    ($which:expr, $value:expr, $value_str:expr, $msg:expr) => {
        panic!(concat!("assertion failed: `chek::", $which, "!(value)`: {}\n value: `{:?}` = `{}`"), $msg, $value, $value_str);
    };
}

