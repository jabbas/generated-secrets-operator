# Artifact Hub Listing — Design

| | |
|---|---|
| **Status** | Approved by owner (2026-11-15) |
| **Date** | 2026-11-15 |
| **Depends on** | Rename design (`docs/design/2026-11-15-rename-to-generated-secrets-operator-design.md`) merged; registration on Artifact Hub after the first final release carrying the annotations (§5) |

> **Decisions reviewed and approved by the owner on 2026-11-15**
>
> 1. List only the OCI repository, not the HTTP repository (§2).
> 2. Hide release candidates through the `ignore` rule (§4).
> 3. Do not use the `artifacthub.io/changes` annotation (§3).
> 4. An agent pushes the repository metadata file once; the release workflow does not push it (§4).
> 5. The `images` annotation is bumped by hand with the other version fields and checked by the chart test; the release pipeline and ADR 0005 stay unchanged (§3).
> 6. Annotations go into 0.2.0 if they can be merged before `0.2.0-rc.1`; otherwise into the next regular release, and Artifact Hub registration waits for it (§5).

**Conventions.** The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are normative (RFC 2119 sense).
Every decision carries its origin tag:

- **[owner]** — decided by the project owner.
- **[architect]** — architect's call; the owner may override it during review.

Section 7 lists what is deliberately left to the builder.

---

## 1. Intent

- This is step 3 of the promotion campaign **[owner]**: rename → repository ready for strangers → Artifact Hub listing → website → Show HN.
- Why: Artifact Hub is where Kubernetes users search for Helm charts. A search for "secret generator" today returns no well-starred, verified chart; the only result is mittwald's chart, unverified, with 0 stars, under a repository named kubernetes-replicator. A listing gives steady discovery, unlike a one-off Hacker News spike.
- Goal: the chart appears as a verified-publisher package with the "Signed" badge, a rendered README, custom resource definition (CRD) cards with an example, license, links and a security scan.

## 2. Which repository to list **[architect]**

- List only the Open Container Initiative (OCI) repository `oci://ghcr.io/jabbas/charts/generated-secrets-operator`. Do not list the HTTP repository `https://jabbas.github.io/helm-charts`.
- Reasons:
  - Artifact Hub shows the "Signed" badge for cosign signatures only on OCI charts: it detects cosign `.sig` tags and sigstore bundle referrers. For HTTP repositories it only checks Helm provenance (`.prov`, PGP) files, which this project does not produce. Source: artifacthub/hub `internal/oci/oci.go` and `internal/tracker/source/helm/helm.go`.
  - The HTTP repository also contains the owner's unrelated charts (for example the homepage chart). Listing it would publish all of them. `artifacthub-repo.yml` has only an `ignore` list (no include list), so every future unrelated chart would have to be added to it.
  - Listing both repositories would create two separate packages for the same chart.
- Artifact Hub needs one repository entry per OCI chart; here that is exactly one.
- The README and the website keep showing the HTTP channel for `helm repo add`; Artifact Hub's own install instructions show the OCI command. Both are correct.

## 3. `Chart.yaml` annotations **[architect]**

Added to `charts/generated-secrets-operator/Chart.yaml` (source: <https://github.com/artifacthub/hub/blob/master/docs/helm_annotations.md>):

- `artifacthub.io/category`: `security`
- `artifacthub.io/license`: `Apache-2.0`
- `artifacthub.io/operator`: `"true"`
- `artifacthub.io/operatorCapabilities`: `Seamless Upgrades`. This is the honest level: install plus upgrades that never rotate credentials (frozen defaults and fingerprints). No "full lifecycle" claims.
- `artifacthub.io/crds`: one entry — kind `GeneratedSecret`, version `v1alpha1`, name `generatedsecrets.generatedsecrets.jabbas.eu`, displayName "Generated Secret", and a one-sentence description.
- `artifacthub.io/crdsExamples`: the `examples/app-admin-password.yaml` manifest (complete `apiVersion`/`kind`/`metadata`/`spec`).
- `artifacthub.io/images`: the operator image `ghcr.io/jabbas/generated-secrets-operator:<appVersion>` with platforms `linux/amd64` and `linux/arm64`, so that Artifact Hub's Trivy scan uses the right image instead of guessing it from a dry-run.
  - **[architect]** The committed annotation carries the final version (the committed `appVersion`), and is bumped by hand together with `version` and `appVersion` in the existing manual "Bump the versions" step of `docs/development.md` (which gets one more line). Release candidates are packaged with `--app-version X.Y.Z-rc.N` but keep the annotation at `:X.Y.Z`; this is harmless because Artifact Hub never shows release candidates (§4). `release.yml`, `chart-compare.sh` and ADR 0005 are NOT changed.
  - The chart test (`cargo xtask chart-test`, next to `chart_version_matches_workspace`) MUST assert that the annotation's tag equals the committed `appVersion`, so a forgotten bump fails CI.
- `artifacthub.io/links`:
  - source — the GitHub repository;
  - documentation — the `docs` folder on GitHub; the website once project 4 is live;
  - support — GitHub issues (Artifact Hub highlights a link named "support").

Not used, with reasons:

- `changes` — would need hand-maintained per-release YAML that duplicates `CHANGELOG.md`, i.e. a second source of truth.
- `signKey` — meant for PGP keys; keyless cosign has none.
- `prerelease` — release candidates are hidden instead (§4).
- `screenshots`, `recommendations`, `alternativeName` — no requirement.

## 4. Repository metadata (`artifacthub-repo.yml`) **[architect]**

- The file is kept in the repository at `.github/artifacthub-repo.yml`, not inside the chart directory, so it is not packaged with the chart:

  ```yaml
  repositoryID: <ID from the Artifact Hub control panel>
  ignore:
    - name: generated-secrets-operator
      version: -rc\.
  ```

- The `ignore` rule keeps release candidates out of Artifact Hub. Reason: Artifact Hub treats the highest semantic version (semver) as the latest, so `0.2.x-rc.N` would otherwise be shown as the latest version until the final release. `version` is an unanchored regular expression.
- The file is published to the OCI repository under the special tag `artifacthub.io`:

  ```sh
  oras push ghcr.io/jabbas/charts/generated-secrets-operator:artifacthub.io \
    --config /dev/null:application/vnd.cncf.artifacthub.config.v1+yaml \
    .github/artifacthub-repo.yml:application/vnd.cncf.artifacthub.repository-metadata.layer.v1.yaml
  ```

  The tag is not valid semver, so Artifact Hub's version listing skips it. The release workflow's own registry lookups MUST also tolerate a non-semver tag in that repository; verify `.github/scripts/registry-lookup.sh` and its test.
- An agent pushes the file once (with a GitHub token allowed to write packages), and again only when the file changes. It is not part of `release.yml`: the file almost never changes, and a release step would add a moving part to the release pipeline without a lasting requirement. `docs/development.md` documents the one command.
- `owners` is not set; it is only needed for ownership claims.

## 5. Steps and who does them

1. **Agent:** add the `Chart.yaml` annotations (§3) and `.github/artifacthub-repo.yml` without `repositoryID`, in a pull request. **[architect]** Timing: preferably merged to `main` after the rename is merged and BEFORE `0.2.0-rc.1` is tagged, so 0.2.0 carries them (annotations only take effect on a new chart version). If `0.2.0-rc.1` is already cut, do not restart the 0.2.0 cycle for metadata: the annotations ship with the next regular release (for example 0.2.1, with its normal release-candidate and home-lab cycle). Either way, the owner registers the repository on Artifact Hub (step 2) only after the first final version WITH the annotations is published, so the first listed version already shows the CRD card and images.

   **What happened [owner, 2026-11-15]:** `0.2.0-rc.1` was tagged before the annotations were merged, but its home-lab test had not started. Restarting therefore cost only one build, not a full cycle: the annotations (pull request #24) are merged, `0.2.0-rc.2` is tagged on the new `main`, tested in the home lab and promoted to 0.2.0; `0.2.0-rc.1` stays as a superseded candidate (precedent `0.1.4-rc.1`). The rule above still applies whenever a candidate's test has already begun.
2. **Owner** (needs the owner's Artifact Hub login, via GitHub): sign in at artifacthub.io → Control Panel → Repositories → Add:
   - kind: Helm charts;
   - name: `generated-secrets-operator` (cannot be renamed later);
   - URL: `oci://ghcr.io/jabbas/charts/generated-secrets-operator`.

   Copy the repository ID shown on the card.
3. **Agent**: put the ID into `.github/artifacthub-repo.yml`, commit, and run the `oras push` (§4).
4. Wait for processing. Artifact Hub processes repositories every 30 minutes; the package is visible within about 5 minutes after a run. Verified publisher appears on the next processing run.
5. **Agent**: verify the package page — Verified publisher, Signed (cosign), README rendered, CRD card with example, license, links, security report present, no release-candidate versions listed. Add the Artifact Hub badge to the README (contributor-readiness design §3 item 1).
6. Not requested: "Official" status. It needs a manual GitHub issue and requires verified publisher plus ownership of the software. Possible later; no requirement now.

## 6. Done criteria

- The package page shows Verified publisher and Signed (cosign).
- The README and the CRD example render.
- No release candidates are listed.
- The Artifact Hub badge is in the README.
- `docs/development.md` documents the `oras push`.

## 7. Left to the builder

- The exact CRD description text.
- Link names other than "support".
- Where in the release tooling the `images` annotation is bumped, as long as the bump is automatic and tested.
