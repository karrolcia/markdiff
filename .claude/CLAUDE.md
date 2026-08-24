# Markdiff — project notes

Native macOS markdown reader/editor with diff tracking (Tauri v2, vanilla-JS frontend in `dist/index.html`, Rust shell in `src-tauri/`). Design decisions live in `DECISIONS.md` (D-001…D-004) — read it before touching file-open, stash, fs-scope, or sanitization code.

## Known Gotchas

### Vendored `dist/lib/` assets must be `git add`ed explicitly — a missed one breaks the whole app with a lying error
`git commit -am` stages only tracked files. `purify.min.js` was nearly shipped untracked (caught in review 2026-08-21): the script tag would 404, `DOMPurify` undefined, `render()` throws inside `loadFileFromPath`'s try/catch — so every file open shows "Could not read file" and the symptom points at file reading, not the missing lib. After adding any `dist/lib/` asset, check `git status` for `??` before committing.

### The `editor.value = currentContent` sync must run BEFORE any `switchView` that hides the edit panel
WebKit commits pending autocorrect/IME on blur, which fires an `input` event that copies stale `editor.value` back into `currentContent` — intermittent cross-file corruption. The sync-before-switch ordering in `loadFileFromPath`, `openFileFromTree`, and `pasteFromClipboard` is load-bearing; moving it after the view switch reopens the bug. (Mechanism is a strong hypothesis; the fix is correct by invariant either way — see the comment at the sync in `loadFileFromPath`.)

### fs scope: `**` matches absolute paths, but NOTHING wildcard-matches dot-components
Tauri sets `require_literal_leading_dot: true` on unix, so `~/.claude/x.md` or `repo/.github/x.md` never match any glob scope. Finder/argv-delivered files work via the runtime `fs_scope().allow_file(path)` grant in `deliver_file` (escaped literal — immune to the dot rule). Do NOT narrow the static `**` scope without adding `recursive: true` to the `openFolder` dialog call — nested folder browsing falls through to the static scope (D-001).

### Testing a build means installing it: `/Applications/Markdiff.app` is what Launch Services opens
`tauri build` output lands at `src-tauri/target/release/bundle/macos/Markdiff.app`. Replace the installed copy and `lsregister` it, or every `open`/Finder test exercises the OLD binary. Also: `strings` cannot verify frontend changes made it into the binary — Tauri brotli-compresses `dist/` — use binary-vs-source timestamps.

### Agent e2e verification: `osascript` and `screencapture` are TCC-blocked; use the instrumented-log method
Window-title assertions (`System Events`) fail with -1719 and screenshots fail even unsandboxed (Screen Recording permission). Working method: temporary Rust debug command writing via `std::fs`, drive the app with `open -a`, assert on the log; back up sources first, restore after, and grep the shipped diff for instrumentation before committing.

### macOS never passes opened files via argv
Finder and `open file.md` deliver an Apple Event → `RunEvent::Opened { urls }` (which fires BEFORE the webview exists on cold start — hence the pending-slot + drain design, D-002). `std::env::args()` only sees direct CLI invocation. Any "app opens blank on double-click" report should also check the LaunchServices default handler — this machine had a stale Safari-webapp ghost owning `.md` until 2026-08-21.
