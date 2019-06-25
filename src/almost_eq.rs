
/// Panics if the provided values are not almost equal to eachother.
///
/// Uses the [`almost` crate](https://crates.io/crates/almost).
/// See [the `almost::equal` documentation](https://docs.rs/almost/%2a/almost/fn.equal.html)
/// for more details.
///
/// A debug-only version is available as `chek::debug_almost_equal!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::almost_equal!(4.0f32, 4.000001f32);
/// chek::almost_equal!(4.0, 4.0, "should be equal");
/// ```
#[macro_export]
macro_rules! almost_equal {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = ($left, $right);
        if !$crate::almost::equal(left, right) {
            $crate::__cmp_assert_fail!("almost_equal", &left, &right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = ($left, $right);
        if !$crate::almost::equal(left, right) {
            $crate::__cmp_assert_fail!("almost_equal", &left, &right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::almost_equal!` in debug builds or release builds where the `-C
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
/// chek::debug_almost_equal!(1.0f32, 1.0f32 + std::f32::EPSILON);
/// chek::debug_almost_equal!(3.0, 3.000005f32, "Example message");
/// ```
#[macro_export]
macro_rules! debug_almost_equal {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = ($left, $right);
            if !$crate::almost::equal(left, right) {
                $crate::__cmp_assert_fail!("debug_almost_equal", &left, &right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = ($left, $right);
            if !$crate::almost::equal(left, right) {
                $crate::__cmp_assert_fail!("debug_almost_equal", &left, &right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

// Note: The 'with_tolerance' versions are hard to use, so not provided for now...

/// Panics if the provided values are almost equal to eachother.
///
/// Uses the [`almost` crate](https://crates.io/crates/almost).
/// See [the `almost::equal` documentation](https://docs.rs/almost/%2a/almost/fn.equal.html)
/// for more details.
///
/// A debug-only version is available as `chek::debug_not_almost_equal!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::not_almost_equal!(4.0, 5.0);
/// chek::not_almost_equal!(4.0, 5.0, "shouldn't be equal");
/// ```
#[macro_export]
macro_rules! not_almost_equal {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = ($left, $right);
        if $crate::almost::equal(left, right) {
            $crate::__cmp_assert_fail!("not_almost_equal", &left, &right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = ($left, $right);
        if $crate::almost::equal(left, right) {
            $crate::__cmp_assert_fail!("not_almost_equal", &left, &right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::not_almost_equal!` in debug builds or release builds where
/// the `-C debug-assertions` was provided to the compiler. For all other
/// builds, vanishes without a trace.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::debug_not_almost_equal!(4.0, 5.0);
/// chek::debug_not_almost_equal!(4.0, 5.0, "shouldn't be equal");
/// ```
#[macro_export]
macro_rules! debug_not_almost_equal {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = ($left, $right);
            if $crate::almost::equal(left, right) {
                $crate::__cmp_assert_fail!("debug_not_almost_equal", &left, &right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = ($left, $right);
            if $crate::almost::equal(left, right) {
                $crate::__cmp_assert_fail!("debug_not_almost_equal", &left, &right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}
