#!/usr/bin/env bash
set -u

model=
while (($#)); do
  case "$1" in
    --model) model="$2"; shift 2;;
    *) shift;;
  esac
done

case "$model" in
  fast-pass) printf 'PASS\n';;
  fast-fail) printf 'FAIL\n';;
  slow-pass) sleep 8; printf 'PASS\n';;
  *) exit 3;;
esac
