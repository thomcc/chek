
/// Panics if the first expression is not less than or equal to the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::le!`. A debug-only version is available as
/// `chek::debug_less_or_equal!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::less_or_equal!(3, 4);
/// chek::less_or_equal!(4, 4);
/// chek::less_or_equal!(3, 4, "With a message");
/// chek::less_or_equal!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! less_or_equal {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left <= right) {
            $crate::__cmp_assert_fail!("less_or_equal", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left <= right) {
            $crate::__cmp_assert_fail!("less_or_equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Panics if the first expression is not less than or equal to the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::less_or_equal!`. A debug-only version is
/// available as `chek::debug_le!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::le!(3, 4);
/// chek::le!(4, 4);
/// chek::le!(3, 4, "With a message");
/// chek::le!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! le {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left <= right) {
            $crate::__cmp_assert_fail!("le", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left <= right) {
            $crate::__cmp_assert_fail!("le", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::less_or_equal!` in debug builds or release builds where the `-C
/// debug-assertions` was provided to the compiler. For all other builds,
/// vanishes without a trace.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// // These are no-ops if debug_assertions are off!
/// chek::debug_less_or_equal!(3, 4);
/// chek::debug_less_or_equal!(4, 4);
/// chek::debug_less_or_equal!(3, 4, "With a message");
/// chek::debug_less_or_equal!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_less_or_equal {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left <= right) {
                $crate::__cmp_assert_fail!("debug_less_or_equal", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left <= right) {
                $crate::__cmp_assert_fail!("debug_less_or_equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

/// Same as `chek::le!` in debug builds or release builds where the `-C
/// debug-assertions` was provided to the compiler. For all other builds,
/// vanishes without a trace.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// // These are no-ops if debug_assertions are off!
/// chek::debug_le!(3, 4);
/// chek::debug_le!(4, 4);
/// chek::debug_le!(3, 4, "With a message");
/// chek::debug_le!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_le {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left <= right) {
                $crate::__cmp_assert_fail!("debug_le", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left <= right) {
                $crate::__cmp_assert_fail!("debug_le", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

