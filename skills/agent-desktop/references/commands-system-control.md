# System Control Commands

Non-AX control of macOS hardware and system state — audio volume, Dark/Light
appearance, Wi-Fi power — plus an audited escape hatch for shell/AppleScript/JXA
and `open`. These commands do not touch the accessibility tree; they drive
system services directly.

> **Platform:** macOS only (Phase 1). Each returns the standard JSON envelope
> `{ "version": "2.0", "ok": true, "command": "...", "data": { ... } }`.

## volume

```bash
agent-desktop volume --get
agent-desktop volume --set 40
agent-desktop volume --up --step 10
agent-desktop volume --down
agent-desktop volume --mute
agent-desktop volume --unmute
```

Reads or changes the system output volume. Exactly one of
`--get` / `--set` / `--up` / `--down` / `--mute` / `--unmute` is required.
**Every form returns the post-change state** `{ output_volume, muted }`, so a
`--set 40` confirms the new level without a follow-up `--get`.

| Flag | Default | Description |
|------|---------|-------------|
| `--get` | — | Print current `{ output_volume, muted }` |
| `--set N` | — | Set output volume, `0..=100` (`INVALID_ARGS` if out of range) |
| `--up` / `--down` | — | Raise/lower by `--step` |
| `--mute` / `--unmute` | — | Set the mute flag |
| `--step` | 5 | Step size for `--up` / `--down` |

`data`: `{ "output_volume": 40, "muted": false }`

## appearance

```bash
agent-desktop appearance --get
agent-desktop appearance --dark
agent-desktop appearance --light
agent-desktop appearance --toggle
```

Reads or sets the system Dark/Light appearance. Exactly one of
`--get` / `--dark` / `--light` / `--toggle` is required. All forms return the
resulting state `{ dark }`.

`data`: `{ "dark": true }`

## wifi

```bash
agent-desktop wifi --status
agent-desktop wifi --on
agent-desktop wifi --off
```

Reads or toggles Wi-Fi power. Exactly one of `--status` / `--on` / `--off` is
required. Returns `{ wifi_power }`, plus `ssid` when associated and known.

`data`: `{ "wifi_power": true, "ssid": "home-network" }`

## The Escape Hatch (run-shell / run-applescript / run-jxa / open-url / open-path)

These five commands run arbitrary code or hand a target to the system opener.
They are **disabled by default** and gated behind an environment variable so an
agent can never shell out implicitly.

### Enabling

Set `AGENT_DESKTOP_ENABLE_EXEC=1` in the environment. Without it the command
returns:

```json
{ "ok": false, "error": { "code": "POLICY_DENIED",
  "suggestion": "Set AGENT_DESKTOP_ENABLE_EXEC=1 to enable run-shell/run-applescript/run-jxa/open-url/open-path" } }
```

Every successful invocation appends one line to `~/.agent-desktop/exec_audit.log`
recording the kind and payload — treat this as a tamper-evident trail, not a
secret store.

### run-shell / run-applescript / run-jxa

```bash
AGENT_DESKTOP_ENABLE_EXEC=1 agent-desktop run-shell "ls -la ~"
AGENT_DESKTOP_ENABLE_EXEC=1 agent-desktop run-applescript 'tell app "Finder" to get name of home'
AGENT_DESKTOP_ENABLE_EXEC=1 agent-desktop run-jxa 'Application("Finder").name()'
```

Runs a shell command, AppleScript, or JXA script body. The child is killed after
`--timeout` ms (default **30000**). Output is drained without blocking, so a
chatty command cannot deadlock the pipe.

| Flag | Default | Description |
|------|---------|-------------|
| `--timeout` | 30000 | Kill the child after this many milliseconds |

`data`: `{ "exit_code": 0, "stdout": "...", "stderr": "", "duration_ms": 12 }`

### open-url / open-path

```bash
AGENT_DESKTOP_ENABLE_EXEC=1 agent-desktop open-url "https://example.com"
AGENT_DESKTOP_ENABLE_EXEC=1 agent-desktop open-path "~/Documents/report.pdf"
```

Hands a URL or filesystem path to the default system handler (`open`). Same env
gate, audit log, and `{ exit_code, stdout, stderr, duration_ms }` result shape as
the `run-*` commands.

## When to reach for System Control

- **Prefer AX commands first.** If a setting is reachable through an app's UI
  (e.g. a toggle in System Settings), driving it with `snapshot` + `toggle`
  keeps the run observable and auditable through refs.
- **Use these for things AX can't express:** reading/setting raw output volume,
  flipping appearance system-wide, Wi-Fi power, or one-shot `open` of a URL/file.
- **Use the escape hatch sparingly and explicitly.** It bypasses the
  accessibility model entirely. Keep `AGENT_DESKTOP_ENABLE_EXEC` unset unless a
  task genuinely needs shell/script access, and inspect `exec_audit.log` after.

## Error Codes

| Code | Cause |
|------|-------|
| `POLICY_DENIED` | Escape-hatch command run without `AGENT_DESKTOP_ENABLE_EXEC=1` |
| `INVALID_ARGS` | Missing/duplicate mode flag, or `--set` outside `0..=100` |
| `PERM_DENIED` | System service refused (e.g. automation permission) |
| `ACTION_FAILED` | Underlying service call failed |
| `TIMEOUT` | `run-*` child exceeded `--timeout` |
