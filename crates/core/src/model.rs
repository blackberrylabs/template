//! What the module finds. Replace with the module's own vocabulary; keep `Kind::severity`
//! and `Kind::slug`, the tower's report is built from them.

use std::path::PathBuf;

use serde::Serialize;

/// The kinds of finding this module produces. One slug each, one place on the common scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// Placeholder: the module does nothing yet.
    NothingYet,
}

impl Kind {
    /// The slug, as it appears in `--json` and in the tower's report.
    pub fn slug(self) -> &'static str {
        match self {
            Kind::NothingYet => "nothing-yet",
        }
    }

    /// Where the kind sits on the four common severities (see `docs/new-module.md` in the
    /// tower). Write the mapping in the CHANGELOG when it changes.
    pub fn severity(self) -> bbl_report::Severity {
        match self {
            Kind::NothingYet => bbl_report::Severity::Info,
        }
    }

    /// Label as the CLI prints it.
    pub fn label(self) -> &'static str {
        match self {
            Kind::NothingYet => "NOTHING YET",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub kind: Kind,
    /// Stable across runs for the same thing: the tower uses it to tell a finding already
    /// seen from a new one.
    pub id: String,
    pub title: String,
    pub subject: String,
    pub detail: String,
    pub path: Option<PathBuf>,
    /// What the person could do — shown, never run.
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Inventory {
    pub findings: Vec<Finding>,
    /// Files the module looked at.
    pub files: Vec<PathBuf>,
}
