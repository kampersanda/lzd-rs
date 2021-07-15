#[macro_export]
#[allow(unused_macros)]
#[cfg(feature = "debug_print")]
macro_rules! debug_print {
    () => {};
    ( $val:expr $(,)? ) => {
        dbg!($val);
    };
    ( $(val:expr),+ $(,)? ) => {
        $( dbg!($val); ),+
    };
}

#[macro_export]
#[allow(unused_macros)]
#[cfg(not(feature = "debug_print"))]
macro_rules! debug_print {
    () => {};
    ( $val:expr $(,)? ) => {};
    ( $(val:expr),+ $(,)? ) => {};
}
