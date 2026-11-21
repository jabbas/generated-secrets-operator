#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# The scenes of the Generated Secrets Operator demo: this is what gets recorded
# (see record.sh and README.md). It assumes the current kubectl context points
# at a cluster with the operator installed in `generated-secrets-system`, and it
# works in the namespace `demo`, which it creates.
#
# Every state change is awaited with `kubectl wait` or `kubectl rollout status`,
# each bounded by a timeout; a wait that times out stops the script with a
# non-zero exit status instead of recording a wrong outcome. `sleep` only paces
# the typing and the time to read the screen.

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly repo_root
readonly demo_namespace="demo"
readonly operator_namespace="generated-secrets-system"
readonly operator_deployment="generated-secrets-operator"
readonly wait_timeout="60s"
# Pacing; override for a faster dry run, e.g. TYPING_DELAY=0.
readonly typing_delay="${TYPING_DELAY:-0.011}"

readonly style_reset=$'\e[0m'
readonly style_bold=$'\e[1m'
readonly style_bold_green=$'\e[1;32m'
readonly style_dim_italic=$'\e[2;3m'

# Output of the last run_command, without the trailing newline.
last_output=""

# The demo shows kubectl output through kubecolor, like a common shell alias
# (alias kubectl=kubecolor): the typed `kubectl ...` is what runs, through this
# function, with colours forced because the output is captured. Output that is
# read as a value (`-o jsonpath=...`) stays plain: kubecolor would colour it,
# and the colour codes would end up inside the value.
kubectl() {
  local argument
  for argument in "$@"; do
    case "$argument" in
      jsonpath=* | --output=jsonpath=*)
        command kubectl "$@"
        return
        ;;
    esac
  done
  kubecolor --force-colors "$@"
}

fail() {
  echo "demo.sh: $*" >&2
  exit 1
}

# type_text <text>: print the text one character at a time, like a human typing.
type_text() {
  local text="$1" index
  for ((index = 0; index < ${#text}; index++)); do
    printf '%s' "${text:index:1}"
    sleep "$typing_delay"
  done
}

print_prompt() {
  printf '%s$ %s' "$style_bold_green" "$style_reset"
}

# type_comment <text>: a typed comment after a prompt, dim italic; not executed.
type_comment() {
  print_prompt
  printf '%s' "$style_dim_italic"
  type_text "# $1"
  printf '%s\n' "$style_reset"
  sleep 0.4
}

# run_command <command>: show the command being typed after a prompt, run it,
# print its output and keep that in last_output. A failing command stops the demo.
# What is typed is exactly what runs (eval of the same text).
run_command() {
  local command_text="$1"
  print_prompt
  type_text "$command_text"
  printf '\n'
  sleep 0.3
  if ! last_output="$(eval "$command_text" 2>&1)"; then
    printf '%s\n' "$last_output"
    fail "command failed: $command_text"
  fi
  if [[ -n "$last_output" ]]; then
    printf '%s\n' "$last_output"
  fi
  sleep 0.6
}

# verdict <text>: the outcome of a scene, in green.
verdict() {
  printf '%s%s%s\n' "$style_bold_green" "$1" "$style_reset"
  sleep 1.2
}

# expect_same / expect_different <before> <after> <text>: verdict only if true.
expect_same() {
  if [[ "$1" != "$2" ]]; then
    fail "expected identical values, got '$1' and '$2'"
  fi
  verdict "$3"
}

expect_different() {
  if [[ "$1" == "$2" ]]; then
    fail "expected different values, both were '$1'"
  fi
  verdict "$3"
}

start_scene() {
  clear
  printf '%s%s%s\n\n' "$style_bold" "$1" "$style_reset"
}

# manifest_command <file>: the one definition of the `yq` command that shows an
# example manifest with YAML colours (keys cyan, values green, comments grey),
# without the comment header of the example file. Both the first frame and
# show_manifest use it, so what is shown and what runs cannot drift. The file
# name is shell-quoted (%q) so that it stays one argument when the text is eval'd.
readonly manifest_command_prefix="yq --colors '. head_comment=\"\"'"
manifest_command() {
  printf '%s %q' "$manifest_command_prefix" "$1"
}

# show_manifest <file>: type and run the manifest_command for the file.
show_manifest() {
  run_command "$(manifest_command "$1")"
}

# --- first frame: printed instantly, never an empty terminal ---
# It comes before the preflight checks below, so the recording has no blank
# start while kubectl talks to the cluster. The tools are checked first; they
# are quick and the first frame needs yq.

for tool in kubectl kubecolor yq; do
  type -P "$tool" > /dev/null || fail "$tool not found on PATH"
done
cd "$repo_root"
clear
printf '%sGenerated Secrets Operator: Kubernetes Secrets generated in-cluster%s\n\n' \
  "$style_bold" "$style_reset"
# The same command as show_manifest, run instead of typed.
eval "$(manifest_command examples/app-admin-password.yaml)"

# --- preflight and setup (while the first frame is on screen) ---

# --ignore-not-found separates "absent" (empty output, success) from a real
# error (unreachable API server, expired credentials, 403): the latter fails.
# `command kubectl` skips the kubecolor function above.
if ! existing_deployment="$(command kubectl get deployment "$operator_deployment" --namespace "$operator_namespace" --ignore-not-found --output name)"; then
  fail "could not check whether deployment $operator_deployment exists in namespace $operator_namespace; see the kubectl error above"
fi
if [[ -z "$existing_deployment" ]]; then
  fail "deployment $operator_deployment not found in namespace $operator_namespace: install the operator first"
fi
if ! existing_namespace="$(command kubectl get namespace "$demo_namespace" --ignore-not-found --output name)"; then
  fail "could not check whether namespace $demo_namespace exists; see the kubectl error above"
fi
if [[ -n "$existing_namespace" ]]; then
  fail "namespace $demo_namespace already exists; delete it first: kubectl delete namespace $demo_namespace"
fi
command kubectl create namespace "$demo_namespace" > /dev/null
sleep 4

# --- scene 1: a deleted GeneratedSecret keeps its password (deletionPolicy: Retain) ---

start_scene "1. Delete the GeneratedSecret, keep the password"
show_manifest examples/app-admin-password.yaml
run_command "kubectl apply -f examples/app-admin-password.yaml -n $demo_namespace"
run_command "kubectl wait --for=condition=Ready gensecret/app-admin -n $demo_namespace --timeout=$wait_timeout"
run_command "kubectl get secret app-admin -n $demo_namespace -o jsonpath='{.data.password}' | base64 -d; echo"
password_before="$last_output"
type_comment "deletionPolicy: Retain keeps the Secret when the GeneratedSecret goes away"
run_command "kubectl delete gensecret app-admin -n $demo_namespace"
run_command "kubectl get secret app-admin -n $demo_namespace"
run_command "kubectl apply -f examples/app-admin-password.yaml -n $demo_namespace"
run_command "kubectl wait --for=condition=Ready gensecret/app-admin -n $demo_namespace --timeout=$wait_timeout"
run_command "kubectl get secret app-admin -n $demo_namespace -o jsonpath='{.data.password}' | base64 -d; echo"
expect_same "$password_before" "$last_output" "Same password after re-creating the GeneratedSecret"

# --- scene 2: the operator restarts, the password stays ---

start_scene "2. Restart the operator, keep the password"
run_command "kubectl rollout restart deployment/$operator_deployment -n $operator_namespace"
run_command "kubectl rollout status deployment/$operator_deployment -n $operator_namespace --timeout=$wait_timeout"
run_command "kubectl get secret app-admin -n $demo_namespace -o jsonpath='{.data.password}' | base64 -d; echo"
expect_same "$password_before" "$last_output" "Same password after an operator restart"

# --- scene 3: a deleted SSH key Secret gets a new key pair ---

start_scene "3. Delete the Secret, get a new SSH key pair"
show_manifest examples/ssh-deploy-key.yaml
run_command "kubectl apply -f examples/ssh-deploy-key.yaml -n $demo_namespace"
run_command "kubectl wait --for=condition=Ready gensecret/deploy-key -n $demo_namespace --timeout=$wait_timeout"
run_command "kubectl get secret deploy-key -n $demo_namespace -o jsonpath='{.data.ssh-publickey}' | base64 -d"
public_key_before="$last_output"
run_command "kubectl delete secret deploy-key -n $demo_namespace"
run_command "kubectl wait --for=create secret/deploy-key -n $demo_namespace --timeout=$wait_timeout"
run_command "kubectl get secret deploy-key -n $demo_namespace -o jsonpath='{.data.ssh-publickey}' | base64 -d"
expect_different "$public_key_before" "$last_output" "A new key pair, generated by the operator"

# --- scene 4: changing the definition regenerates the value ---

start_scene "4. Change the definition, get a new token"
show_manifest examples/rotated-shared-token.yaml
run_command "kubectl apply -f examples/rotated-shared-token.yaml -n $demo_namespace"
run_command "kubectl wait --for=condition=Ready gensecret/shared-token -n $demo_namespace --timeout=$wait_timeout"
run_command "kubectl get secret shared-token -n $demo_namespace -o jsonpath='{.data.token}' | base64 -d; echo"
token_before="$last_output"
if [[ ${#token_before} -ne 48 ]]; then
  fail "expected a 48-character token before the patch, got ${#token_before}"
fi
# The patch raises metadata.generation by one; the operator reports it in
# status.observedGeneration once the new value is applied.
if ! generation_before="$(command kubectl get gensecret shared-token --namespace "$demo_namespace" --output 'jsonpath={.metadata.generation}')"; then
  fail "could not read the generation of shared-token"
fi
if [[ ! "$generation_before" =~ ^(0|[1-9][0-9]*)$ ]]; then
  fail "expected the generation of shared-token to be a non-negative integer, got '$generation_before'"
fi
generation_after=$((generation_before + 1))
type_comment "token length 48 -> 64"
run_command "kubectl patch gensecret shared-token -n $demo_namespace --type merge -p '{\"spec\":{\"fields\":[{\"name\":\"token\",\"length\":64}]}}'"
run_command "kubectl wait --for=jsonpath='{.status.observedGeneration}'=$generation_after gensecret/shared-token -n $demo_namespace --timeout=$wait_timeout"
run_command "kubectl get secret shared-token -n $demo_namespace -o jsonpath='{.data.token}' | base64 -d; echo"
if [[ ${#last_output} -ne 64 ]]; then
  fail "expected a 64-character token after the patch, got ${#last_output}"
fi
expect_different "$token_before" "$last_output" "A new, longer token"
run_command "kubectl get events -n $demo_namespace --field-selector reason=Regenerated"
if [[ "$last_output" != *Regenerated* ]]; then
  fail "no Regenerated event for shared-token"
fi

# --- closing frame ---

sleep 1
printf '\n'
type_comment "github.com/jabbas/generated-secrets-operator"
sleep 3
