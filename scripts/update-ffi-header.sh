#!/usr/bin/env bash
set -euo pipefail

# Regenerate the committed FFI header from source. This script is deliberately
# outside the normal Cargo build graph so cbindgen never executes during
# ordinary builds, tests, CI, or release packaging.

ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"

if ! command -v cbindgen >/dev/null 2>&1; then
  echo "ERROR: cbindgen is not installed. Install and audit it explicitly before regenerating the FFI header." >&2
  exit 1
fi

cbindgen crates/ffi \
  --config crates/ffi/cbindgen.toml \
  --output crates/ffi/include/agent_desktop.h
echo "Updated crates/ffi/include/agent_desktop.h"
