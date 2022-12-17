//! Generated bindings for the _mirabel_ and _surena_ plugin APIs.
//!
//! This crate also provides some helpers.
//!
//! # Features
//! - `surena`: Generate bindings for _surena_.
//! - `mirabel`: Generate bindings also for _mirabel_.

#[cfg(feature = "surena")]
pub mod sys;

#[cfg(feature = "surena")]
pub mod error;

#[cfg(feature = "surena")]
pub mod game_init;

#[cfg(feature = "mirabel")]
pub mod event;

#[cfg(feature = "mirabel")]
pub mod imgui;

#[cfg(feature = "mirabel")]
pub mod log;

pub mod string;

pub use string::*;

/// Simple macro for counting the number of provided arguments.
///
/// # Example
/// ```
/// # use mirabel_sys::count;
/// assert_eq!(3, count!(1, "AB", true));
/// ```
#[macro_export]
macro_rules! count {
    () => { 0 };
    ($_e: expr $(, $rest: expr)*) => { 1 + $crate::count!($($rest),*) }
}
