//! The read. Replace this file with what the module actually reads — through `bbl-hostrules`
//! if it reads the files of AI hosts (locations, readers, secret masking are there).

use std::path::Path;

use crate::model::{Finding, Inventory, Kind};

/// Look at `home` and say what was found. This placeholder reads nothing and returns one
/// finding that says so.
pub fn scan(home: &Path) -> Inventory {
    let _ = home;
    Inventory {
        findings: vec![Finding {
            kind: Kind::NothingYet,
            id: "nothing-yet".into(),
            title: "Mymodule does nothing yet".into(),
            subject: "mymodule".into(),
            detail: "replace crates/core/src/scan.rs with the module's own read".into(),
            path: None,
            hint: Some("see docs/new-module.md in the Blackberrylabs tower".into()),
        }],
        files: Vec::new(),
    }
}
