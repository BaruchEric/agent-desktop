#!/usr/bin/env bash
# Run the gated real-app e2e suite against the local desktop.
#
# These tests launch and drive real macOS applications (Safari, TextEdit,
# Finder, System Settings) and require Accessibility permission for the
# terminal running them. They are #[ignore]-gated so a normal `cargo test`
# never touches the desktop; this script opts in explicitly.
#
# Usage:
#   scripts/e2e-macos.sh                 # build, then run every e2e target
#   scripts/e2e-macos.sh browser_test    # run a single target
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "SKIP: the e2e suite is macOS-only (got $(uname -s))" >&2
  exit 0
fi

# Real apps share global desktop + ~/.agent-desktop state, so serialize.
THREADS=(--test-threads=1)

# Exec-gated commands (run-shell roundtrips) need the escape hatch enabled.
export AGENT_DESKTOP_ENABLE_EXEC=1

echo "==> Building agent-desktop (debug) so tests can exec the binary"
cargo build -p agent-desktop

E2E_TARGETS=(
  browser_test
  interaction_test
  clipboard_test
  window_test
  batch_test
  keyboard_test
  mouse_test
  scroll_test
  notifications_test
  wait_test
)

if [[ $# -gt 0 ]]; then
  E2E_TARGETS=("$@")
fi

for target in "${E2E_TARGETS[@]}"; do
  echo "==> cargo test --test ${target} -- --ignored"
  cargo test -p agent-desktop --test "${target}" -- --ignored "${THREADS[@]}"
done

echo "OK: e2e suite complete"
