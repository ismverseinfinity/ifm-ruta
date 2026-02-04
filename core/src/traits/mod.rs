//! Core traits and interfaces

pub mod command;
pub mod event;
pub mod process;
pub mod settings;
pub mod tool;
pub mod validation;
pub mod async_tool;  // NEW for Phase 1: Async architecture

// Re-export all traits
pub use command::*;
pub use event::*;
pub use process::*;
pub use settings::*;
pub use tool::*;
pub use validation::*;
pub use async_tool::*;  // NEW for Phase 1
