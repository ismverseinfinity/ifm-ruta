//! Core traits and interfaces

pub mod async_tool;
pub mod command;
pub mod event;
pub mod process;
pub mod settings;
pub mod tool;
pub mod validation; // NEW for Phase 1: Async architecture

// Re-export all traits
pub use async_tool::*;
pub use command::*;
pub use event::*;
pub use process::*;
pub use settings::*;
pub use tool::*;
pub use validation::*; // NEW for Phase 1
