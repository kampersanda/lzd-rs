#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_export]
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
#[cfg(not(feature = "debug_print"))]
macro_rules! debug_print {
    () => {};
    ( $val:expr $(,)? ) => {};
    ( $(val:expr),+ $(,)? ) => {};
}
