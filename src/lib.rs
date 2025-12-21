//! # `rust_engine`
//!
//! ## Example
//!
//! ```
//! use rust_engine::SystemInfo;
//!
//! let info = SystemInfo::new();
//! println!("{}", info.os);
//! ```

mod error;
mod system;

pub use error::SystemError;
pub use system::SystemInfo;
