
/// Panics if the first expression is not strictly greater than the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::gt!`. A debug-only version is available as
/// `chek::debug_greater!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::greater!(4, 3);
/// chek::greater!(4, 3, "With a message");
/// chek::greater!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! greater {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left > right) {
            $crate::__cmp_assert_fail!("greater", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left > right) {
            $crate::__cmp_assert_fail!("greater", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Panics if the first expression is not strictly greater than the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::greater!`. A debug-only version is available as
/// `chek::debug_gt!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::gt!(4, 3);
/// chek::gt!(4, 3, "With a message");
/// chek::gt!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! gt {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left > right) {
            $crate::__cmp_assert_fail!("gt", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left > right) {
            $crate::__cmp_assert_fail!("gt", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::greater!` in debug builds or release builds where the `-C
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
/// chek::debug_greater!(4, 3);
/// chek::debug_greater!(4, 3, "With a message");
/// chek::debug_greater!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_greater {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left > right) {
                $crate::__cmp_assert_fail!("debug_greater", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left > right) {
                $crate::__cmp_assert_fail!("debug_greater", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

/// Same as `chek::gt!` in debug builds or release builds where the `-C
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
/// chek::debug_gt!(4, 3);
/// chek::debug_gt!(4, 3, "With a message");
/// chek::debug_gt!(4, 3, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_gt {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left > right) {
                $crate::__cmp_assert_fail!("debug_gt", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left > right) {
                $crate::__cmp_assert_fail!("debug_gt", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

