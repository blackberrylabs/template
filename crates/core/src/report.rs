//! The `bbl-report/1` view of an inventory, for the Blackberrylabs control tower. Nothing
//! here is new: only what the CLI and the app already show, secrets masked upstream.

use bbl_report::{Action, ActionKind, Evidence, Finding, Module, Open, Report, Subject, SubjectKind, Summary, Tone};

use crate::model::Inventory;

/// Binary name of the app, as packaged.
pub const APP: &str = "mymodule-app";
/// Binary name of the command line, as packaged.
pub const CLI: &str = "mymodule";

pub fn module() -> Module {
    Module {
        id: "mymodule".into(),
        name: "Mymodule".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        verb: "inspect".into(),
        tagline: "One sentence: what Mymodule answers".into(),
        app: Some(APP.into()),
        cli: Some(CLI.into()),
        home: "https://github.com/blackberrylabs/mymodule".into(),
    }
}

/// Build the report. `duration_ms` is the time the scan took.
pub fn build(inv: &Inventory, duration_ms: u64) -> Report {
    let mut report = Report::new(module());
    report.duration_ms = duration_ms;
    let n = inv.findings.len();
    // 1 to 4 figures: the ones the app shows on its cards.
    report.summary = vec![
        Summary { key: "files".into(), label: "Files read".into(), value: inv.files.len().to_string(), tone: Tone::Neutral },
        Summary { key: "findings".into(), label: "Findings".into(), value: n.to_string(), tone: if n == 0 { Tone::Good } else { Tone::Alert } },
    ];
    for f in &inv.findings {
        let mut actions = Vec::new();
        if let Some(h) = &f.hint {
            actions.push(Action { kind: ActionKind::Hint, label: h.clone(), target: None });
        }
        if let Some(p) = &f.path {
            actions.push(Action { kind: ActionKind::OpenFile, label: "Open file".into(), target: Some(p.display().to_string()) });
        }
        report.findings.push(Finding {
            id: format!("{}/{}", f.kind.slug(), f.id),
            severity: f.kind.severity(),
            kind: f.kind.slug().into(),
            title: f.title.clone(),
            subject: Subject { kind: SubjectKind::Other, label: f.subject.clone() },
            detail: f.detail.clone(),
            evidence: f.path.iter().map(|p| Evidence { label: p.display().to_string(), path: Some(p.display().to_string()), line: None }).collect(),
            actions,
            open: Some(Open { app: APP.into(), args: vec!["--select".into(), f.id.clone()] }),
        });
    }
    report.notes.push(format!("{} file(s) read", inv.files.len()));
    report
}
