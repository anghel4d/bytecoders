#!/usr/bin/env bash
# Compose prelude + mod a{A} + mod b{B} + mod c{C} + testmain, compile, run the fitness battery.
# One candidate occupies its stage's slot; the other two slots hold golden or frozen-winner sources.
# Module wrapping isolates each stage's `use` imports so two real winners never collide (E0252).
#
# Usage:  CONTEST_ROOT=/path/to/harness  compose_and_test.sh <A.rs> <B.rs> <C.rs>
# Needs:  $CONTEST_ROOT/prelude.rs and $CONTEST_ROOT/testmain.rs ; rustc on PATH.
# Emits one line:  FITNESS pass=X/N in_ops=.. out_ops=.. reduction=..  | BUILD_FAIL | TIMEOUT | CRASH rc=N
set -u
ROOT="${CONTEST_ROOT:?set CONTEST_ROOT to the harness dir (holds prelude.rs, testmain.rs)}"
A="$1"; B="$2"; C="$3"
work=$(mktemp -d "${TMPDIR:-/tmp}/vmcomp.XXXXXX")
src="$work/m.rs"
{
  cat "$ROOT/prelude.rs"
  echo 'mod a { #[allow(unused_imports)] use super::Op::{self, *};'; cat "$A"; echo '}'
  echo 'mod b { #[allow(unused_imports)] use super::Op::{self, *};'; cat "$B"; echo '}'
  echo 'mod c { #[allow(unused_imports)] use super::Op::{self, *};'; cat "$C"; echo '}'
  cat "$ROOT/testmain.rs"
} > "$src"

if ! rustc --edition 2021 -O -A warnings -o "$work/bin" "$src" 2>"$work/err"; then
  echo "BUILD_FAIL"; rm -rf "$work"; exit 0
fi
out=$(timeout 5 "$work/bin" 2>/dev/null); rc=$?
if [ $rc -eq 124 ]; then
  echo "TIMEOUT"
elif echo "$out" | grep -q FITNESS; then
  echo "$out"
else
  echo "CRASH rc=$rc"
fi
rm -rf "$work"
