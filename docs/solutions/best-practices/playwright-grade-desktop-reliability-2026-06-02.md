---
title: Playwright-grade desktop reliability contract
date: 2026-06-02
category: best-practices
module: crates/core
problem_type: best_practice
component: reliability
severity: high
applies_when:
  - Ref resolution, action dispatch, wait semantics, session scope, or FFI action paths change
  - A platform adapter is added or modified
  - A command moves from direct adapter calls to shared helpers
tags:
  - reliability
  - playwright
  - refs
  - actionability
  - cross-platform
---

# Playwright-grade desktop reliability contract

## Context

Playwright is reliable because actions flow through a consistent ladder:
resolve a locator, prove it is actionable, wait when the state is changing, and
fail with structured recovery when the target is stale or ambiguous. Desktop
automation cannot copy browser semantics directly, but it can use the same
engineering shape.

## Contract

Ref actions must pass through the shared reliability path:

1. Load the refmap from the caller's session.
2. Resolve the ref with strict platform identity checks.
3. Return `STALE_REF` when the old element no longer matches.
4. Return `AMBIGUOUS_TARGET` when multiple candidates match.
5. Run live actionability checks before adapter dispatch.
6. Keep waits bounded by the caller timeout, including platform resolution retries.
7. Emit trace events only to the requested JSONL trace path, never stdout.

## Cross-Platform Rule

Core owns the contract; adapters own native evidence. Windows and Linux should
not fork CLI semantics. UIA and AT-SPI implementations must map their native
identity fields into the same `RefEntry` concepts: role, name, value,
description, state, bounds, supported actions, source surface, root ref, and
tree path.

Actionability should prefer one native live-state read that returns state,
bounds, and supported actions together. Platform adapters may fall back to
separate reads, but the CLI behavior must remain identical: empty transient
action reads do not erase snapshot capabilities, while a non-empty live action
set that lacks the required action can block dispatch.

## Review Rule

Any change to ref resolution or action dispatch must include tests for:

- stale ref rejection
- ambiguous target rejection
- actionability failure before dispatch
- retrying waits that honor timeout and report last observed state
- session isolation
- FFI parity when the behavior is exposed through C ABI

If a platform needs a coordinate fallback, the fallback must be explicit and
lower confidence. Do not silently replace a failed semantic action with a pixel
click.
