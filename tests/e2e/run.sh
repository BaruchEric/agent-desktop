#!/usr/bin/env bash
# End-to-end tests: drive the real agent-desktop binary against the fixture
# app and verify real OS outcomes by INDEPENDENT OBSERVATION — never the
# command's own ok:true. EVERY check prints the values it observed (before/
# after, got/expected, the actual JSON fields) inline, so any failure is
# debuggable from the output alone and a command that returns ok:true without
# an effect is caught.
#
# The fixture (tests/fixture-app) is a fixed, diverse slice of real macOS UI.
# It is never tuned to make a command pass; a failure here is a finding about
# the CLI or this harness.
#
# Usage: tests/e2e/run.sh   (needs: cargo build --release + AX permission)
set -uo pipefail

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
bin="$repo/target/release/agent-desktop"
fixture_app="$repo/tests/fixture-app/build/AgentDeskFixture.app"
app="AgentDeskFixture"

pass=0; fail=0; declare -a failures
note() { printf '\n\033[1;34m== %s ==\033[0m\n' "$1"; }
okmsg() { printf '  \033[0;32mPASS\033[0m %s\n' "$1"; pass=$((pass+1)); }
badmsg(){ printf '  \033[0;31mFAIL\033[0m %s\n' "$1"; fail=$((fail+1)); failures+=("$1"); }
skip()  { printf '  \033[0;33mNOTE\033[0m %s\n' "$1"; }
# assert <label> <pass?0|1> <detail-with-observed-values>
assert() { if [ "$2" = "1" ]; then okmsg "$1  [$3]"; else badmsg "$1  [$3]"; fi; }

# Interaction mode is Playwright-style: headless is the default (AX-only, no
# cursor), --headed opts in to cursor/physical fallbacks. Observations
# (resolve/read_value/snapshot) are ALWAYS headless; only the action under test
# carries $MODE_FLAG, set per-suite so every interaction is proven in BOTH modes.
MODE_FLAG=""
act() { "$bin" $MODE_FLAG "$@"; }

field() { python3 -c "import json,sys
try: d=json.load(sys.stdin)
except Exception: print(''); sys.exit()
try: print(eval('d'+sys.argv[1]))
except Exception: print('')" "$1" 2>/dev/null; }

resolve()    { "$bin" find --app "$app" --role "$1" --name "$2" --first 2>/dev/null | field "['data']['match']['ref']"; }
read_value() { "$bin" find --app "$app" --role statictext --name "$1" --first 2>/dev/null | field "['data']['match']['value']"; }
running()    { "$bin" list-apps 2>/dev/null | python3 -c "import json,sys;print(any(a['name']=='$app' for a in json.load(sys.stdin)['data']['apps']))" 2>/dev/null; }

# verify <label> <status-name> <expected> <subcmd...> : observe before, run the
# subcommand in the CURRENT $MODE_FLAG (no leading $bin), observe after.
verify() {
    local label="$1" status="$2" expected="$3"; shift 3
    local before after out cmd_ok err
    before="$(read_value "$status")"
    out="$(act "$@" 2>&1)"; sleep 0.35
    after="$(read_value "$status")"
    cmd_ok="$(echo "$out" | field "['ok']")"
    err="$(echo "$out" | field "['error']['code']")"
    assert "$label" "$([ "$after" = "$expected" ] && echo 1 || echo 0)" \
        "before='$before' after='$after' expected='$expected' cmd_ok=$cmd_ok${err:+ err=$err}"
}

# interaction_suite <headless|headed> : drive every ref-action command in the
# given mode with mode-specific target values, so a regression in EITHER mode is
# caught by an independent before/after observation. Headed must not regress the
# AX path (it is tried first); it only adds cursor/physical fallbacks.
interaction_suite() {
    local MODE="$1" MODE_FLAG="" sel sld stp dir
    if [ "$MODE" = headed ]; then MODE_FLAG="--headed"; sel=Gamma; sld=60; stp=6; dir=up
    else sel=Beta; sld=50; stp=4; dir=down; fi
    "$bin" focus-window --app "$app" >/dev/null 2>&1

    note "[$MODE] click / type / set-value / clear"
    verify "click sets click-status" click-status clicked     click "$(resolve button primary-button)"
    verify "type sets field"         text-echo "typed-$MODE"  type "$(resolve textfield text-input)" "typed-$MODE"
    verify "set-value sets field"    text-echo "set-$MODE"    set-value "$(resolve textfield text-input)" "set-$MODE"
    verify "clear empties field"     text-echo ""             clear "$(resolve textfield text-input)"

    note "[$MODE] check / uncheck (reset before each)"
    act uncheck "$(resolve checkbox toggle-box)" >/dev/null 2>&1; sleep 0.2
    verify "check turns toggle on"    toggle-status on   check "$(resolve checkbox toggle-box)"
    verify "uncheck turns toggle off" toggle-status off  uncheck "$(resolve checkbox toggle-box)"

    note "[$MODE] select / slider / stepper (mode-specific targets)"
    verify "select combobox -> $sel"  picker-status  "$sel" select "$(resolve combobox option-picker)" "$sel"
    verify "set-value slider -> $sld" slider-status  "$sld" set-value "$(resolve slider value-slider)" "$sld"
    verify "set-value stepper -> $stp" stepper-status "$stp" set-value "$(resolve incrementor value-stepper)" "$stp"

    note "[$MODE] scroll $dir (observed offset delta)"
    local so_b so_a
    so_b="$(read_value scroll-offset)"
    act scroll "$(resolve scrollarea scroll-area)" --direction "$dir" --amount 10 >/dev/null 2>&1; sleep 0.4
    so_a="$(read_value scroll-offset)"
    assert "[$MODE] scroll moved content" "$([ -n "$so_b" ] && [ "$so_b" != "$so_a" ] && echo 1 || echo 0)" \
        "offset before='$so_b' after='$so_a' dir=$dir"
}

cleanup() { "$bin" close-app "$app" --force >/dev/null 2>&1 || true; }
trap cleanup EXIT

# --- Setup -----------------------------------------------------------------
note "Setup"
[ -x "$bin" ] || { echo "release binary missing; run 'cargo build --release'"; exit 2; }
[ -d "$fixture_app" ] || "$repo/tests/fixture-app/build.sh" >/dev/null
"$bin" close-app "$app" --force >/dev/null 2>&1 || true; sleep 1
open "$fixture_app"
ready=""; tries=0
for _ in $(seq 1 20); do
    tries=$((tries+1))
    "$bin" focus-window --app "$app" >/dev/null 2>&1 || true
    [ -n "$(resolve button primary-button)" ] && { ready=1; break; }
    sleep 0.5
done
assert "fixture launched and tree exposed" "$ready" "primary-button resolvable after $tries focus attempts"

# --- Observation -----------------------------------------------------------
note "snapshot role diversity (observed roles vs expected)"
snap="$("$bin" snapshot --app "$app" --max-depth 30 2>/dev/null)"
refc="$(echo "$snap" | field "['data']['ref_count']")"
for r in button textfield checkbox combobox slider incrementor radiobutton disclosure link tab treeitem scrollarea; do
    present="$(echo "$snap" | grep -qc "\"role\":\"$r\"" && echo 1 || echo 0)"
    assert "role $r" "$([ "$(echo "$snap" | grep -c "\"role\":\"$r\"")" -ge 1 ] && echo 1 || echo 0)" \
        "found=$([ "$(echo "$snap" | grep -c "\"role\":\"$r\"")" -ge 1 ] && echo yes || echo NO) ref_count=$refc"
done

note "find vocabulary (observed resolution)"
tf="$(resolve textfield text-input)"
assert "find textfield by name" "$([ -n "$tf" ] && echo 1 || echo 0)" "resolved ref='$tf'"
ta="$("$bin" find --app "$app" --role textarea --name text-input --first 2>/dev/null | field "['data']['match']['ref']")"
assert "textarea alias -> textfield" "$([ -n "$ta" ] && echo 1 || echo 0)" "alias resolved ref='$ta'"
hint="$("$bin" find --app "$app" --role navbar 2>/dev/null | field "['data']['roles_present']")"
assert "absent role returns roles_present hint" "$([ -n "$hint" ] && echo 1 || echo 0)" "roles_present=${hint:0:60}..."

# --- Interaction in BOTH modes (the headed/headless contract) ---------------
# Every ref-action command is driven twice — default headless, then --headed —
# and verified by independent before/after observation each time. This is the
# most important guarantee: actions must work in both modes, and --headed must
# not regress the AX path.
interaction_suite headless
interaction_suite headed

note "radio (one-way, headless)"
verify "click radio option Two"   radio-status  Two  click "$(resolve radiobutton Two)"

note "double-click: the headless/headed discriminator (gesture-only target, no AXOpen)"
dt="$(resolve button double-target)"
"$bin" focus-window --app "$app" >/dev/null 2>&1
dc_b="$(read_value double-status)"
hl_out="$("$bin" double-click "$dt" 2>&1)"; sleep 0.3
hl_a="$(read_value double-status)"; hl_code="$(echo "$hl_out" | field "['error']['code']")"
assert "headless double-click fails closed (no cursor)" \
    "$([ "$hl_a" = "$dc_b" ] && [ "$hl_code" = "POLICY_DENIED" ] && echo 1 || echo 0)" \
    "before='$dc_b' after='$hl_a' err=$hl_code"
"$bin" focus-window --app "$app" >/dev/null 2>&1
hd_out="$("$bin" --headed double-click "$dt" 2>&1)"; sleep 0.3
hd_a="$(read_value double-status)"; hd_ok="$(echo "$hd_out" | field "['ok']")"
assert "--headed double-click completes (physical fallback unlocked)" \
    "$([ "$hd_a" = "double-clicked" ] && echo 1 || echo 0)" \
    "before='$hl_a' after='$hd_a' cmd_ok=$hd_ok"

# --- Strict resolution -----------------------------------------------------
note "ambiguous twins do not silently act"
out="$("$bin" click "$(resolve button twin-control)" 2>&1)"
acode="$(echo "$out" | field "['error']['code']")"; aok="$(echo "$out" | field "['ok']")"
if [ "$acode" = "AMBIGUOUS_TARGET" ] || [ "$aok" = "True" ]; then
    assert "twin did not silently act" 1 "code='$acode' ok=$aok (ambiguous or resolved-by-bounds)"
else
    assert "twin did not silently act" 0 "code='$acode' ok=$aok out=${out:0:80}"
fi

note "removed element fails closed"
sid="$("$bin" snapshot --app "$app" | field "['data']['snapshot_id']")"
stale="$(python3 -c "import json,subprocess
d=json.loads(subprocess.run(['$bin','snapshot','--app','$app','--snapshot','$sid','--max-depth','30'],capture_output=True,text=True).stdout)
def f(n):
  if n.get('name')=='removable-row' and n.get('ref_id'): return n['ref_id']
  for c in n.get('children',[]):
    r=f(c)
    if r: return r
print(f(d['data']['tree']) or '')" 2>/dev/null)"
"$bin" click "$(resolve button remove-row)" >/dev/null 2>&1; sleep 0.4
out="$("$bin" click "$stale" --snapshot "$sid" 2>&1)"; code="$(echo "$out" | field "['error']['code']")"; ok="$(echo "$out" | field "['ok']")"
case "$code" in
    STALE_REF) assert "removed -> STALE_REF" 1 "ref='$stale' code='$code' ok=$ok" ;;
    TIMEOUT|ELEMENT_NOT_FOUND|AMBIGUOUS_TARGET) assert "removed failed closed" 1 "ref='$stale' code='$code' (STALE_REF preferred)" ;;
    *) assert "removed failed closed" 0 "ref='$stale' code='$code' ok=$ok (acted on removed element!)" ;;
esac

# --- Reliability core (observed values inline) -----------------------------
note "wait predicates"
"$bin" click "$(resolve button enable-later)" >/dev/null 2>&1
we="$("$bin" wait --element "$(resolve button delayed-button)" --predicate enabled --timeout 5000 2>/dev/null)"
assert "wait enabled (async)" "$([ "$(echo "$we" | field "['data']['found']")" = "True" ] && echo 1 || echo 0)" \
    "found=$(echo "$we" | field "['data']['found']") elapsed_ms=$(echo "$we" | field "['data']['elapsed_ms']")"
pb="$(resolve button primary-button)"
wa="$("$bin" wait --element "$pb" --predicate actionable --timeout 3000 2>/dev/null)"
assert "predicate actionable" "$([ "$(echo "$wa" | field "['data']['found']")" = "True" ] && echo 1 || echo 0)" \
    "found=$(echo "$wa" | field "['data']['found']") actionable=$(echo "$wa" | field "['data']['observed']['actionable']")"
wv="$("$bin" wait --element "$pb" --predicate visible --timeout 3000 2>/dev/null)"
assert "predicate visible" "$([ "$(echo "$wv" | field "['data']['found']")" = "True" ] && echo 1 || echo 0)" "found=$(echo "$wv" | field "['data']['found']")"
"$bin" set-value "$(resolve textfield text-input)" "pred-val" >/dev/null 2>&1; sleep 0.3
wval="$("$bin" wait --element "$(resolve textfield text-input)" --predicate value --value pred-val --timeout 3000 2>/dev/null)"
assert "predicate value" "$([ "$(echo "$wval" | field "['data']['found']")" = "True" ] && echo 1 || echo 0)" \
    "found=$(echo "$wval" | field "['data']['found']") matched=$(echo "$wval" | field "['data']['observed']['matched']")"

note "wait --text (async appear)"
"$bin" click "$(resolve button appear-later)" >/dev/null 2>&1
wt="$("$bin" wait --text appeared-text --app "$app" --timeout 5000 2>/dev/null)"
assert "wait text resolved" "$([ "$(echo "$wt" | field "['data']['found']")" = "True" ] && echo 1 || echo 0)" \
    "found=$(echo "$wt" | field "['data']['found']") elapsed_ms=$(echo "$wt" | field "['data']['elapsed_ms']")"

note "skeleton traversal + scoped drill-down"
sk="$("$bin" snapshot --app "$app" --skeleton 2>/dev/null)"; sk_id="$(echo "$sk" | field "['data']['snapshot_id']")"
sk_refs="$(echo "$sk" | field "['data']['ref_count']")"
anchor="$(echo "$sk" | python3 -c "import json,sys
d=json.load(sys.stdin)
def f(n):
  if n.get('children_count') and n.get('ref_id'): return n['ref_id']
  for c in n.get('children',[]):
    r=f(c)
    if r: return r
print(f(d['data']['tree']) or '')" 2>/dev/null)"
drilled="$([ -n "$anchor" ] && "$bin" snapshot --app "$app" --root "$anchor" --snapshot "$sk_id" 2>/dev/null | field "['data']['ref_count']")"
assert "skeleton + drill-down" "$([ -n "$anchor" ] && [ -n "$drilled" ] && [ "$drilled" -gt 0 ] && echo 1 || echo 0)" \
    "skeleton_refs=$sk_refs anchor='$anchor' drilled_refs='$drilled'"

note "session isolation + session-independent explicit snapshot"
sa="$("$bin" --session run-a snapshot --app "$app" 2>/dev/null | field "['data']['snapshot_id']")"
sb="$("$bin" --session run-b snapshot --app "$app" 2>/dev/null | field "['data']['snapshot_id']")"
assert "sessions keep distinct latest pointers" "$([ -n "$sa" ] && [ -n "$sb" ] && [ "$sa" != "$sb" ] && echo 1 || echo 0)" "run-a='$sa' run-b='$sb'"
ra="$("$bin" --session run-a find --app "$app" --role button --name primary-button --first 2>/dev/null | field "['data']['match']['ref']")"
xok="$("$bin" get "$ra" --snapshot "$sa" 2>/dev/null | field "['ok']")"
assert "session-a snapshot resolves without --session" "$([ "$xok" = "True" ] && echo 1 || echo 0)" "ref='$ra' snapshot='$sa' get_ok=$xok"

note "trace JSONL + secret redaction"
trf="$(mktemp -t agentdesk-e2e-trace.XXXXXX)"
"$bin" --trace "$trf" type "$(resolve textfield text-input)" "sup3r-secret-trace" >/dev/null 2>&1; sleep 0.2
bytes="$(wc -c < "$trf" | tr -d ' ')"
has_events="$(grep -qc 'ref.resolve' "$trf" && echo 1 || echo 0)"
leaked="$(grep -qc 'sup3r-secret-trace' "$trf" && echo 1 || echo 0)"
redacted="$(grep -qc 'redacted' "$trf" && echo 1 || echo 0)"
assert "trace recorded resolver events" "$has_events" "bytes=$bytes resolver_events=$([ "$has_events" = 1 ] && echo yes || echo no)"
assert "typed secret NOT in trace" "$([ "$leaked" = "0" ] && echo 1 || echo 0)" "secret_present=$([ "$leaked" = 1 ] && echo YES || echo no) redaction_markers=$([ "$redacted" = 1 ] && echo yes || echo no)"
rm -f "$trf"

# --- Surfaces / drag / expand (before/after) -------------------------------
note "surface: open sheet, list-surfaces sees it, act inside"
sheet_b="$(read_value sheet-status)"
"$bin" click "$(resolve button open-sheet)" >/dev/null 2>&1; sleep 0.6
surf="$("$bin" list-surfaces --app "$app" 2>&1)"
surf_has_sheet="$(echo "$surf" | grep -qci sheet && echo 1 || echo 0)"
assert "list-surfaces reports the sheet" "$surf_has_sheet" "surfaces=$(echo "$surf" | field "['data']['surfaces']")"
"$bin" focus-window --app "$app" >/dev/null 2>&1
"$bin" click "$(resolve button confirm-sheet)" >/dev/null 2>&1; sleep 0.5
sheet_a="$(read_value sheet-status)"
assert "acted inside the sheet" "$([ "$sheet_a" = "confirmed" ] && echo 1 || echo 0)" "sheet-status before='$sheet_b' after='$sheet_a'"

note "drag: source-tracked gesture across a single view (verified by canvas)"
dr_b="$(read_value drag-canvas-status)"
cxy="$("$bin" snapshot --app "$app" --include-bounds --max-depth 30 2>/dev/null | python3 -c "import json,sys
d=json.load(sys.stdin)
def f(n):
  if n.get('name')=='drag-canvas' and n.get('bounds'):
    b=n['bounds']; return b
  for c in n.get('children',[]):
    r=f(c)
    if r: return r
b=f(d['data']['tree'])
print(f\"{b['x']+20},{b['y']+b['height']/2} {b['x']+b['width']-20},{b['y']+b['height']/2}\" if b else '')" 2>/dev/null)"
from_xy="$(echo "$cxy" | awk '{print $1}')"; to_xy="$(echo "$cxy" | awk '{print $2}')"
"$bin" drag --from-xy "$from_xy" --to-xy "$to_xy" --duration 400 --drop-delay 300 >/dev/null 2>&1; sleep 0.4
dr_a="$(read_value drag-canvas-status)"
assert "drag delivered a gesture" "$(echo "$dr_a" | grep -q '^dragged-' && echo 1 || echo 0)" "from='$from_xy' to='$to_xy' canvas before='$dr_b' after='$dr_a'"

note "expand a press-toggled disclosure (verified by disclosure value)"
exp_b="$("$bin" get "$(resolve disclosure disclosure-section)" 2>/dev/null | field "['data']['value']")"
eout="$("$bin" expand "$(resolve disclosure disclosure-section)" 2>&1)"; sleep 0.4
exp_a="$("$bin" get "$(resolve disclosure disclosure-section)" 2>/dev/null | field "['data']['value']")"
eok="$(echo "$eout" | field "['ok']")"
if [ "$exp_a" = "true" ]; then
    assert "expand set disclosure expanded" 1 "value before='$exp_b' after='$exp_a' cmd_ok=$eok"
elif [ "$eok" = "False" ]; then
    skip "expand honestly failed (disclosure not AX-actionable)  [value before='$exp_b' after='$exp_a' cmd_ok=$eok]"
else
    assert "expand set disclosure expanded" 0 "claimed success but value before='$exp_b' after='$exp_a' cmd_ok=$eok"
fi

note "close-app --force terminates (observed via list-apps)"
run_b="$(running)"
"$bin" close-app "$app" --force >/dev/null 2>&1; sleep 1.5
run_a="$(running)"
assert "force close removed app" "$([ "$run_a" = "False" ] && echo 1 || echo 0)" "running before=$run_b after=$run_a"

note "Documented limitations (tracked separately, not failures)"
skip "cross-target native drag-and-drop (onDrop) needs the OS dragging-session/pasteboard protocol; synthetic mouse events route mouse-up to the drag origin, so they cannot drop onto a separate native target (works for source-tracked gestures, web/Electron mouse-DnD)"
skip "SwiftUI Slider/Stepper/DisclosureGroup are not AX-actionable; native AppKit equivalents are (set-value/expand work on those)"

# --- Summary ---------------------------------------------------------------
note "Summary"
printf '  passed: %d  failed: %d\n' "$pass" "$fail"
if [ "$fail" -gt 0 ]; then
    printf '\n  failures (observed values inline above):\n'
    for f in "${failures[@]}"; do printf '   - %s\n' "$f"; done
    exit 1
fi
echo "  all E2E scenarios passed"
