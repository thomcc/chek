
/// Panics if the first expression is not strictly less than the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::lt!`. A debug-only version is available as
/// `chek::debug_less!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::less!(3, 4);
/// chek::less!(3, 4, "With a message");
/// chek::less!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! less {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left < right) {
            $crate::__cmp_assert_fail!("less", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left < right) {
            $crate::__cmp_assert_fail!("less", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Panics if the first expression is not strictly less than the second.
///
/// Requires that the values implement `PartialOrd` and `Debug`.
///
/// Note: is also aliased as `chek::less!`. A debug-only version is available as
/// `chek::debug_lt!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::lt!(3, 4);
/// chek::lt!(
///     3,
///     4,
/// );
/// chek::lt!(3, 4, "With a message");
/// chek::lt!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! lt {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left < right) {
            $crate::__cmp_assert_fail!("lt", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left < right) {
            $crate::__cmp_assert_fail!("lt", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::less!` in debug builds or release builds where the `-C
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
/// chek::debug_less!(3, 4);
/// chek::debug_less!(
///     3,
///     4,
/// );
/// chek::debug_less!(3, 4, "With a message");
/// chek::debug_less!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_less {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left < right) {
                $crate::__cmp_assert_fail!("debug_less", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left < right) {
                $crate::__cmp_assert_fail!("debug_less", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

/// Same as `chek::lt!` in debug builds or release builds where the `-C
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
/// chek::debug_lt!(3, 4);
/// chek::debug_lt!(
///     3,
///     4,
/// );
/// chek::debug_lt!(3, 4, "With a message");
/// chek::debug_lt!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_lt {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left < right) {
                $crate::__cmp_assert_fail!("debug_lt", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left < right) {
                $crate::__cmp_assert_fail!("debug_lt", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

