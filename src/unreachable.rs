
/// Panics if reached. This is a variant of the standard library's `unreachable!`
/// macro that is controlled by `cfg!(debug_assertions)`.
///
/// Same as prelude's `unreachable!` in debug builds or release builds where the
/// `-C debug-assertions` was provided to the compiler. For all other builds,
/// vanishes without a trace.
///
/// # Example
///
/// ```rust
/// // You probably wouldn't actually use this here
/// let mut value = 0.5;
/// if value < 0.0 {
///     chek::debug_unreachable!("Value out of range {}", value);
///     value = 0.0;
/// }
/// ```
#[macro_export]
macro_rules! debug_unreachable {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            unreachable!($($arg)*);
        }
    }
}

/// In debug mode, panics if reached (with `unreachable!`). In release mode, is
/// a `std::hint::unreachable_unchecked()` call. This is `unsafe` to call in
/// both debug and release builds.
///
/// # Example
///
/// ```rust
/// let value = Some(10);
/// // Obviously, be extremely sure you're correct if you use this.
/// let contents = value.unwrap_or_else(|| unsafe {
///     chek::debug_unreachable_unchecked!("optional message")
/// });
/// ```
#[macro_export]
macro_rules! debug_unreachable_unchecked {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            unreachable!($($arg)*);
        } else {
            $crate::__internal::unreachable_unchecked();
        }
    }
}
