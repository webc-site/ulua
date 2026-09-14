#!/usr/bin/env bash
# Compile + run every harvested script through ulua and report the ones that
# CRASH or HANG. The standalone `run` / `compile` targets treat compile errors
# and runtime errors as normal outcomes (exit 0); only a panic / assertion /
# abort / hang is a finding, so a non-zero exit == a bug reproducer.
#
#   OUT=/tmp/luau-corpus scripts/run-corpus.sh            # run + compile scan
#   scripts/run-corpus.sh /path/to/corpus
#
# Findings (with the offending file) go to $OUT/FINDINGS.tsv, grouped by target
# and exit signal. A low VM step cap keeps finite-but-long scripts fast; a
# per-file wall-clock timeout catches uninterruptible C-loop hangs.
set -uo pipefail

CORPUS="${1:-${OUT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/fuzz/corpus-harvest}}"
FUZZ="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/fuzz"
STEPS="${ULUA_FUZZ_STEPS:-200000}"
TIMEOUT="${TIMEOUT:-15}"
FINDINGS="$CORPUS/FINDINGS.tsv"
: > "$FINDINGS"

echo ">> building standalone run + compile targets"
( cd "$FUZZ" && cargo build --no-default-features --bin run --bin compile 2>&1 | grep -E '^error' ) && exit 1

mapfile -t files < <(find "$CORPUS" -name '*.luau' -type f)
echo ">> scanning ${#files[@]} scripts (steps=$STEPS, timeout=${TIMEOUT}s/file)"

scan() { # $1 = target binary name
  local bin="$FUZZ/target/debug/$1" n=0 bad=0
  for f in "${files[@]}"; do
    ULUA_FUZZ_STEPS="$STEPS" timeout "$TIMEOUT" "$bin" "$f" >/dev/null 2>&1
    local ec=$?
    if [[ $ec -ne 0 ]]; then
      local kind
      case $ec in
        124) kind="HANG" ;;
        134) kind="ABORT/assert" ;;
        139) kind="SIGSEGV" ;;
        1)   kind="panic" ;;
        *)   kind="exit:$ec" ;;
      esac
      printf '%s\t%s\t%s\n' "$1" "$kind" "$f" >> "$FINDINGS"
      bad=$((bad+1))
    fi
    n=$((n+1))
    (( n % 500 == 0 )) && echo "   $1: $n/${#files[@]} ($bad findings)"
  done
  echo "   $1: done, $bad findings"
}

scan compile
scan run

echo "==> findings:"
if [[ -s "$FINDINGS" ]]; then
  cut -f1,2 "$FINDINGS" | sort | uniq -c | sort -rn
  echo "(full list with files: $FINDINGS)"
else
  echo "   none — no crashes or hangs across the corpus"
fi
