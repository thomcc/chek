
/// Panics if the first expression is not strictly not_equal than the second.
///
/// Requires that the values implement `PartialEq` and `Debug`.
///
/// Note: is also aliased as `chek::ne!`. A debug-only version is available as
/// `chek::debug_not_equal!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::not_equal!(4, 1);
/// chek::not_equal!(4, 5, "With a message");
/// chek::not_equal!(4, 2, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! not_equal {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if left == right {
            $crate::__cmp_assert_fail!("not_equal", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if left == right {
            $crate::__cmp_assert_fail!("not_equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Panics if the first expression is not strictly not_equal than the second.
/// Requires that the values implement `PartialEq` and `Debug`.
///
/// Note: is also aliased as `chek::not_equal!`. A debug-only version is available as
/// `chek::debug_ne!`.
///
/// Optionally may take an additional message to display on failure, which is
/// formatted using standard format syntax.
///
/// # Example
///
/// ```rust
/// chek::ne!(4, 1);
/// chek::ne!(
///     "bar",
///     "foo",
/// );
/// chek::ne!(4, 5, "With a message");
/// chek::ne!(4, 2, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! ne {
    ($left:expr, $right:expr $(,)?) => ({
        let (left, right) = (&($left), &($right));
        if left == right {
            $crate::__cmp_assert_fail!("ne", left, right, stringify!($left), stringify!($right));
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        let (left, right) = (&($left), &($right));
        if left == right {
            $crate::__cmp_assert_fail!("ne", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
        }
    })
}

/// Same as `chek::not_equal!` in debug builds or release builds where the `-C
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
/// chek::debug_not_equal!(4, 1);
/// chek::debug_not_equal!(
///     "bar",
///     "foo",
/// );
/// chek::debug_not_equal!(4, 5, "With a message");
/// chek::debug_not_equal!(4, 2, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_not_equal {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if left == right {
                $crate::__cmp_assert_fail!("debug_not_equal", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if left == right {
                $crate::__cmp_assert_fail!("debug_not_equal", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

/// Same as `chek::ne!` in debug builds or release builds where the `-C
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
/// chek::debug_ne!(4, 1);
/// chek::debug_ne!(
///     "bar",
///     "foo",
/// );
/// chek::debug_ne!(4, 5, "With a message");
/// chek::debug_ne!(4, 2, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! debug_ne {
    ($left:expr, $right:expr $(,)?) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if left == right {
                $crate::__cmp_assert_fail!("debug_ne", left, right, stringify!($left), stringify!($right));
            }
        }
    });
    ($left:expr, $right:expr, $($msg_args:tt)+) => ({
        if cfg!(debug_assertions) {
            let (left, right) = (&($left), &($right));
            if left == right {
                $crate::__cmp_assert_fail!("debug_ne", left, right, stringify!($left), stringify!($right), format_args!($($msg_args)+))
            }
        }
    })
}

