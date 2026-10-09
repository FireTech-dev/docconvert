#!/usr/bin/env bash
# USER/CI ONLY: run libFuzzer targets in resumable time chunks until a total
# CPU-time budget per target is met. Crash/oom/timeout artifacts stop everything
# immediately and are never skipped over. State survives kills: re-run the same
# command to resume toward the budget.
#
# Usage:
#   scripts/fuzz_batch.sh [--targets rtf,pdf,csv] [--hours 24] [--chunk-min 60]
#                         [--logs fuzz/logs] [--reset] [--dry-run]
# Examples:
#   scripts/fuzz_batch.sh                                  # 24h x 3 targets, 60-min chunks
#   scripts/fuzz_batch.sh --targets pdf --hours 24         # one target only
#   scripts/fuzz_batch.sh --hours 6 --chunk-min 30         # short gate / CI slices
#   scripts/fuzz_batch.sh --reset --targets rtf            # drop resume state, keep corpus
set -u
cd "$(dirname "$0")/.." || exit 1

TARGETS="rtf,pdf,csv"
HOURS="24"
CHUNK_MIN="60"
LOGDIR="fuzz/logs"
RESET=0
DRYRUN=0
while [ $# -gt 0 ]; do
  case "$1" in
    --targets) TARGETS="$2"; shift 2;;
    --hours) HOURS="$2"; shift 2;;
    --chunk-min) CHUNK_MIN="$2"; shift 2;;
    --logs) LOGDIR="$2"; shift 2;;
    --reset) RESET=1; shift;;
    --dry-run) DRYRUN=1; shift;;
    -h|--help) sed -n '2,14p' "$0"; exit 0;;
    *) echo "unknown flag: $1 (see --help)" >&2; exit 2;;
  esac
done

is_posnum() { awk -v v="$1" 'BEGIN{exit !(v+0>0 && v==v+0)}'; }
is_posnum "$HOURS" || { echo "--hours must be a positive number (hours, fractions allowed)" >&2; exit 2; }
is_posnum "$CHUNK_MIN" || { echo "--chunk-min must be a positive number (minutes, fractions allowed)" >&2; exit 2; }
TOTAL_SECS=$(awk -v h="$HOURS" 'BEGIN{printf "%.0f", h*3600}')
CHUNK_SECS=$(awk -v m="$CHUNK_MIN" 'BEGIN{printf "%.0f", m*60}')
# A 0-second chunk would mean "no limit" to libFuzzer (-max_total_time=0) and
# fuzz forever: refuse tiny values loudly instead of hanging.
[ "$TOTAL_SECS" -ge 30 ] || { echo "--hours too small (minimum 30 seconds total)" >&2; exit 2; }
[ "$CHUNK_SECS" -ge 30 ] || { echo "--chunk-min too small (minimum 30 seconds per chunk)" >&2; exit 2; }

command -v cargo >/dev/null || { echo "cargo not found" >&2; exit 1; }
cargo +nightly fuzz --version >/dev/null 2>&1 || { echo "nightly toolchain with cargo-fuzz 0.13.x required (rustup toolchain install nightly; cargo +nightly fuzz --version)" >&2; exit 1; }
[ -d fuzz/fuzz_targets ] || { echo "run from the repo root (fuzz/fuzz_targets missing)" >&2; exit 1; }
mkdir -p "$LOGDIR" fuzz/.batch-state

STAMP=$(date +%Y%m%d-%H%M%S)
LOG="$LOGDIR/fuzz-batch-$STAMP.log"
echo "budget=${HOURS}h/target chunk=${CHUNK_MIN}min log=$LOG" | tee "$LOG"

OLDIFS="$IFS"; IFS=','
# shellcheck disable=SC2086
set -- $TARGETS
IFS="$OLDIFS"
for short in "$@"; do
  tgt="fuzz_$short"
  [ -f "fuzz/fuzz_targets/$tgt.rs" ] || { echo "unknown target: $short" >&2; exit 2; }
  state="fuzz/.batch-state/$tgt.secs"
  runs_state="fuzz/.batch-state/$tgt.runs"
  [ "$RESET" = "1" ] && rm -f "$state" "$runs_state"
  acc=0; runs=0
  [ -f "$state" ] && acc=$(cat "$state")
  [ -f "$runs_state" ] && runs=$(cat "$runs_state")
  echo "== target=$tgt accumulated=${acc}s runs=$runs ==" | tee -a "$LOG"
  if [ "$DRYRUN" = "1" ]; then continue; fi
  while [ "$acc" -lt "$TOTAL_SECS" ]; do
    left=$((TOTAL_SECS - acc))
    this=$CHUNK_SECS; [ "$this" -gt "$left" ] && this=$left
    before=$(find "fuzz/artifacts/$tgt" -maxdepth 1 \( -name 'crash-*' -o -name 'oom-*' -o -name 'timeout-*' -o -name 'leak-*' \) 2>/dev/null | wc -l)
    start=$(date +%s)
    echo "-- chunk ${this}s (acc ${acc}s/${TOTAL_SECS}s) --" | tee -a "$LOG"
    # shellcheck disable=SC2086
    if (cd fuzz && cargo +nightly fuzz run "$tgt" -- -max_total_time="$this" -print_final_stats=1 >>"../$LOG" 2>&1); then
      end=$(date +%s); acc=$((acc + end - start))
      units=$(grep -a -o 'stat::number_of_executed_units: *[0-9]*' "$LOG" | tail -n 1 | grep -a -o '[0-9]*$')
      [ -n "$units" ] && runs=$((runs + units))
      echo "$acc" >"$state"; echo "$runs" >"$runs_state"
      echo "chunk ok: acc=${acc}s runs=$runs corpus=$(find fuzz/corpus/$tgt -type f 2>/dev/null | wc -l)" | tee -a "$LOG"
    else
      end=$(date +%s); acc=$((acc + end - start))
      echo "$acc" >"$state"; echo "$runs" >"$runs_state"
      after=$(find "fuzz/artifacts/$tgt" -maxdepth 1 \( -name 'crash-*' -o -name 'oom-*' -o -name 'timeout-*' -o -name 'leak-*' \) 2>/dev/null | wc -l)
      echo "CHUNK FAILED (exit nonzero). artifacts before=$before after=$after" | tee -a "$LOG"
      find "fuzz/artifacts/$tgt" -maxdepth 1 -newer "$LOG" 2>/dev/null | tee -a "$LOG"
      echo "STOP: triage the artifact above before continuing. State kept; re-run the same command to resume." | tee -a "$LOG"
      exit 1
    fi
  done
  cpuh=$(awk -v a="$acc" 'BEGIN{printf "%.2f", a/3600}')
  echo "TARGET DONE: $tgt ${acc}s (${cpuh} CPU-h) runs=$runs corpus=$(find fuzz/corpus/$tgt -type f 2>/dev/null | wc -l)" | tee -a "$LOG"
done
echo "ALL TARGETS COMPLETE. Evidence lines for agent.md:" | tee -a "$LOG"
for short in $(echo "$TARGETS" | tr ',' ' '); do
  tgt="fuzz_$short"
  a=$(cat "fuzz/.batch-state/$tgt.secs" 2>/dev/null || echo 0)
  r=$(cat "fuzz/.batch-state/$tgt.runs" 2>/dev/null || echo 0)
  c=$(find "fuzz/corpus/$tgt" -type f 2>/dev/null | wc -l)
  f=$(find "fuzz/artifacts/$tgt" -maxdepth 1 \( -name 'crash-*' -o -name 'oom-*' -o -name 'timeout-*' -o -name 'leak-*' \) 2>/dev/null | wc -l)
  echo "$tgt: ${a}s ($(awk -v a="$a" 'BEGIN{printf "%.2f", a/3600}') CPU-h), $r runs, corpus=$c, findings=$f" | tee -a "$LOG"
done
exit 0
