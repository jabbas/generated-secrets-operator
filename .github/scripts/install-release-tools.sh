#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Installs pinned release binaries on a GitHub Actions linux/amd64 runner:
#
#   bash .github/scripts/install-release-tools.sh syft grype crane oras
#
# Used by the `image`, `promote` and `helm-index` jobs of release.yml (scan,
# SBOM, registry access), by artifacthub-metadata.yml (oras) and by the
# `release-pins` job of ci.yml, which runs the same script on every pull
# request, so a pin that does not verify is found there and not at the next
# release candidate.
#
# This file is the one place for these pins. Each archive is verified with
# `sha256sum --check --strict` against the SHA-256 of the linux_amd64 archive,
# taken from the checksums file of that upstream release: `*_checksums.txt`
# for syft, grype and oras (`oras_<version>_checksums.txt`), `checksums.txt`
# for go-containerregistry (crane). Bump
# version and checksum together. Renovate proposes the versions (the
# `# renovate:` comments are read by renovate.json) but cannot compute the
# checksums: a version-only bump fails the check, and the error names the
# variable to update. The script only downloads, verifies and unpacks; it
# runs nothing it downloaded. cosign is not here: sigstore/cosign-installer
# installs it (COSIGN_VERSION in release.yml) and verifies the download itself.
set -euo pipefail

# renovate: datasource=github-releases depName=anchore/syft extractVersion=^v(?<version>.+)$
SYFT_VERSION=1.57.1
SYFT_SHA256=bf52519eb57f675d47c07d43213cb6dff6449f3d6d1147ff6701ed91f44eef0c

# renovate: datasource=github-releases depName=anchore/grype extractVersion=^v(?<version>.+)$
GRYPE_VERSION=0.121.3
GRYPE_SHA256=e9da18cc1429e786722e32161288103600efffd8e483c90761f90485bd1bec85

# renovate: datasource=github-releases depName=google/go-containerregistry extractVersion=^v(?<version>.+)$
CRANE_VERSION=0.22.1
CRANE_SHA256=0ab7a1d6932a213aed964ce97666c3077fe691c8606413674a8b3e0b9ec4cda0

# renovate: datasource=github-releases depName=oras-project/oras extractVersion=^v(?<version>.+)$
ORAS_VERSION=1.3.6
ORAS_SHA256=66fb0ad7206f3f6fa77ed5b552049d364ccf4b50b1684554f731f76302b52ca0

if [[ $# -eq 0 ]]; then
  echo "usage: install-release-tools.sh <syft|grype|crane|oras>... | all" >&2
  exit 2
fi
# `all` is every tool of this file: the release-pins job of ci.yml uses it, so
# a tool added here is checked there without touching that job.
if [[ "$*" == all ]]; then
  set -- syft grype crane oras
fi

temp="${RUNNER_TEMP:?RUNNER_TEMP is not set (this script is for Actions runners)}"
github_path="${GITHUB_PATH:?GITHUB_PATH is not set (this script is for Actions runners)}"
dir="$temp/tools/bin"
mkdir -p "$dir"

for tool in "$@"; do
  case "$tool" in
    syft)
      prefix=SYFT
      version="$SYFT_VERSION"
      checksum="$SYFT_SHA256"
      archive="syft_${version}_linux_amd64.tar.gz"
      url="https://github.com/anchore/syft/releases/download/v${version}/${archive}"
      ;;
    grype)
      prefix=GRYPE
      version="$GRYPE_VERSION"
      checksum="$GRYPE_SHA256"
      archive="grype_${version}_linux_amd64.tar.gz"
      url="https://github.com/anchore/grype/releases/download/v${version}/${archive}"
      ;;
    crane)
      prefix=CRANE
      version="$CRANE_VERSION"
      checksum="$CRANE_SHA256"
      archive="go-containerregistry_Linux_x86_64.tar.gz"
      url="https://github.com/google/go-containerregistry/releases/download/v${version}/${archive}"
      ;;
    oras)
      prefix=ORAS
      version="$ORAS_VERSION"
      checksum="$ORAS_SHA256"
      archive="oras_${version}_linux_amd64.tar.gz"
      url="https://github.com/oras-project/oras/releases/download/v${version}/${archive}"
      ;;
    *)
      printf '::error::install-release-tools.sh does not know the tool %q (syft, grype, crane, oras)\n' "$tool" >&2
      exit 2
      ;;
  esac

  curl -fsSL --retry 3 -o "$temp/$archive" "$url"
  if ! echo "$checksum  $temp/$archive" | sha256sum --check --strict -; then
    echo "::error::$tool $version ($archive) does not match the pinned checksum; update ${prefix}_SHA256 (and ${prefix}_VERSION) together in .github/scripts/install-release-tools.sh, from the checksums file of the upstream release (docs/development.md, \"Pinned tools and Renovate\")" >&2
    exit 1
  fi
  tar -xzf "$temp/$archive" -C "$dir" "$tool"
done

echo "$dir" >> "$github_path"
