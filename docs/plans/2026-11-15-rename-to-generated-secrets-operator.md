# Rename to Generated Secrets Operator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename SecretSpec Operator to Generated Secrets Operator (kind `GeneratedSecret`, short name `gensecret`, API group `generatedsecrets.jabbas.eu`) and release it as 0.2.0, with no other API or behaviour change.

**Architecture:** One branch, `rename/generated-secrets-operator`, cut from `main` after every open Renovate pull request is merged or closed. Tasks 1–5 are code and documentation changes reviewed one by one. Task 6 is the one-off search and the whole-branch review. The release phase (R1–R6) runs from `main` after the merge, with STOP points for owner actions.

**Tech Stack:** Rust workspace (kube 5, schemars, cargo xtask), Helm 4 chart, GitHub Actions (`ci.yml`, `release.yml`, `.github/scripts/*.sh`), cosign keyless signing, GHCR, the jabbas/helm-charts HTTP index, Flux in the home lab.

**Spec:** `docs/design/2026-11-15-rename-to-generated-secrets-operator-design.md` (approved by the owner). §3 (name map) and §4 (fingerprints) are normative. The behaviour reference stays `docs/design/2026-10-18-secretspec-operator-design.md`, read through the name map.

## Global Constraints

- Every rename follows the design §3 name map. For anything the map does not list, apply its rules: `secretspec-operator` → `generated-secrets-operator`, `secretspec-system` → `generated-secrets-system`, `secretspec.jabbas.eu` → `generatedsecrets.jabbas.eu`, kind or type `SecretSpec` → `GeneratedSecret` (in prose as well), `secretspec_` → `generated_secrets_`, `SECRETSPEC_` → `GENERATED_SECRETS_`.
- Label `secretspec.jabbas.eu/spec-name` becomes `generatedsecrets.jabbas.eu/owner-name`. Annotation `secretspec.jabbas.eu/spec-uid` becomes `generatedsecrets.jabbas.eu/owner-uid`.
- The readiness gauge becomes `generated_secrets_resources{ready}`. It is NOT `generated_secrets_generatedsecrets`.
- The short name becomes `gensecret`. The old short name `ss` is removed.
- The fingerprint header becomes `generated-secrets-fingerprint-v1`. The canonical text body (slot, mode, parameters) MUST NOT change.
- Frozen defaults MUST NOT change: length 32, alphanumeric charset, RSA 4096, ECDSA P-256, bcrypt cost 12. Fields, validation rules, CEL rules, conditions and reasons, Event reasons and the reconciliation contract also MUST NOT change. Validation message strings change only where they embed a name (`MSG_RESERVED_METADATA`, `MSG_NAME_LENGTH`).
- No compatibility shims. Do not read old labels or annotations, do not serve the old API group, do not add a migration.
- Nothing outside the design scope (§2.2). No other improvements, no promotion work, no helm-index extraction.
- Exceptions that keep the old name (§5) are NOT edited, apart from the one-line notes in Task 4:
  - `docs/adr/0001`–`0005`;
  - `docs/plans/*` other than this file;
  - `docs/releases/*`;
  - `docs/design/2026-10-18-secretspec-operator-design.md`;
  - CHANGELOG 0.1.x entries;
  - the untracked `secretspec-demo.cast` and `secretspec-demo.gif`.
- Use `git mv` for every renamed directory or file.
- **Release freeze:** no tag (rc or final) is pushed until this branch is merged to `main` and `main` CI is green.
- Renovate pull requests opened while this branch is open are left alone (not merged, not edited) until the rename is merged; Renovate rebases them itself afterwards (design §6).
- Never run `gh auth token`. Never print Secret values. No force pushes.
- Run `Cargo.lock` changes only through building or `cargo metadata`, never `cargo update`. Workspace package names change; dependency versions MUST NOT change.

## Review Focus

1. **The operator stops seeing its own Secrets.** This happens if the Secret watch selector, `owner-name` mapping or the managed-by value still uses an old string anywhere. The operator must find, adopt and reconcile only Secrets that carry the new labels. Pinned by `api_identity_constants` (Task 2), plus the e2e and operator-integration suites on kind.
2. **Reserved-key CEL rule.** It must reject `generatedsecrets.jabbas.eu/…` keys in `secret.labels`/`secret.annotations`. Keys under the old prefix become ordinary labels and must be accepted. Pinned by fixtures `i24`/`i25` (new prefix, rejected) and a new valid fixture `v20-old-prefix-label.yaml` (Task 2).
3. **First push to brand-new GHCR packages.** The release must treat "package does not exist" (NAME_UNKNOWN, or denied/403 for a missing package) as "no published versions" in lenient lookups, and must never treat it as absent in the strict pre-write check. Pinned by new cases in `registry-lookup-test.sh` (Task 3).
4. **Chart fullname rule and the default image tag.** With release name `generated-secrets-operator` the fullname is the release name. With release name `x` it is `x-generated-secrets-operator`. The default image renders as `ghcr.io/jabbas/generated-secrets-operator:<appVersion>`. Pinned by the existing chart-test assertions with the new names (Task 2).
5. **Golden fingerprints silently unchanged.** If the header constant or a vector is missed, the golden tests could still pass against stale data. Pinned by `fingerprint_header_is_generated_secrets` (Task 1) and recomputed vectors in `fingerprint_golden.rs`, `resolve_golden.rs` and `docs/fingerprints.md`.

---

### Task 1: Fingerprint header and golden vectors

**Files:**
- Modify: `crates/secretspec-core/src/fingerprint.rs` (`HEADER`, line ~30)
- Modify: `crates/secretspec-core/tests/fingerprint_golden.rs`, `crates/secretspec-core/tests/resolve_golden.rs`
- Modify: `docs/fingerprints.md` (all `secretspec-fingerprint-v1` occurrences, the vector table, the `printf … | shasum` example, plus one sentence on the freeze)

**Interfaces:**
- Produces: `HEADER = "generated-secrets-fingerprint-v1"`. Every canonical text is `HEADER` + an unchanged body.

- [ ] **Step 1:** In `fingerprint_golden.rs`, add test `fingerprint_header_is_generated_secrets`. It builds the canonical text of vector 6 (`ssh/keypair`, ed25519) and asserts that the text equals `"generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=ed25519\n"` and that it does not contain `"secretspec"`.
- [ ] **Step 2:** Run `cargo test -p secretspec-core --test fingerprint_golden fingerprint_header_is_generated_secrets`. Expected: FAIL.
- [ ] **Step 3:** Change `HEADER` to `"generated-secrets-fingerprint-v1"`.
- [ ] **Step 4:** Recompute every golden SHA-256 with `printf '<canonical text>' | shasum -a 256`, using the exact canonical texts from `docs/fingerprints.md`'s vector table with only the header line replaced. Update the expected strings and digests in `fingerprint_golden.rs`, `resolve_golden.rs` and the `docs/fingerprints.md` table and examples. For each vector, check that the body after the first line is byte-identical to the old one.
- [ ] **Step 5:** In `docs/fingerprints.md`, add this sentence: the format is frozen from 0.2.0; the 0.1.x format (header `secretspec-fingerprint-v1`) belonged to the project's former name and is never read. This is the only allowed `secretspec` mention outside §5.
- [ ] **Step 6:** Run `cargo test -p secretspec-core` and `cargo test -p secretspec-core --release -- --ignored`. Expected: PASS.
- [ ] **Step 7:** Commit `feat(core)!: fingerprint header generated-secrets-fingerprint-v1`.

### Task 2: Rust workspace, API identity, chart and xtask

**Files:**
- `git mv` the crates: `crates/secretspec-api` → `crates/generated-secrets-api`, `crates/secretspec-core` → `crates/generated-secrets-core`, `crates/secretspec-operator` → `crates/generated-secrets-operator`.
- `git mv` the chart: `charts/secretspec-operator` → `charts/generated-secrets-operator`, and its CRD file → `files/crd/generatedsecrets.generatedsecrets.jabbas.eu.yaml` (regenerated, not hand-edited).
- Modify:
  - root `Cargo.toml` (members, `repository` = `https://github.com/jabbas/generated-secrets-operator`, the comment), every crate `Cargo.toml`, `xtask/Cargo.toml`, `Cargo.lock` (via build only);
  - all Rust sources and tests;
  - `crates/*/tests/fixtures/**` (`apiVersion`, `kind`, reserved-prefix keys, expected messages);
  - `examples/*.yaml`, `examples/README.md`;
  - `xtask/src/{main.rs,chart.rs,licenses.rs}` (`CRD_PATH`, `OPERATOR_BIN`, `CHART_DIR`, `RELEASE`, `NAMESPACE` = `generated-secrets-system`, repo `jabbas/generated-secrets-operator`, lease rules, CRD name);
  - every chart file: `Chart.yaml` (name, description, home, sources), `values.yaml`, `values.schema.json` title, all templates and helpers `generated-secrets-operator.*`, `templates/crd.yaml` path, `NOTES.txt`, chart `README.md`;
  - `build/Containerfile` (title, description, source, binary path, ENTRYPOINT).
- Create: `crates/generated-secrets-api/tests/fixtures/valid/v20-old-prefix-label.yaml`.

**Interfaces:**
- Produces:
  - Rust crates `generated_secrets_api`, `generated_secrets_core`, `generated_secrets_operator`;
  - types `GeneratedSecret`, `GeneratedSecretSpec`, `GeneratedSecretStatus`;
  - constants in `generated_secrets_api::constants`:
    - `GROUP = "generatedsecrets.jabbas.eu"`, `KIND = "GeneratedSecret"`;
    - `MANAGED_BY_VALUE = "generated-secrets-operator"`, `FIELD_MANAGER = "generated-secrets-operator"`;
    - `LABEL_OWNER_NAME = "generatedsecrets.jabbas.eu/owner-name"` (renamed from `LABEL_SPEC_NAME`);
    - `ANNOTATION_OWNER_UID = "generatedsecrets.jabbas.eu/owner-uid"` (renamed from `ANNOTATION_SPEC_UID`);
    - `ANNOTATION_FINGERPRINTS`/`ANNOTATION_GENERATED_AT`/`ANNOTATION_KEYS` under `generatedsecrets.jabbas.eu/`;
    - `RESERVED_PREFIX = "generatedsecrets.jabbas.eu/"`;
  - CRD plural `generatedsecrets`, singular `generatedsecret`, short names `["gensecret"]`;
  - binary `generated-secrets-operator`;
  - metrics `generated_secrets_reconcile_total`, `generated_secrets_reconcile_duration_seconds`, `generated_secrets_generations_total`, `generated_secrets_resources{ready}`, `generated_secrets_next_rotation_timestamp_seconds`;
  - test environment variables `GENERATED_SECRETS_TEST_KUBECONFIG`, `GENERATED_SECRETS_TEST_LOG`, `GENERATED_SECRETS_E2E_KUBECONFIG`, `GENERATED_SECRETS_E2E_IMAGE`;
  - test namespaces with prefix `generated-secrets-it-`, and the e2e release namespace `generated-secrets-system`. Keep every namespace name at or below 63 characters.

- [ ] **Step 1:** Write the failing test `api_identity_constants` in `crates/secretspec-api/tests/schema.rs` (it moves with the crate in Step 3). It asserts:
  - every constant value listed under Produces;
  - from `GeneratedSecret::crd()`: group, kind, plural, singular, `short_names == ["gensecret"]`, CRD name `generatedsecrets.generatedsecrets.jabbas.eu`;
  - `MSG_RESERVED_METADATA` contains `generatedsecrets.jabbas.eu/` and does not contain `secretspec`.
- [ ] **Step 2:** Run it. Expected: FAIL (compile error or wrong values).
- [ ] **Step 3:** Run the `git mv` renames, then apply the name map to every file listed above.
- [ ] **Step 4:** Create the valid fixture `v20-old-prefix-label.yaml`: a minimal `GeneratedSecret` with `spec.secret.labels: {"secretspec.jabbas.eu/legacy": "x"}`. It must pass CEL, `resolve()` and server validation. It is the second allowed `secretspec` occurrence; add a comment explaining that the old prefix is no longer reserved.
- [ ] **Step 5:** Regenerate the CRD with `cargo xtask crdgen`. Then:
  - `git diff --stat` must show the old CRD file removed and the new one added;
  - in the new file, only names, descriptions mentioning names, the short name and the reserved-prefix CEL rule/message may differ from the old one (`diff` the old file from `HEAD` against the new one).
- [ ] **Step 6:** Run the full gate:
  ```
  cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check && cargo audit && cargo xtask crdgen --check && cargo xtask chart-test && cargo xtask licenses
  ```
  Expected: PASS, chart-test 15/15. Check `git diff Cargo.lock`: only the workspace package names changed.
- [ ] **Step 7:** Run server validation, operator integration and e2e on a fresh kind 1.37.0 cluster (Podman provider, `--kubeconfig $TMPDIR/opencode/rename.kubeconfig`, never the default context), following `docs/development.md` with the new variable names. Build the image with `cargo xtask dist` + `podman build` tagged `localhost/generated-secrets-operator:e2e`. Expected: every test in the three suites passes (`v20` accepted by the server). Delete the cluster.
- [ ] **Step 8:** Commit in logical commits, for example:
  - `refactor!: rename crates and types to generated-secrets`
  - `feat(api)!: API group generatedsecrets.jabbas.eu, kind GeneratedSecret`
  - `feat(chart)!: chart generated-secrets-operator`

### Task 3: CI, release workflow, scripts and Renovate

**Files:**
- Modify:
  - `.github/workflows/ci.yml`: package names in `-p`, kind cluster names, test env vars, e2e image tag, diagnostics namespace/selector, `kubectl get generatedsecrets`;
  - `.github/workflows/release.yml`: `IMAGE`, `cargo metadata` package name, chart paths, binary paths, SBOM file names, every identity, the source annotation and its check, chart names, release notes text, helm-index arguments;
  - `.github/scripts/helm-index.sh` (`CHART_NAME=generated-secrets-operator`);
  - `.github/scripts/helm-index-test.sh` and `.github/scripts/chart-compare-test.sh` (fixture names);
  - `.github/scripts/chart-compare.sh` (comment);
  - `renovate.json`: names and paths; verify, change only if a path or package name matches the map;
  - `.containerignore` / `.dockerignore`, only if they name the binary.
- Test: `.github/scripts/registry-lookup-test.sh`.

**Interfaces:**
- Consumes: Task 2 names (chart dir `charts/generated-secrets-operator`, binary `generated-secrets-operator`, package `generated-secrets-operator`).
- Produces:
  - release identity `https://github.com/jabbas/generated-secrets-operator/.github/workflows/release.yml@refs/tags/<tag>`;
  - `IMAGE=ghcr.io/jabbas/generated-secrets-operator`;
  - chart `oci://ghcr.io/jabbas/charts/generated-secrets-operator`;
  - SBOMs `generated-secrets-operator-<tag>-linux-{amd64,arm64}.spdx.json`.

- [ ] **Step 1:** In `registry-lookup-test.sh`, add failing cases if they are not already covered:
  - (a) `tag_digest <ref> lenient` with registry stderr `NAME_UNKNOWN: repository name not known to registry` → empty output (absent);
  - (b) the same with `denied: requested access to the resource is denied` → absent;
  - (c) `tag_digest <ref> strict` with `NAME_UNKNOWN` → fails, so it is never treated as absent;
  - (d) `chart_state` with `NAME_UNKNOWN` → `CHART_STATE=absent`.
- [ ] **Step 2:** Run `bash .github/scripts/registry-lookup-test.sh`. Expected: the new cases fail, or all pass if they are already covered. If they all pass, record that in the report; no script change is needed.
- [ ] **Step 3:** Apply the name map to the files listed. Release notes keep using the previous final tag (0.1.5) as the start; check that the notes logic still selects `0.1.5`.
- [ ] **Step 4:** Run `bash` on every `.github/scripts/*-test.sh`, then actionlint, shellcheck, and `npx --yes --package renovate@latest -- renovate-config-validator --strict`. Expected: all pass.
- [ ] **Step 5:** Run `bash .github/scripts/release-graph-check.sh` against `release.yml`. Expected: pass.
- [ ] **Step 6:** Commit `ci!: release and CI pipelines for generated-secrets-operator`.

### Task 4: User documentation, ADR 0006, NOTICE, CHANGELOG

**Files:**
- Modify: `README.md`, `docs/installation.md`, `docs/api-reference.md`, `docs/operations.md`, `docs/security.md`, `docs/development.md`, `NOTICE`, `CHANGELOG.md`, `docs/adr/README.md`, `docs/design/2026-10-18-secretspec-operator-design.md` (one-line note at the top only).
- Create: `docs/adr/0006-rename-to-generated-secrets-operator.md`.

**Interfaces:**
- Consumes: every name from Tasks 1–3.

- [ ] **Step 1:** Apply the name map to every listed user document. The install namespace becomes `generated-secrets-system`, the short name `gensecret`. Update the metric names, environment variables, `kubectl` examples (`kubectl get generatedsecrets` / `gensecret`), Flux examples (HelmRepository chart name, OCIRepository URL and identity regex), cosign verify blocks with the new identity, the HTTP repository chart `jabbas/generated-secrets-operator`, and the security advisory URL. Leave version pins on 0.1.5 for now; Task 5 sets 0.2.0.
- [ ] **Step 2:** In `docs/api-reference.md` and `docs/operations.md`, add one sentence each: "owner" in `owner-name`/`owner-uid` means the GeneratedSecret that manages the Secret, whether or not an ownerReference exists (with `deletionPolicy: Retain` there is none).
- [ ] **Step 3:** Add one-line notes at the top of `docs/design/2026-10-18-secretspec-operator-design.md` and in `docs/adr/README.md`: the project was renamed in 0.2.0 (ADR 0006); names in older records are historical; the 2026-10-18 design remains the behaviour reference, read through the name map of the 2026-11-15 design. Change nothing else in those files.
- [ ] **Step 4:** Write ADR 0006 in the format of ADRs 0001–0005, and add it to the ADR index. Use: Status Accepted, Date 2026-11-15, decided by the project owner; Context, Decision, Alternatives considered and Consequences from design §1, §4 and §5. The alternatives are:
  - keep the name with a "not affiliated" note;
  - a distinctive brand name (for example Keysmith) with kind `GeneratedSecret`;
  - compatibility code that reads old labels.
- [ ] **Step 5:** Change `NOTICE` to "Generated Secrets Operator" / "Copyright 2026 The Generated Secrets Operator Authors".
- [ ] **Step 6:** Add an `## Unreleased` entry to `CHANGELOG.md`, marked **BREAKING**, with a short name-map summary and the section "Moving from 0.1.x" with the five steps from design §6, verbatim in substance. Also explain why 0.1.x Secrets are not adopted. Leave the 0.1.x entries untouched.
- [ ] **Step 7:** Check that every relative link in the changed docs resolves, including those into `charts/generated-secrets-operator/…`. Expected: none broken.
- [ ] **Step 8:** Commit `docs!: Generated Secrets Operator (ADR 0006)`.

### Task 5: Version 0.2.0

**Files:**
- Modify: root `Cargo.toml` (`version = "0.2.0"`), `Cargo.lock` (workspace crates only), `charts/generated-secrets-operator/Chart.yaml` (`version: 0.2.0`, `appVersion: "0.2.0"`), `CHANGELOG.md` (`## Unreleased` → `## 0.2.0 - <date of the merge>`), and the current-release pins in `README.md`, `docs/installation.md`, `docs/security.md` and the chart README.

- [ ] **Step 1:** Bump the versions. Set the docs pins (verify blocks `TAG=0.2.0` / `CHART_VERSION=0.2.0`, `--version 0.2.0`, Flux examples, tgz names, the rc example `0.2.0-rc.1`) per the "Bump the versions" procedure in `docs/development.md`.
- [ ] **Step 2:** Run the full gate from Task 2 Step 6, then `helm package charts/generated-secrets-operator --version 0.2.0-rc.1 --app-version 0.2.0-rc.1` and `helm template` on the result. Expected: it renders `image: "ghcr.io/jabbas/generated-secrets-operator:0.2.0-rc.1"`.
- [ ] **Step 3:** Commit `chore(release): 0.2.0`.

### Task 6: One-off search and whole-branch review

- [ ] **Step 1:** Run `git grep -n -i secretspec` over the tracked files. Every hit must be one of:
  - a §5 exception file;
  - the history sentence in `docs/fingerprints.md`;
  - the old-prefix comment and value in `v20-old-prefix-label.yaml`;
  - the CHANGELOG "Moving from 0.1.x" section and the 0.1.x entries;
  - the notes added in Task 4 Step 3.

  Write the complete list to the ledger. Every other hit is a defect to fix before the review.
- [ ] **Step 2:** Run the final whole-branch review (`@reviewer`) over `merge-base..HEAD`. Then do one fix wave and one scoped re-review.
- [ ] **Step 3:** Push the branch and open a pull request so CI runs, then merge it with `--no-ff` into `main` after CI is green. Watch `main` CI to green.

---

## Release phase (from `main`, after the merge)

- **R1. Owner STOP A: repository surfaces.** Ask the owner to:
  - update the GitHub repository description and topics;
  - for the jabbas/helm-charts README or chart list, either add the new chart (the old entry may stay, marked as renamed) or let the agent prepare that commit.

  Continue once the owner confirms. This STOP does not block R2.
- **R2. Tag `0.2.0-rc.1`** (annotated) on the `main` merge commit, only after `main` CI is green. Watch `release.yml`: build mode, every build-mode job green, `promote` the only skipped job.
- **R3. Owner STOP B (conditional): GHCR visibility.** If `helm-index` fails only because its anonymous `crane manifest` read hit a private package:
  - STOP;
  - the owner sets `generated-secrets-operator` and `charts/generated-secrets-operator` public and links them to the repository;
  - re-run only `helm-index`.

  A green re-run counts as "rc.1 published"; no rc.2 is needed for that alone. If it fails for the personal access token, STOP: the owner regenerates the token to include the renamed repository.
- **R4. Anonymous verification.** Run every verification command in `docs/installation.md` and `docs/security.md` exactly as written (new identity, image, chart, attestations, HTTP index digest, `helm template`). Check that the image and the OCI chart pull anonymously.
- **R5. Owner STOP C: home-lab cut-over (design §7, flux-home-system).** Precondition: `kubectl get secretspecs --all-namespaces` is empty. The owner then:
  1. removes the `secretspec-operator` HelmRelease;
  2. deletes CRD `secretspecs.secretspec.jabbas.eu` if it was kept;
  3. adds a HelmRelease for `generated-secrets-operator` pinned to `0.2.0-rc.1` in `generated-secrets-system` (Pod Security `restricted` label);
  4. removes `secretspec-system`.

  Continue once the owner confirms.
- **R6. Home-lab test** of `0.2.0-rc.1`. Use the 2026-10-18 design §12 procedure and the scenario list a–k from `docs/releases/0.1.5-home-lab-test.md`, with the new names: test namespace `test-generated-secrets`, `kubectl get generatedsecrets`, new labels and annotations, new metrics. Write the report to `docs/releases/0.2.0-home-lab-test.md` and merge it to `main`.
- **R7. Promote `0.2.0`** (annotated tag on the rc.1 commit, no `rebuild:`). Verify:
  - the digest equals rc.1;
  - both identities verify;
  - the release notes start at 0.1.5;
  - Latest is set;
  - the HTTP index has the entry.
- **R8. Owner STOP D: pin `0.2.0`** in flux-home-system (design §7 step 4). Then verify the HelmRelease and the pod read-only.
- **R9. Done report** against design §8: the one-off search list (Task 6), CI results, the rc.1 publication evidence, the home-lab result, the promotion, and any leftover outside §5.
