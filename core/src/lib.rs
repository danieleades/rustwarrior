//! Core task management logic and persistence layer for `RustWarrior`

#![deny(
    clippy::all,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::cargo_common_metadata)]

/// Task storage and persistence
pub mod store;
/// Task domain model
pub mod task;

pub use store::{Error as StoreError, OpenTask, Store, paths};
pub use task::{Priority, Task};
