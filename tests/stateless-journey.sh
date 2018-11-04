#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots"
fixture="$root/fixtures"

SUCCESSFULLY=0

(with "input from stdin"
  (when "the input is well formed and there is a clear winner"
    it "produces the expected output" && {
      WITH_SNAPSHOT="$snapshot/success-input-file-produces-correct-output" \
      expect_run ${SUCCESSFULLY} "$exe" < "$fixture/valid.input"
    }
  )
  (when "the input is well formed and there is no a clear winner"
    it "produces the expected output" && {
      WITH_SNAPSHOT="$snapshot/success-input-file-produces-correct-output-no-winner" \
      expect_run ${SUCCESSFULLY} "$exe" < "$fixture/valid-runoff.input"
    }
  )
  (when "the input is well formed and huge"
    it "produces the expected output" && {
      WITH_SNAPSHOT="$snapshot/success-input-file-produces-correct-output-huge" \
      expect_run ${SUCCESSFULLY} "$exe" < "$fixture/valid-big.input"
    }
  )
)
