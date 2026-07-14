#!/usr/bin/env bash
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
runner=$(cd "$here/.." && pwd)/run_fleet.sh
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

fleet=$'01 fast-pass high\n02 fast-fail high\n03 slow-pass high'
PROMPT_FILE="$here/prompt.txt" \
FITNESS_CMD="bash \"$here/fitness.sh\" \"\$1\"" \
WORK="$work" NAME=self EXT=txt FLEET="$fleet" ALLOW_SMALLER_FLEET=1 \
SOL=1 LUNA=2 CLAUDEX="$here/fake_claudex.sh" ROUND_TIMEOUT=20 FITNESS_TIMEOUT=5 \
CAMPAIGN_TEST_MODE=1 TEST_SOL_PASS_GRACE=3 TEST_SOL_PASS_WARNING=2 \
  bash "$runner"

results="$work/self/results.tsv"
grep -q $'^01\tpass=1/1\t' "$results"
grep -q $'^02\tpass=0/1\t' "$results"
grep -q $'^03\tCULLED\t' "$results"
grep -q '^PASS$' "$work/champion_self.txt"
test -s "$work/self/first_sol_pass"
for idx in 01 02 03; do test -s "$work/self/warning_${idx}.txt"; done
grep -q 'SOL_PASS_ONE_MINUTE_WARNING' "$work/self/group_events.tsv"

luna_fleet=$'01 fast-fail high\n02 fast-pass high'
PROMPT_FILE="$here/prompt.txt" FITNESS_CMD="bash \"$here/fitness.sh\" \"\$1\"" \
  WORK="$work/luna-trigger" NAME=self EXT=txt FLEET="$luna_fleet" ALLOW_SMALLER_FLEET=1 \
  SOL=1 LUNA=1 CLAUDEX="$here/fake_claudex.sh" ROUND_TIMEOUT=20 FITNESS_TIMEOUT=5 \
  CAMPAIGN_TEST_MODE=1 TEST_SOL_PASS_GRACE=3 TEST_SOL_PASS_WARNING=2 \
  bash "$runner"
test ! -e "$work/luna-trigger/self/first_sol_pass"
test ! -e "$work/luna-trigger/self/group_events.tsv"

if PROMPT_FILE="$here/prompt.txt" FITNESS_CMD='sleep 8; echo "FITNESS pass=1/1 metric=1"' \
  WORK="$work/fitness" NAME=self EXT=txt FLEET="$luna_fleet" ALLOW_SMALLER_FLEET=1 \
  SOL=1 LUNA=1 CLAUDEX="$here/fake_claudex.sh" ROUND_TIMEOUT=20 FITNESS_TIMEOUT=1 \
  CAMPAIGN_TEST_MODE=1 TEST_SOL_PASS_GRACE=3 TEST_SOL_PASS_WARNING=2 \
  bash "$runner" >/dev/null 2>&1; then
  echo 'expected a timed-out fitness command to prevent a champion' >&2
  exit 1
fi
grep -q $'^01\tTIMEOUT\t' "$work/fitness/self/results.tsv"

if PROMPT_FILE="$here/prompt.txt" FITNESS_CMD='true' WORK="$work/small" FLEET="$fleet" SOL=1 LUNA=2 CLAUDEX="$here/fake_claudex.sh" bash "$runner" >/dev/null 2>&1; then
  echo 'expected an unauthorized smaller fleet to be rejected' >&2
  exit 1
fi

too_many=$'01 fast-pass high\n02 fast-pass high\n03 fast-pass high\n04 fast-pass high\n05 fast-pass high\n06 fast-pass high\n07 fast-pass high\n08 fast-pass high\n09 fast-pass high\n10 fast-pass high\n11 fast-pass high'
if PROMPT_FILE="$here/prompt.txt" FITNESS_CMD='true' WORK="$work/max" FLEET="$too_many" SOL=4 LUNA=7 CLAUDEX="$here/fake_claudex.sh" bash "$runner" >/dev/null 2>&1; then
  echo 'expected the eleven-unit fleet to be rejected' >&2
  exit 1
fi

wrong_ratio=$'01 fast-pass high\n02 fast-pass high\n03 fast-pass high\n04 fast-pass high\n05 fast-pass high\n06 fast-pass high\n07 fast-pass high\n08 fast-pass high\n09 fast-pass high\n10 fast-pass high'
if PROMPT_FILE="$here/prompt.txt" FITNESS_CMD='true' WORK="$work/ratio" FLEET="$wrong_ratio" \
  SOL=5 LUNA=5 CLAUDEX="$here/fake_claudex.sh" CAMPAIGN_TEST_MODE=1 bash "$runner" >/dev/null 2>&1; then
  echo 'expected a non-4/6 ten-unit fleet to be rejected' >&2
  exit 1
fi

echo 'run_fleet.sh self-test passed'
