# Concepts

Shared domain vocabulary for this project -- entities, named processes, and status concepts with project-specific meaning. Seeded with core domain vocabulary, then accretes as ce-compound and ce-compound-refresh process learnings; direct edits are fine. Glossary only, not a spec or catch-all.

## Desktop Observation

### Accessibility Tree
A structured representation of an application's user interface exposed by the operating system accessibility APIs and used by agent-desktop as the source of truth for observation and semantic interaction.

### Snapshot
An observation of an accessibility tree at a point in time, persisted with the element refs allocated from that observation.

### Snapshot ID
A compact identifier for one persisted snapshot. Explicit snapshot IDs are direct handles; callers do not need the original session when they pass the ID.

### Surface
A scoped UI layer that can be observed separately from the whole window, such as an open menu, sheet, popover, alert, or focused area.

### Drill-down
A snapshot operation that starts from an existing ref to observe that element's subtree instead of re-reading the entire window.

## Refs And Identity

### Ref
A short element identifier assigned by agent-desktop to an actionable or drillable node in a snapshot.

Refs are deterministic inside one snapshot but are not stable across UI changes. Callers either pass the snapshot ID that produced the ref or intentionally use the session's latest snapshot pointer.

### RefMap
The persisted mapping from refs to the identity evidence needed to re-identify elements later.

### Stale Ref
A ref whose stored identity no longer matches a live element strongly enough to act safely.

### Strict Ref Resolution
The fail-closed process of re-identifying a ref from stored identity evidence before a command acts on it.

Strict ref resolution rejects missing, stale, and ambiguous matches instead of guessing. It is the boundary between an old observation and a live desktop mutation.

## Coordination

### Session
A coordination key for one agent or a coordinated group of agents that share a latest-snapshot pointer.

Use sessions when callers intentionally omit `--snapshot` and want a shared latest observation. Explicit snapshot IDs remain the deterministic path for pinned actions and can be resolved without also passing the session.

## Action Reliability

### Actionability
The pre-dispatch judgement that a resolved element is safe to act on, based on native evidence such as visibility, stability, enabled state, supported action, policy, and editability.

### Capability Vocabulary
The platform-neutral set of supported action names that core uses to compare command intent with native adapter evidence.

Each adapter maps native primitives into this shared vocabulary before core evaluates actionability. New commands should extend the central vocabulary first, then reuse it from actionability, ref allocation, predicates, FFI tests, and platform adapters.

### Interaction Policy
The side-effect contract attached to an action request, controlling whether the command may steal focus, move the cursor, or use physical input fallbacks.

### Headless Ref Action
A ref-based action that uses semantic accessibility operations without implicit focus stealing, cursor movement, synthetic keyboard input, or pasteboard use.

Headless ref actions may still fail when the native accessibility API cannot perform the requested semantic operation. A broader interaction policy must be explicit rather than silently substituting physical input.

### Wait Predicate
The condition a wait command polls for before returning, such as element actionability, text presence, window appearance, menu state, or notification arrival.

### Resolver Deadline
The remaining time budget carried through strict ref resolution so every native read can fail with a structured timeout instead of using an unrelated platform default timeout.

### Coordinate Fallback
An explicit opt-in path that uses screen coordinates or physical input when semantic accessibility operations cannot perform the requested action.

### FFI Ref-Action Parity
The requirement that language bindings using refs follow the same strict resolution, actionability, and interaction-policy semantics as CLI ref commands.

## Relationships

A session owns one latest-snapshot pointer. A snapshot persists a ref map and can be selected directly by snapshot ID. A ref resolves through strict ref resolution into live native evidence, then actionability decides whether a headless ref action can safely dispatch. FFI ref-action parity keeps that same relationship true for language bindings.
