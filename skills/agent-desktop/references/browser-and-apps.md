# Browser & App Automation

How to drive web browsers and operate desktop apps end-to-end. These are
workflow patterns, not new commands — they compose `snapshot`, `find`, `type`,
`click`, window ops, and `wait`.

## Browsers are just apps

agent-desktop reads a browser's **window accessibility tree** — its *chrome*:
the toolbar, address field, tab strip, bookmarks bar, back/forward buttons. It
does **not** read or drive the rendered web page (links, form fields, and
buttons *inside* the page DOM).

- **In scope:** open a browser, read the address/title, type a URL, switch tabs,
  click toolbar buttons, manage browser windows.
- **Out of scope:** clicking a link on a page, filling a web form, scraping page
  text. For web-page DOM automation use **agent-browser** / a WebDriver tool —
  not this tool.

Safari ships on every Mac; Chrome and others may be absent. Probe at runtime: if
`launch` returns `APP_NOT_FOUND`, skip rather than fail.

### Browser flow

```bash
# 1. Launch (waits for the window)
agent-desktop launch "Safari"

# 2. Snapshot the chrome — interactive only
agent-desktop snapshot --app "Safari" -i
#    Keep the returned snapshot_id. The address field is a textfield/combobox.

# 3. Find the address field and drive it
agent-desktop find --app "Safari" --role textfield --first
agent-desktop type @e7 "example.com" --snapshot <snapshot_id>
agent-desktop press return --app "Safari"

# 4. Navigation depends on the network — wait, don't assume
agent-desktop wait 1000
agent-desktop snapshot --app "Safari" -i   # re-read to confirm new chrome state
```

**Treat navigation as nondeterministic.** Loading hinges on network, timing, and
focus. Drive the address field and press return, then *observe* — don't assume a
specific page loaded. The address bar reflecting the new URL (or the window
title changing) is your signal.

## Operating apps end-to-end

The same observe → act → verify loop applies to any app. Text editors are the
canonical example.

```bash
# Launch and open a fresh document
agent-desktop launch "TextEdit"
agent-desktop press cmd+n --app "TextEdit"
agent-desktop wait 400

# Find the editor. A macOS AXTextArea maps to the `textfield` role, so the
# editor body IS ref-addressable.
agent-desktop snapshot --app "TextEdit" -i           # keep snapshot_id
agent-desktop type @e3 "hello world" --snapshot <snapshot_id>

# Verify the real outcome — re-snapshot and read the value back
agent-desktop get @e3 --snapshot <snapshot_id> --property value
# or re-snapshot and confirm the text is present in the tree
```

### Window lifecycle

```bash
agent-desktop list-windows --app "TextEdit"          # data is the array itself
agent-desktop resize-window --app "TextEdit" --width 640 --height 480
agent-desktop move-window  --app "TextEdit" --x 120 --y 120
agent-desktop minimize --app "TextEdit"
agent-desktop restore  --app "TextEdit"
agent-desktop close-app "TextEdit" --force
```

`list-windows` returns the array of `{ id, title, app_name, pid, bounds, is_focused }`
**as `data` itself** (not nested under `data.windows`). The `--app` filter is an
exact case-insensitive name match.

## Gotchas these patterns depend on

1. **Refs are snapshot-scoped — pass `--snapshot` explicitly.** Every snapshot
   returns a `snapshot_id`. Action commands can fall back to a shared
   *latest-snapshot* pointer, but that pointer is global state: in concurrent or
   multi-agent runs, always pass `--snapshot <id>` (and/or `--session <id>`) so
   one run can't act on another's refs.

2. **The editor role is `textfield`.** `AXTextArea`, `AXSearchField`, and
   `AXSecureTextField` all map to `textfield`, which is interactive and gets a
   ref. If `snapshot -i` shows no editable ref, the field may be inside a sheet
   or not yet focused — open the document first (`press cmd+n`) and re-snapshot.

3. **Headless by default; physical input can be policy-denied.** Ref actions use
   semantic AX paths. Explicit `mouse-*`, `hover`, `drag`, and raw `press`
   synthesis may return `POLICY_DENIED` on a stock setup — that is the policy
   working, not a bug. Prefer refs (`click @e5`) over coordinates.

4. **Some actions aren't supported on some roles.** `set-value`/`click` on a
   text area, or `scroll` on a non-scrollable element, can return
   `ACTION_NOT_SUPPORTED` or `ACTION_FAILED`. Pick the command that matches the
   element's `available_actions`, and re-snapshot after any UI-changing action.

5. **Verify outcomes you can, assert envelopes you can't.** Text insertion and
   window listing are observable — confirm them. Navigation, drags, and key
   synthesis are timing-dependent — confirm the command returned `ok` (or an
   expected error code) and then re-observe state, rather than asserting the side
   effect blind.

## Verification loop

```
LAUNCH  → wait for the window (launch already blocks on it)
SNAPSHOT→ -i, keep snapshot_id
ACT     → type / click / select / window-op  (--snapshot <id>)
OBSERVE → re-snapshot or get; confirm the expected state
REPEAT  → next region or action; close-app when done
```

Drive real apps in a clean session: launching and force-quitting apps is
disruptive, and quitting an app the user already had open can lose unsaved work.
When a tool launches an app it should quit only that instance, leaving
pre-existing apps untouched.
