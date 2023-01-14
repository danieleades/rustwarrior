//! Command-line task tracking

#![deny(
    clippy::all,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
#![warn(clippy::pedantic, clippy::nursery)]

mod store;
mod task;

pub use store::Store;
pub use task::Task;
