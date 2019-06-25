
/// Panics if the provided value is not almost equal to zero.
///
/// Uses the [`almost` crate](https://crates.io/crates/almost).
/// See [the `almost::zero` documentation](https://docs.rs/almost/%2a/almost/fn.zero.html)
/// for more details.
///
/// A debug-only version is available as `chek::debug_almost_zero!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::almost_zero!(0.00000001);
/// chek::almost_zero!(std::f32::EPSILON, "Should be almost zero!");
/// ```
#[macro_export]
macro_rules! almost_zero {
    ($value:expr $(,)?) => ({
        let value = $value;
        if !$crate::almost::zero(value) {
            $crate::__value_assert_fail!("almost_zero", &value, stringify!($value));
        }
    });
    ($value:expr, $($msg_args:tt)+) => ({
        let value = $value;
        if !$crate::almost::zero(value) {
            $crate::__value_assert_fail!("almost_zero", &value, stringify!($value), format_args!($($msg_args)+));
        }
    });
}

/// Same as `chek::almost_zero!` in debug builds or release builds where the `-C
/// debug-assertions` was provided to the compiler. For all other builds,
/// vanishes without a trace.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// // These are compiled to nothing if debug_assertions are off!
/// chek::debug_almost_zero!(0.00000001);
/// chek::debug_almost_zero!(std::f32::EPSILON, "Should be almost zero!");
/// ```
#[macro_export]
macro_rules! debug_almost_zero {
    ($value:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let value = $value;
            if !$crate::almost::zero(value) {
                $crate::__value_assert_fail!("debug_almost_zero", &value, stringify!($value));
            }
        }
    });
    ($value:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let value = $value;
            if !$crate::almost::zero(value) {
                $crate::__value_assert_fail!("debug_almost_zero", &value, stringify!($value), format_args!($($msg_args)+));
            }
        }
    });
}

/// Panics if the provided value is not almost equal to zero.
///
/// Uses the [`almost` crate](https://crates.io/crates/almost). See the
/// [`almost::zero_with_tolerance`](https://docs.rs/almost/%2a/almost/fn.zero_with_tolerance.html)
/// documentation for more details.
///
/// A debug-only version is available as `chek::debug_almost_zero_with_tolerance!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::almost_zero_with_tolerance!(0.00000001, 0.01);
/// chek::almost_zero_with_tolerance!(std::f32::EPSILON, std::f32::EPSILON * 2.0, "Should be almost zero!");
/// ```
#[macro_export]
macro_rules! almost_zero_with_tolerance {
    ($value:expr, $tolerance:expr $(,)?) => ({
        let (value, tolerance) = ($value, $tolerance);
        if !$crate::almost::zero_with_tolerance(value, tolerance) {
            $crate::__value_assert_fail!("almost_zero_with_tolerance", &value, stringify!($value));
        }
    });
    ($value:expr, $tolerance:expr, $($msg_args:tt)+) => ({
        let (value, tolerance) = ($value, $tolerance);
        if !$crate::almost::zero_with_tolerance(value, tolerance) {
            $crate::__value_assert_fail!("almost_zero_with_tolerance", &value, stringify!($value), format_args!($($msg_args)+));
        }
    });
}

/// Same as `chek::almost_zero_with_tolerance!` in debug builds or release
/// builds where the `-C debug-assertions` was provided to the compiler. For all
/// other builds, vanishes without a trace.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// // These are compiled to nothing if debug_assertions are off!
/// chek::debug_almost_zero_with_tolerance!(0.00000001, 0.01);
/// chek::debug_almost_zero_with_tolerance!(std::f32::EPSILON, std::f32::EPSILON * 2.0, "Should be almost zero!");
/// ```
#[macro_export]
macro_rules! debug_almost_zero_with_tolerance {
    ($value:expr, $tolerance:expr $(,)?) => ({
        let (value, tolerance) = ($value, $tolerance);
        if !$crate::almost::zero_with_tolerance(value, tolerance) {
            $crate::__value_assert_fail!("debug_almost_zero_with_tolerance", &value, stringify!($value));
        }
    });
    ($value:expr, $tolerance:expr, $($msg_args:tt)+) => ({
        let (value, tolerance) = ($value, $tolerance);
        if !$crate::almost::zero_with_tolerance(value, tolerance) {
            $crate::__value_assert_fail!("debug_almost_zero_with_tolerance", &value, stringify!($value), format_args!($($msg_args)+));
        }
    });
}


