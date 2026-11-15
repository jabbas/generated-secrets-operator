#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Re-records the demo with one command:
#
#   bash demo/record.sh <chart-version>      e.g. bash demo/record.sh 0.2.0
#
# It creates a throwaway kind cluster, installs the released chart, records
# demo/demo.sh with asciinema and renders a GIF with agg. Outputs:
#   docs/assets/demo.cast
#   docs/assets/demo.gif
# The cluster is deleted when the script ends, whatever the outcome. The kind
# provider (docker or podman) comes from the environment, e.g.
# KIND_EXPERIMENTAL_PROVIDER=podman; this script does not set it.

set -euo pipefail

readonly cluster_name="generated-secrets-demo"
readonly operator_namespace="generated-secrets-system"
readonly chart_reference="oci://ghcr.io/jabbas/charts/generated-secrets-operator"
readonly terminal_size="110x34"
readonly idle_time_limit_seconds="2"
readonly recording_title="Generated Secrets Operator demo"

usage() {
  echo "usage: bash demo/record.sh <chart-version>   (e.g. 0.2.0 or 0.2.0-rc.2)" >&2
}

if [[ $# -ne 1 ]]; then
  usage
  exit 2
fi
chart_version="$1"
readonly chart_version
if [[ ! "$chart_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
  echo "record.sh: '$chart_version' is not a chart version like 0.2.0 or 0.2.0-rc.2" >&2
  usage
  exit 2
fi

for tool in kind helm kubectl asciinema agg; do
  if ! command -v "$tool" > /dev/null; then
    echo "$tool not found on PATH" >&2
    exit 1
  fi
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly repo_root
cd "$repo_root"

# Refuse before creating anything: never touch a cluster this script did not create.
if ! existing_clusters="$(kind get clusters)"; then
  echo "record.sh: 'kind get clusters' failed; see the message above" >&2
  exit 1
fi
if grep -qx "$cluster_name" <<< "$existing_clusters"; then
  echo "record.sh: a kind cluster named $cluster_name already exists." >&2
  echo "It was not created by this run, so it is left alone. Delete it yourself if it is stale:" >&2
  echo "  kind delete cluster --name $cluster_name" >&2
  exit 1
fi

work_directory="$(mktemp -d)"
readonly work_directory
cluster_created=false

# Runs on every exit: deletes only the cluster this run created, and the
# scratch directory. A failed cleanup is reported but keeps the original status.
cleanup() {
  local exit_status=$?
  if [[ "$cluster_created" == true ]]; then
    # kind removes a half-created cluster itself, so delete only if it is listed.
    if ! remaining_clusters="$(kind get clusters)"; then
      echo "record.sh: could not list kind clusters; if $cluster_name exists, delete it by hand" >&2
    elif grep -qx "$cluster_name" <<< "$remaining_clusters"; then
      echo "Deleting kind cluster $cluster_name" >&2
      if ! kind delete cluster --name "$cluster_name" --kubeconfig "$work_directory/kubeconfig"; then
        echo "record.sh: could not delete kind cluster $cluster_name; delete it by hand" >&2
      fi
    fi
  fi
  rm -rf "$work_directory"
  exit "$exit_status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# A private kubeconfig keeps the user's own kubeconfig and current context untouched.
export KUBECONFIG="$work_directory/kubeconfig"

echo "Creating kind cluster $cluster_name"
cluster_created=true # set first: anything that exists after this point is ours
kind create cluster --name "$cluster_name" --wait 120s

echo "Installing generated-secrets-operator $chart_version"
kubectl create namespace "$operator_namespace"
kubectl label namespace "$operator_namespace" pod-security.kubernetes.io/enforce=restricted
helm install generated-secrets-operator "$chart_reference" \
  --version "$chart_version" \
  --namespace "$operator_namespace" \
  --create-namespace \
  --wait \
  --timeout 5m

echo "Recording demo/demo.sh"
# The command is relative to the repository root, so no local path ends up in the recording.
asciinema rec "$work_directory/demo.cast" \
  --command "bash demo/demo.sh" \
  --window-size "$terminal_size" \
  --idle-time-limit "$idle_time_limit_seconds" \
  --title "$recording_title" \
  --return \
  --overwrite

echo "Rendering the GIF"
agg "$work_directory/demo.cast" "$work_directory/demo.gif" \
  --theme github-dark \
  --font-size 14 \
  --idle-time-limit "$idle_time_limit_seconds" \
  --fps-cap 10 \
  --quiet \
  --last-frame-duration 4

# Both files were recorded in a scratch directory and are moved only now: a
# failed run must not overwrite the committed outputs.
mkdir -p docs/assets
mv "$work_directory/demo.cast" docs/assets/demo.cast
mv "$work_directory/demo.gif" docs/assets/demo.gif
echo "Wrote docs/assets/demo.cast and docs/assets/demo.gif"
