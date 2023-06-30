// #![warn(missing_docs)]
//! Tk bindings for Rust.
//!
//! Provides Rustacean friendly bindings to the Tk GUI toolkit.
//! The goal of this API is to provide an idiomatic interface for Tk
//! that allows for little-to-no knowledge of Tcl/Tk itself in order
//! to be used.
//!
//! This crate is in early and active development.
//!
//! # Basic Usage
//! ```
//! fn main() {
//!     todo!("Hello world label, press me button, quit button");
//! }
//! ```

pub mod base;
pub use base::*;

pub mod widgets;
pub use widgets::*;
