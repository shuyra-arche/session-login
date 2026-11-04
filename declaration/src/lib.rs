#![no_std]
//! Standalone Login unit declaration.

pub use unit;

mod build;
mod login;

pub use build::BUILD;
pub use login::PROGRAM;
