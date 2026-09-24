# Roadmap — markdiff

What's next. Why things are the way they are lives in `DECISIONS.md` (its Status lines also carry deliberately deferred follow-ups: CSP, pinning marked/diff, New/Paste stash gap).

## Later

- [ ] **Resolve Dependabot alert #2 (glib 0.18 unsoundness)** [quick] — either dismiss it on GitHub as "vulnerable code not used" with the reasoning below, or clear it by upgrading Tauri once Tauri moves to gtk-rs ≥ 0.20.
  Helps: infra — keeps the security alert list meaningful, so a real alert on the app Karolina opens her client files in isn't lost among stale ones.
  Why: left open on 2026-09-24 when the other three alerts (tauri, serde_with, rand) were fixed in ad209c2. glib comes in only via Tauri's Linux GTK stack (`muda`/`gtk` → `glib`), is not compiled into the macOS build, and markdiff never calls `glib::VariantStrIter`. Fixed only in glib 0.20, which Tauri 2 does not use.
  Where: `~/GitHub/markdiff/src-tauri/Cargo.lock`; alert at github.com/karrolcia/markdiff/security/dependabot/2.
  Assumes: Tauri 2.x still pins gtk 0.18 (check first: `cargo update` in `src-tauri`, then `grep -A1 '^name = "glib"$' Cargo.lock` — if it shows ≥ 0.20, the alert closes itself on push).
  Done: alert #2 is closed on GitHub, either as fixed (lockfile shows glib ≥ 0.20, release build passes, the app still opens and saves a file) or dismissed with the reason recorded.
  Close: commit to main if the lockfile changed; otherwise the dismissal reason on GitHub.

### Watch list from 2026-09-24 session (sidebar tree fixes + Tauri 2.11.6)

- [ ] **Smoke-test the paths the probe didn't cover, on the installed build.** Double-click a `.md` in Finder (Apple Event → `RunEvent::Opened`, D-002), Open Folder dialog, Save As dialog. tao 0.34 → 0.35 and wry 0.54 → 0.55 came in with the Tauri bump (ad209c2); only `readTextFile`/`writeTextFile` IPC was verified.
- [ ] **Open a very large folder (e.g. `~`) once and judge the scan time.** `walkDir` is now uncapped and reads sibling folders concurrently (D-005). `~/Library` is not a dot-folder, so it gets walked. If it's slow, add `Library` to `SKIP_DIRS` or a total-entry budget with a toast.

### Assumptions made in 2026-09-24 session (verify before any work that depends on them)

- [ ] **Tauri 2.11's IPC origin check doesn't affect the dialog plugin or the `get_pending_file` handoff.** Assumed because fs-plugin IPC worked under 2.11.6. If false, Open Folder or Finder-open breaks silently. Verify with the first watch item.
- [ ] **Hiding folders with no md/txt file, and the SKIP_DIRS list (which leaves `dist`/`build` walked), match how she browses.** Taste calls in D-005, not confirmed with her. If false, a folder she expects to see (e.g. one holding only PDFs) is missing from the sidebar. Verify by asking once she has used the new sidebar.
