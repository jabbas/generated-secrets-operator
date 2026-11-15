#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Hermetic test of show_log in registry-lookup.sh: no line of the output may
# start with `::` (a workflow command), also after a carriage return.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source-path=SCRIPTDIR source=registry-lookup.sh
source "$here/registry-lookup.sh"

log="$(mktemp)"
chart_dest="$(mktemp -d)"
trap 'rm -f "$log"; rm -rf "$chart_dest"' EXIT

printf 'Error: boom\n::error::injected\r::warning::also injected\nlast line' > "$log"
out="$(show_log "$log")"

if grep -q '^::' <<<"$out"; then
  echo "FAIL: show_log output has a line that starts with ::" >&2
  echo "$out" >&2
  exit 1
fi
for needle in 'Error: boom' '::error::injected' '::warning::also injected' 'last line'; do
  if ! grep -qF -- "$needle" <<<"$out"; then
    echo "FAIL: show_log lost the text '$needle'" >&2
    exit 1
  fi
done
echo "PASS: show_log prefixes every line, carriage returns included, and keeps the text"

# A package that was never pushed (the first release of a renamed package) is
# answered with NAME_UNKNOWN or with an access error. The registry client is
# replaced by stubs that fail with the answer held in registry_answer.
registry_answer=""
docker() {
  printf '%s\n' "$registry_answer" >&2
  return 1
}
helm() {
  printf '%s\n' "$registry_answer"
  return 1
}

# expect_digest <description> <registry answer> <mode> <expected output>
expect_digest() {
  local description="$1" mode="$3" expected="$4" got
  registry_answer="$2"
  got="$(tag_digest "ghcr.io/example/package:1.0.0" "$mode" 2> /dev/null)" || {
    echo "FAIL: $description: tag_digest returned an error" >&2
    exit 1
  }
  if [[ "$got" != "$expected" ]]; then
    echo "FAIL: $description: expected '$expected', got '$got'" >&2
    exit 1
  fi
  echo "PASS: $description"
}

# expect_digest_error <description> <registry answer> <mode>
expect_digest_error() {
  local description="$1" mode="$3"
  registry_answer="$2"
  if tag_digest "ghcr.io/example/package:1.0.0" "$mode" > /dev/null 2>&1; then
    echo "FAIL: $description: tag_digest succeeded, it must fail" >&2
    exit 1
  fi
  echo "PASS: $description"
}

# expect_chart_state <description> <registry answer> <expected state>
expect_chart_state() {
  local description="$1" expected="$3"
  registry_answer="$2"
  CHART_STATE=""
  chart_state "oci://ghcr.io/example/charts/package" "1.0.0" "$chart_dest" "$log" 2> /dev/null || {
    echo "FAIL: $description: chart_state returned an error" >&2
    exit 1
  }
  if [[ "$CHART_STATE" != "$expected" ]]; then
    echo "FAIL: $description: expected CHART_STATE=$expected, got $CHART_STATE" >&2
    exit 1
  fi
  echo "PASS: $description"
}

name_unknown='ERROR: failed to resolve: NAME_UNKNOWN: repository name not known to registry'
access_denied='ERROR: denied: requested access to the resource is denied'

expect_digest "lenient: NAME_UNKNOWN counts as no published version" "$name_unknown" lenient ""
expect_digest "lenient: denied for a missing package counts as no published version" "$access_denied" lenient ""
expect_digest_error "strict: NAME_UNKNOWN is never absent" "$name_unknown" strict
expect_digest_error "strict: denied is never absent" "$access_denied" strict
expect_chart_state "chart_state: NAME_UNKNOWN is absent" "$name_unknown" absent
expect_chart_state "chart_state: denied stays denied, not absent" "$access_denied" denied

echo "all registry-lookup tests passed"
