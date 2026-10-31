#![deny(unsafe_op_in_unsafe_fn)]
#[cfg(feature = "ui")]
pub mod app;
pub mod date_calculator;
pub mod engine;
#[cfg(feature = "ui")]
pub mod views;
