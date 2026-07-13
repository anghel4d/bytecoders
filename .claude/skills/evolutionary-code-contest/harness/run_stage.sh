#!/usr/bin/env bash
# One supervisor agent, one generation. Commissions a fleet of claudex candidates for a single stage,
# fitness-tests each against the frozen substrate, ranks, and copies a provisional champion.
# The Claude supervisor that invokes this then reviews results.tsv and may override the champion on quality.
#
# Usage:  CONTEST_ROOT=/harness  WORK=/contest/agent-N  run_stage.sh <SLOT A|B|C>
#   CONTEST_ROOT   holds prelude.rs, testmain.rs, compose_and_test.sh, prompts/prompt_<SLOT>.txt,
#                  and substrate/{A,B,C}.rs (the frozen winners of prior generations + goldens for later slots)
#   WORK           this agent's output dir; candidates land in $WORK/<SLOT>/, champion at $WORK/champion_<SLOT>.rs
# Env knobs: CLAUDEX (default ~/.local/bin/claudex), PARALLEL (default 3),
#   SOL / LUNA (counts of gpt-5.6-sol/high and gpt-5.6-luna/xhigh candidates; default 6 and 12).
#   The supervisor invoking this owns the mix — size it to the stage and to expected attrition;
#   the original contest ran SOL=5 LUNA=10 (set those to reproduce it exactly).
#   FLEET (newline list of "<idx> <model> <effort>"; full override, ignores SOL/LUNA).
set -u
ROOT="${CONTEST_ROOT:?set CONTEST_ROOT}"; SLOT="$1"
WORK="${WORK:?set WORK to the output dir of this agent}"
SUB="$ROOT/substrate"
CLAUDEX="${CLAUDEX:-$HOME/.local/bin/claudex}"
PARALLEL="${PARALLEL:-3}"
mkdir -p "$WORK/$SLOT"
PROMPT="$(cat "$ROOT/prompts/prompt_$SLOT.txt")"
export WORK PROMPT SLOT CLAUDEX

gen() {  # $1 idx  $2 model  $3 effort
  "$CLAUDEX" -p --model "$2" --effort "$3" "$PROMPT" 2>/dev/null \
    | grep -vi 'connectors are disabled' | sed '/^```/d' > "$WORK/$SLOT/cand_${1}.rs"
}
export -f gen

SOL="${SOL:-6}"; LUNA="${LUNA:-12}"
FLEET="${FLEET:-$(
  for i in $(seq 1 "$SOL");  do printf '%02d gpt-5.6-sol high\n'   "$i"; done
  for i in $(seq 1 "$LUNA"); do printf '%02d gpt-5.6-luna xhigh\n' $((SOL + i)); done
)}"

echo "[$WORK $SLOT] commissioning $(echo "$FLEET" | grep -c .) claudex candidates ..."
echo "$FLEET" | grep . | xargs -P "$PARALLEL" -I{} bash -c 'set -- {}; gen "$1" "$2" "$3"'

: > "$WORK/$SLOT/results.tsv"
for c in "$WORK/$SLOT"/cand_*.rs; do
  idx=$(basename "$c" .rs | sed 's/cand_//')
  case "$SLOT" in
    A) fit=$(CONTEST_ROOT="$ROOT" bash "$ROOT/compose_and_test.sh" "$c" "$SUB/B.rs" "$SUB/C.rs");;
    B) fit=$(CONTEST_ROOT="$ROOT" bash "$ROOT/compose_and_test.sh" "$SUB/A.rs" "$c" "$SUB/C.rs");;
    C) fit=$(CONTEST_ROOT="$ROOT" bash "$ROOT/compose_and_test.sh" "$SUB/A.rs" "$SUB/B.rs" "$c");;
  esac
  pass=$(echo "$fit"   | grep -oE 'pass=[0-9]+/[0-9]+' | head -1)
  red=$(echo  "$fit"   | grep -oE 'reduction=[0-9]+'   | head -1)
  status=$(echo "$fit" | awk '{print $1}')
  printf '%s\t%s\t%s\t%sL\n' "$idx" "${pass:-$status}" "${red:-reduction=NA}" "$(wc -l < "$c" | tr -d ' ')" >> "$WORK/$SLOT/results.tsv"
done

# rank: fitness numerator desc, reduction desc, fewest lines
best=$(awk -F'\t' '{split($2,p,"[=/]"); split($3,r,"="); split($4,l,"L");
  print (p[2]+0)"\t"(r[2]+0)"\t"(-(l[1]+0))"\t"$1}' "$WORK/$SLOT/results.tsv" \
  | sort -k1,1nr -k2,2nr -k3,3nr | head -1 | cut -f4)

echo "[$WORK $SLOT] ranked results:"
sort "$WORK/$SLOT/results.tsv" | column -t -s $'\t'
[ -n "$best" ] && cp "$WORK/$SLOT/cand_${best}.rs" "$WORK/champion_$SLOT.rs" \
  && echo "PROVISIONAL_WINNER=$best  (champion at $WORK/champion_$SLOT.rs)"
