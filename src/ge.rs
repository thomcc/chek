
/// Panics if the first expression is not strictly greater_or_equal than the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::ge!`. A debug-only version is available as
/// `chek::debug_greater_or_equal!`.
///
/// On failure, panics and prints the values out in a manner similar to
/// prelude's `assert_eq!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::greater_or_equal!(4, 3);
/// chek::greater_or_equal!(4, 4);
/// chek::greater_or_equal!(4, 3, "With a message");
/// chek::greater_or_equal!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! greater_or_equal {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left >= right) {
            $crate::__cmp_assert_fail!("greater_or_equal", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left >= right) {
            $crate::__cmp_assert_fail!("greater_or_equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Panics if the first expression is not strictly greater_or_equal than the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::greater_or_equal!`. A debug-only version is available as
/// `chek::debug_ge!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::ge!(4, 3);
/// chek::ge!(4, 4);
/// chek::ge!(4, 3, "With a message");
/// chek::ge!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! ge {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left >= right) {
            $crate::__cmp_assert_fail!("ge", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left >= right) {
            $crate::__cmp_assert_fail!("ge", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::greater_or_equal!` in debug builds or release builds where the `-C
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
/// chek::debug_greater_or_equal!(4, 3);
/// chek::debug_greater_or_equal!(4, 4);
/// chek::debug_greater_or_equal!(4, 3, "With a message");
/// chek::debug_greater_or_equal!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_greater_or_equal {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left >= right) {
                $crate::__cmp_assert_fail!("debug_greater_or_equal", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left >= right) {
                $crate::__cmp_assert_fail!("debug_greater_or_equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

/// Same as `chek::ge!` in debug builds or release builds where the `-C
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
/// chek::debug_ge!(4, 3);
/// chek::debug_ge!(4, 4);
/// chek::debug_ge!(4, 3, "With a message");
/// chek::debug_ge!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_ge {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left >= right) {
                $crate::__cmp_assert_fail!("debug_ge", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left >= right) {
                $crate::__cmp_assert_fail!("debug_ge", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

