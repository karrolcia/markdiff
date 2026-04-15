# Markdiff

A native macOS markdown reader and editor with built-in diff tracking. Designed for the AI-assisted writing loop: paste markdown from Claude, edit it, copy the diff back.

![Markdiff](icon.svg)

## What it does

- **Read, edit, and diff** markdown files in three tabbed views
- **Side-by-side folder browser** — open a directory, click through files in a tree
- **Diff against original** — see exactly what you changed since opening the file, copy the diff in a format ready to paste back to Claude
- **Registers as `.md` handler** — double-click any markdown file in Finder to open it in Markdiff
- **CLI integration** — `open -a Markdiff file.md` works from scripts and launchd jobs
- **~5MB binary** — uses macOS WebKit, no Electron runtime

## Install

Download the `.dmg` from [Releases](https://github.com/karrolcia/markdiff/releases) and drag Markdiff to `/Applications`.

## Usage

- `Cmd+O` — open file
- `Cmd+N` — new file
- `Cmd+S` — save (triggers Save As if untitled)
- `Cmd+B` — toggle sidebar
- `Cmd+1/2/3` — switch between Read, Edit, Diff views

Click **Paste** to drop markdown from your clipboard straight into the reader. Edit it, switch to the Diff tab, click **Diff** to copy a git-style patch ready for Claude.

## Build from source

Requires [Rust](https://rustup.rs) and [Node.js](https://nodejs.org).

```bash
git clone https://github.com/karrolcia/markdiff.git
cd markdiff
npm install
npm run dev    # dev mode with live reload
npm run build  # produces .dmg in src-tauri/target/release/bundle/dmg/
```

First build downloads and compiles ~478 Rust crates and takes 5–10 minutes. Subsequent builds are incremental and finish in seconds.

## Architecture

- **Frontend**: single-file `index.html` with vanilla JS (~850 lines), `marked` for rendering, `diff` for change tracking
- **Shell**: [Tauri 2](https://tauri.app) — Rust backend, WebKit frontend, native dialogs and FS via plugins
- **No bundler, no build step for the frontend** — `withGlobalTauri: true` exposes `window.__TAURI__` directly

## License

MIT — see [LICENSE](LICENSE). Use it, fork it, ship your own version.
