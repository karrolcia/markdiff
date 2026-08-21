# Decisions — markdiff

Append-only, newest first.

### D-001 · 2026-08-21 · Per-path runtime fs grant for Finder-opened files; static scope stays `**`
**Why:** Finder/argv-delivered paths get `fs_scope().allow_file(path)` at delivery time (escaped-literal pattern), which is the only mechanism that matches dot-directory components (`~/.claude/`, `.github/`) under Tauri's `require_literal_leading_dot: true`. The static capability scope stays at the original `{"path": "**"}` — it is load-bearing: `openFolder` uses a non-recursive dialog grant, so nested folder browsing falls through to the static scope. Narrowing it without adding `recursive: true` to the openFolder dialog call breaks folder browse.
**Instead of:** Widening/rewriting the static scope (`/**` + `$HOME/**` was tried and reverted — the old `**` already matched absolute paths, and no wildcard matches dot-components anyway); or narrowing the static scope (breaks folder browse as above).
**Status:** active
**Where:** commit 018fcdb; review notes in the 2026-08-21 session.

### D-002 · 2026-08-21 · Pending-file slot + frontend_ready flag in ONE mutex, not an AtomicBool
**Why:** File-open handoff is a slot (`Option<String>`) drained by `get_pending_file` after the frontend awaits its `open-file` listener registration. The ready flag lives inside the same mutex as the slot: a separate atomic leaves a TOCTOU where a delivery racing the drain re-parks a path after drain, resurrecting the stale-replay bug the flag exists to prevent. Invariant: after first drain, the slot is and stays empty; warm deliveries emit only. Lock is never held across the emit.
**Instead of:** sleep-then-emit (the original 300ms race — `RunEvent::Opened` fires before the webview exists, so it could never be reliable); separate AtomicBool (TOCTOU above).
**Status:** active
**Where:** commit 018fcdb, `src-tauri/src/main.rs`.
