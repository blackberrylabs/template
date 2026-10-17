//! Mymodule command line: `mymodule` (the list), `mymodule report` (for the Blackberrylabs
//! control tower, always JSON), `--json` for scripts.

use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use mymodule_core::{scan, Inventory};

#[derive(Parser)]
#[command(name = "mymodule", version, about = "One sentence: what Mymodule answers. Read-only.")]
struct Cli {
    /// Machine-readable output.
    #[arg(long, global = true)]
    json: bool,
    /// Look at another home directory (a backup, a fixture) instead of yours.
    #[arg(long, global = true)]
    home: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// What was found, most serious first (default).
    List,
    /// The inventory as a `bbl-report/1` document for the Blackberrylabs control tower
    /// (always JSON).
    Report,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let home = cli
        .home
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("/"));
    let started = std::time::Instant::now();
    let inv = scan(&home);
    let duration_ms = started.elapsed().as_millis() as u64;
    let mut out = std::io::stdout().lock();
    match cli.command.unwrap_or(Command::List) {
        Command::Report => {
            out.write_all(mymodule_core::report::build(&inv, duration_ms).to_json().as_bytes())?;
        }
        Command::List if cli.json => writeln!(out, "{}", serde_json::to_string_pretty(&inv)?)?,
        Command::List => list(&mut out, &inv)?,
    }
    Ok(())
}

fn list(out: &mut impl Write, inv: &Inventory) -> Result<()> {
    if inv.findings.is_empty() {
        writeln!(out, "No finding.")?;
        return Ok(());
    }
    for f in &inv.findings {
        writeln!(out, "{:<16} {} — {}", f.kind.label(), f.title, f.detail)?;
        if let Some(h) = &f.hint {
            writeln!(out, "{:<16} → {h}", "")?;
        }
    }
    writeln!(out)?;
    writeln!(out, "{} finding(s). Mymodule changes nothing: every action above is yours to take.", inv.findings.len())?;
    Ok(())
}
