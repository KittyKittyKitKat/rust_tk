pub mod error;
pub use error::*;

pub mod interpreter;
pub use interpreter::*;

pub mod registry;
pub use registry::*;

pub mod tk;
pub use tk::*;

pub mod window_manager;
pub use window_manager::*;

pub enum TkOption<T> {
    Get,
    Set(T),
}
