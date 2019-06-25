
/// Panics if the first expression is not strictly equal than the second.
///
/// Requires that the values implement `PartialEq` and `Debug`.
///
/// Note: is also aliased as `chek::eq!`. A debug-only version is available as
/// `chek::debug_equal!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::equal!(4, 4);
/// chek::equal!(4, 4, "With a message");
/// chek::equal!(4, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! equal {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left == right) {
            $crate::__cmp_assert_fail!("equal", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left == right) {
            $crate::__cmp_assert_fail!("equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Panics if the first expression is not strictly equal than the second.
///
/// Requires that the values implement `PartialEq` and `Debug`.
///
/// Note: is also aliased as `chek::equal!`. A debug-only version is available as
/// `chek::debug_eq!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::eq!(4, 4);
/// chek::eq!(4, 4, "With a message");
/// chek::eq!(4, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! eq {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if !(left == right) {
            $crate::__cmp_assert_fail!("eq", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if !(left == right) {
            $crate::__cmp_assert_fail!("eq", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::equal!` in debug builds or release builds where the `-C
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
/// chek::debug_equal!(4, 4);
/// chek::debug_equal!(4, 4, "With a message");
/// chek::debug_equal!(4, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_equal {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left == right) {
                $crate::__cmp_assert_fail!("debug_equal", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left == right) {
                $crate::__cmp_assert_fail!("debug_equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

/// Same as `chek::eq!` in debug builds or release builds where the `-C
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
/// chek::debug_eq!(4, 4);
/// chek::debug_eq!(4, 4, "With a message");
/// chek::debug_eq!(4, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_eq {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left == right) {
                $crate::__cmp_assert_fail!("debug_eq", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if !(left == right) {
                $crate::__cmp_assert_fail!("debug_eq", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

