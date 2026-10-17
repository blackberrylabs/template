# Blackberrylabs module template

The starting point of a new [Blackberrylabs](https://github.com/blackberrylabs) tool: a Rust
workspace that already keeps the family's promises, already speaks the control tower's
contract, and already packages as `.deb`, `.rpm` and AppImage. Clone it, give it a name,
replace the read, and you have a module.

```sh
git clone https://github.com/blackberrylabs/template newname && cd newname
./rename.sh newname "Newname"        # every mymodule / Mymodule becomes your name
rm rename.sh
cargo test                           # green before you touched anything
```

## What is in the box

| | |
|---|---|
| `crates/core` | `model.rs` (the module's findings, each kind with its slug and its place on the four common severities), `scan.rs` (the read — a placeholder to replace), `report.rs` (the `bbl-report/1` document for the tower, built from what the CLI and the app already show). `tests/promises.rs`: the source never writes nor reaches the network, a scan writes nothing, the report conforms. |
| `crates/cli` | `newname` (the list), `newname report` (for the tower, always JSON), `--json`, `--home`. |
| `crates/app` | `newname-app` on [`bbl-ui`](https://github.com/blackberrylabs/bbl-ui): KPI cards, list, detail, footer with EN·FR and theme; `--select <finding id>` so the tower can open it at a finding; preferences in `~/.config/newname/settings.json`. |
| `locales/` | `en.json`, `fr.json`, kept in step by a test. English is the default. |
| `assets/`, `packaging/` | Desktop entry, icon in the family's style (replace the shape, keep the tile), `build.sh` for the three packages, `.deb` / `.rpm` metadata in `crates/app/Cargo.toml`. |
| `BRIEF.md`, `PLAN.md`, `JOURNAL.md`, `CLAUDE.md` | The pilotage files, in French, with the house rules already written. The brief is validated before the code. |

If the module reads the files of AI hosts (settings, MCP configurations), add
[`bbl-hostrules`](https://github.com/blackberrylabs/bbl-hostrules): locations per host with
their sources, readers for JSON / JSONC / TOML / YAML, secret masking.

## The three obligations of a module

1. **`newname report` prints a `bbl-report/1` document** — `crates/core/src/report.rs`,
   checked by `tests/promises.rs`. Nothing in it that the CLI or the app do not already show.
2. **Each kind of finding has its place on the four common severities** — act now · review ·
   good to know · information — in `Kind::severity`, and the mapping is written in the
   CHANGELOG whenever it changes. The tower never reinterprets.
3. **The app accepts `--select <finding id>`** so the tower opens it at the right place.

Then one declarative sheet in the tower's `modules/` folder, and the module is part of the
family. The full walk-through is
[`docs/new-module.md`](https://github.com/blackberrylabs/blackberrylabs/blob/main/docs/new-module.md)
in the tower.

## The promises, already kept here

Read-only (nothing written outside the app's preferences); no network; secrets masked the
moment they are read; actions shown, never run; every rule sourced. `#![forbid(unsafe_code)]`.

## License

MIT — Alexandre Do.
