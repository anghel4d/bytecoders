#!/usr/bin/env bash
# run_fleet.sh — one squadron, one contest round. Domain-agnostic: PROMPT_FILE says what
# to generate, FITNESS_CMD says how to score it. Commissions a fleet of cheap-backend
# candidates, scores each, ranks, and copies a provisional champion. The squadron leader
# that invokes this then reviews results.tsv and may override the champion on
# engineering quality among top-fitness candidates.
set -u

usage() { cat <<'EOF'
usage:  PROMPT_FILE=prompts/parser.txt FITNESS_CMD='bash fit.sh "$1"' \
        WORK=/contest/squad-1 NAME=parser  bash run_fleet.sh

required (env)
  PROMPT_FILE  generation prompt (exact signature, forbid contract redefinition, raw code only)
  FITNESS_CMD  shell snippet run once per candidate; the candidate path arrives as $1.
               Must print one line: 'FITNESS pass=X/Y metric=N' (metric optional;
               'reduction=' accepted as an alias) or a bare BUILD_FAIL / TIMEOUT / CRASH.
  WORK         output dir: candidates in $WORK/$NAME/, results.tsv beside them,
               champion copied to $WORK/champion_$NAME.$EXT

optional (env)                                             [default]
  NAME / EXT   round label / candidate file extension      [round / rs]
  SOL / LUNA   candidate counts — the squadron leader      [6 / 12]
               owns this mix; size it to the round and
               to expected attrition
  SOL_MODEL  / SOL_EFFORT                                  [gpt-5.6-sol / high]
  LUNA_MODEL / LUNA_EFFORT                                 [gpt-5.6-luna / xhigh]
  FLEET        newline list of '<idx> <model> <effort>';   [built from SOL/LUNA]
               full override, ignores the knobs above
  PARALLEL     concurrent generations                      [3]
  CLAUDEX      backend CLI                                 [claudex on PATH, else ~/.local/bin/claudex]
EOF
}
case "${1:-}" in -h|--help) usage; exit 0;; esac
for v in PROMPT_FILE FITNESS_CMD WORK; do
  [ -n "${!v:-}" ] || { echo "run_fleet.sh: \$$v is not set (run_fleet.sh -h for usage)" >&2; exit 2; }
done
[ -r "$PROMPT_FILE" ] || { echo "run_fleet.sh: cannot read PROMPT_FILE '$PROMPT_FILE'" >&2; exit 2; }
CLAUDEX="${CLAUDEX:-$(command -v claudex || echo "$HOME/.local/bin/claudex")}"
[ -x "$CLAUDEX" ] || { echo "run_fleet.sh: backend '$CLAUDEX' not found — install claudex or set CLAUDEX" >&2; exit 2; }

NAME="${NAME:-round}" EXT="${EXT:-rs}" PARALLEL="${PARALLEL:-3}"
mkdir -p "$WORK/$NAME"
PROMPT="$(cat "$PROMPT_FILE")"
export WORK PROMPT NAME EXT CLAUDEX

gen() {  # $1 idx  $2 model  $3 effort
  "$CLAUDEX" -p --model "$2" --effort "$3" "$PROMPT" 2>/dev/null \
    | grep -vi 'connectors are disabled' | sed '/^```/d' > "$WORK/$NAME/cand_${1}.$EXT"
}
export -f gen

SOL="${SOL:-6}" LUNA="${LUNA:-12}"
SOL_MODEL="${SOL_MODEL:-gpt-5.6-sol}"    SOL_EFFORT="${SOL_EFFORT:-high}"
LUNA_MODEL="${LUNA_MODEL:-gpt-5.6-luna}" LUNA_EFFORT="${LUNA_EFFORT:-xhigh}"
FLEET="${FLEET:-$(
  for i in $(seq 1 "$SOL");  do printf '%02d %s %s\n' "$i" "$SOL_MODEL" "$SOL_EFFORT"; done
  for i in $(seq 1 "$LUNA"); do printf '%02d %s %s\n' $((SOL + i)) "$LUNA_MODEL" "$LUNA_EFFORT"; done
)}"

echo "[$WORK $NAME] commissioning $(grep -c . <<<"$FLEET") candidates ..."
grep . <<<"$FLEET" | xargs -P "$PARALLEL" -I{} bash -c 'set -- {}; gen "$1" "$2" "$3"'

shopt -s nullglob
: > "$WORK/$NAME/results.tsv"
for c in "$WORK/$NAME"/cand_*."$EXT"; do
  fit=$(bash -c "$FITNESS_CMD" fitness "$c")
  pass=$(grep -oE 'pass=[0-9]+/[0-9]+' <<<"$fit" | head -1)
  met=$(grep -oE '(metric|reduction)=[0-9]+' <<<"$fit" | head -1 | grep -oE '[0-9]+$')
  status=$(awk '{print $1; exit}' <<<"$fit")
  printf '%s\t%s\t%s\t%sL\n' "$(basename "$c" ".$EXT" | sed 's/^cand_//')" \
    "${pass:-${status:-NO_OUTPUT}}" "metric=${met:-NA}" "$(wc -l < "$c" | tr -d ' ')" \
    >> "$WORK/$NAME/results.tsv"
done
[ -s "$WORK/$NAME/results.tsv" ] || { echo "[$WORK $NAME] no candidates produced" >&2; exit 1; }

# rank: fitness numerator desc, metric desc, fewest lines
best=$(awk -F'\t' '{split($2,p,"[=/]"); split($3,m,"="); sub(/L$/,"",$4);
  print (p[2]+0)"\t"(m[2]+0)"\t"(-$4)"\t"$1}' "$WORK/$NAME/results.tsv" \
  | sort -k1,1nr -k2,2nr -k3,3nr | head -1 | cut -f4)

echo "[$WORK $NAME] ranked results:"
sort "$WORK/$NAME/results.tsv" | column -t -s $'\t'
cp "$WORK/$NAME/cand_${best}.$EXT" "$WORK/champion_$NAME.$EXT"
echo "PROVISIONAL_WINNER=$best  (champion at $WORK/champion_$NAME.$EXT)"
