//! Mymodule, core: what the module reads and what it finds. No UI, no network.
//!
//! The Blackberrylabs promises, the same for every tool: read-only (nothing written outside
//! `~/.config/mymodule/`, and only by the app's preferences); no network; secrets masked the
//! moment they are read; actions shown, never run; every rule sourced.
//!
//! `tests/promises.rs` checks that the source of this crate never mentions a write or a
//! network API, and that `report` conforms to `bbl-report/1`.

#![forbid(unsafe_code)]

pub mod model;
pub mod report;
pub mod scan;

pub use model::{Finding, Inventory, Kind};
pub use scan::scan;
