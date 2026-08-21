# Decisions — markdiff

Append-only, newest first.

### D-004 · 2026-08-21 · XSS defense: DOMPurify at the render sink only; attribute escaping for filenames; no CSP yet
**Why:** Hostile markdown executed script in a webview with full-fs read/write. DOMPurify (3.4.14, defaults, vendored UMD — pinned as a devDependency for provenance, unlike the unpinned marked/diff vendors) wraps the single display sink so raw markdown round-trips untouched through editor/stash/diff/save. Filename sinks are a different class: `escapeHtml` never escapes quotes, so paths in `data-path="…"` needed a dedicated `escapeAttr` (attribute context), not DOMPurify — sanitizing filenames as HTML would mangle legitimate names.
**Instead of:** sanitizing `currentContent` (corrupts source text); CSP in tauri.conf.json (deferred — good defense-in-depth follow-up that would also stop remote-image beaconing; kept out of this change).
**Status:** active. Open follow-ups: `default-src 'self'` CSP; pin marked/diff like dompurify.
**Where:** commit 34d4c7b.

### D-003 · 2026-08-21 · Unsaved edits auto-stash: in-memory, absolute-path-keyed, session-scoped
**Why:** Karolina chose auto-stash over a confirm dialog (interruption + still one-click data loss) and over persistent local history (more build than the job needs). Switching files parks `{original, editedContent}` in a `Map` keyed by absolute path — the only key form that unifies a file reached from the folder tree with the same file opened from Finder (`fileEntries` is relative-path-keyed and wiped by every rescan, so it can't serve). Staleness is content-based: disk differing from the stashed baseline wins and drops the stash. Cleared on save, Save-As, and revert-to-clean.
**Instead of:** confirm dialog; persistent snapshots; reusing `fileEntries` as the store (see above).
**Status:** active. Known in-spec gaps, deliberately not built: `newFile()`/paste still discard edits from a named file (one-line `stashCurrentEdits()` call would close it — scope not chosen); symlinked paths (`/tmp` vs `/private/tmp`) can dual-key a file — degrades to non-restoring, never lossy; quitting the app discards all stashes by design.
**Where:** commit af6f5c8, `dist/index.html`. Note: the `editor.value` sync before any `switchView` in load/paste/tree-open paths guards against WebKit blur/composition-commit input events — ordering is load-bearing, see the comment at the sync in `loadFileFromPath`.

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
