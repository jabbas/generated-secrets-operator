# Rename to Generated Secrets Operator — Design

| | |
|---|---|
| **Status** | Approved by owner (2026-11-15) |
| **Date** | 2026-11-15 |
| **Target release** | 0.2.0 (breaking) |
| **Supersedes names in** | `docs/design/2026-10-18-secretspec-operator-design.md` (behaviour unchanged) |

**Conventions.** The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are normative (RFC 2119 sense).
Every decision carries its origin tag, as in the 2026-10-18 design:

- **[owner]** — decided by the project owner.
- **[architect]** — architect's call; the owner may override it during review.
- **[architect assumption]** — a fact the architect assumed because no information was available; the owner should confirm or correct it.

Section 9 lists what is deliberately left to the builder.

---

## 1. Intent

- The project "SecretSpec Operator" is renamed to "Generated Secrets Operator", and the custom resource definition (CRD) kind `SecretSpec` becomes `GeneratedSecret`. The rename happens before any public promotion (project website, Artifact Hub listing, Show HN post). **[owner]**
- Why: the current name collides with cachix/secretspec (secretspec.dev), a Rust secrets tool with about 1.6k GitHub stars and its own ecosystem:
  - packages on crates.io, npm and PyPI, a GitHub Action and an IDE plugin;
  - "declarative secret generation" since its 0.7 (February 2026);
  - a Kubernetes provider since 0.20;
  - several Hacker News front-page posts.

  Search results and discussion would conflate the two projects.
- Why this name **[owner]**: it follows the ecosystem convention External Secrets Operator / `ExternalSecret`, Sealed Secrets / `SealedSecret`. A platform engineer who reads `ExternalSecret`, `SealedSecret` and `GeneratedSecret` side by side knows what each one is. The name also serves the original design goal "self-explaining API".
- Accepted risk: the kind `GeneratedSecret` was used before by four small projects (1–5 stars). krystal/generated-secrets (archived 2023) even used the title "Generated Secrets Operator"; containerinfra/kube-secrets-operator is active. There is no technical collision, because the API group differs. Only a cluster that runs two of these operators would need the fully qualified form `kubectl get generatedsecrets.generatedsecrets.jabbas.eu`.
- Why now: the project is still 0.x and `v1alpha1`, and there are no installations with resources. The owner's home lab runs operator 0.1.5 through Flux but has no `SecretSpec` objects. **[owner]** This is the cheapest moment the rename will ever have.

## 2. Goals, non-goals, anti-goals

### 2.1 Goals

- Every user-visible name moves to the new name according to the name map (§3).
- API shape and behaviour stay identical apart from names: fields, defaults, validation rules, Common Expression Language (CEL) rules, status conditions and reasons, Event reasons, and the reconciliation contract.
- Release 0.2.0 goes through the normal process: release candidate → home-lab test (2026-10-18 design §12) → promotion (ADR 0005).

### 2.2 Non-goals **[architect]**

- Any other API or behaviour change. One change at a time; other improvements go into later releases.
- Migration tooling, compatibility with `secretspec.jabbas.eu` labels or annotations, serving both API groups, conversion webhooks. The 0.2.0 operator does not recognise anything created by 0.1.x. Justification: there are no installations with resources.
- Deleting, deprecating or rewriting already published artefacts (see §5).
- Promotion work: rewriting the README as a landing page, the website, the Artifact Hub listing, re-recording the demo. These are separate follow-up projects. This project changes the README only as far as names go.
- Extracting `.github/scripts/helm-index.sh` into a reusable workflow. This is known debt; its trigger is a third chart in jabbas/helm-charts, which has not been reached.

### 2.3 Anti-goals (a failure even if it "works")

- Any leftover "secretspec" on a user-visible surface of 0.2.0 outside the exceptions in §5.
- Compatibility shims "just in case".
- Any change to the frozen defaults (length 32, alphanumeric charset, RSA 4096, ECDSA P-256, bcrypt cost 12) or to the canonical fingerprint text other than its header line (§4).

## 3. Name map (normative)

All rows are **[architect]** unless tagged otherwise.

| Thing | Old | New |
|---|---|---|
| Project display name **[owner]** | SecretSpec Operator | Generated Secrets Operator |
| GitHub repository | `jabbas/secretspec-operator` | `jabbas/generated-secrets-operator` |
| API group | `secretspec.jabbas.eu` | `generatedsecrets.jabbas.eu` |
| API version | `v1alpha1` | `v1alpha1` (unchanged) |
| Kind **[owner]** | `SecretSpec` | `GeneratedSecret` |
| Plural / singular | `secretspecs` / `secretspec` | `generatedsecrets` / `generatedsecret` |
| Short names | `ss` | `gensecret` |
| CRD name | `secretspecs.secretspec.jabbas.eu` | `generatedsecrets.generatedsecrets.jabbas.eu` |
| CRD file in chart | `files/crd/secretspecs.secretspec.jabbas.eu.yaml` | `files/crd/generatedsecrets.generatedsecrets.jabbas.eu.yaml` |
| Rust API types | `SecretSpec`, `SecretSpecSpec`, `SecretSpecStatus` | `GeneratedSecret`, `GeneratedSecretSpec`, `GeneratedSecretStatus` |
| Crate directories / package names | `crates/secretspec-api`, `secretspec-core`, `secretspec-operator` | `crates/generated-secrets-api`, `generated-secrets-core`, `generated-secrets-operator` (Rust paths `generated_secrets_api`, …) |
| Binary | `secretspec-operator` | `generated-secrets-operator` |
| Container image | `ghcr.io/jabbas/secretspec-operator` | `ghcr.io/jabbas/generated-secrets-operator` |
| OCI chart | `oci://ghcr.io/jabbas/charts/secretspec-operator` | `oci://ghcr.io/jabbas/charts/generated-secrets-operator` |
| Chart directory and name | `charts/secretspec-operator` | `charts/generated-secrets-operator` |
| HTTP repository chart (index `https://jabbas.github.io/helm-charts` unchanged) | `jabbas/secretspec-operator` | `jabbas/generated-secrets-operator` |
| Helm template helpers | `secretspec-operator.*` | `generated-secrets-operator.*` |
| Label `app.kubernetes.io/managed-by` value | `secretspec-operator` | `generated-secrets-operator` |
| Label naming the managing resource | `secretspec.jabbas.eu/spec-name` | `generatedsecrets.jabbas.eu/owner-name` |
| Annotation with the managing resource UID | `secretspec.jabbas.eu/spec-uid` | `generatedsecrets.jabbas.eu/owner-uid` |
| Annotations `fingerprints`, `generated-at`, `keys` | `secretspec.jabbas.eu/…` | `generatedsecrets.jabbas.eu/…` |
| Reserved label / annotation prefix | `secretspec.jabbas.eu/` | `generatedsecrets.jabbas.eu/` |
| Server-side apply field manager | `secretspec-operator` | `generated-secrets-operator` |
| Install namespace in docs, examples, chart README, `xtask` (`NAMESPACE` in `xtask/src/chart.rs`) and CI diagnostics | `secretspec-system` | `generated-secrets-system` |
| Lease, ServiceAccount, RBAC (role-based access control) object names | derived from the chart fullname | follow the new chart name |
| Metrics prefix | `secretspec_` | `generated_secrets_` |
| Gauge of resources by readiness | `secretspec_secretspecs{ready}` | `generated_secrets_resources{ready}` |
| Fingerprint header line | `secretspec-fingerprint-v1` | `generated-secrets-fingerprint-v1` (see §4) |
| Test environment variable | `SECRETSPEC_TEST_KUBECONFIG` | `GENERATED_SECRETS_TEST_KUBECONFIG` |
| SBOM (software bill of materials) and release asset file names | `secretspec-operator-*` | `generated-secrets-operator-*` |
| cosign keyless certificate identity | `https://github.com/jabbas/secretspec-operator/.github/workflows/release.yml@refs/tags/<tag>` | `https://github.com/jabbas/generated-secrets-operator/.github/workflows/release.yml@refs/tags/<tag>` |
| OCI image source annotation | `https://github.com/jabbas/secretspec-operator` | `https://github.com/jabbas/generated-secrets-operator` |
| Validation messages that embed names (`MSG_RESERVED_METADATA`, `MSG_NAME_LENGTH` in `crates/secretspec-api/src/constants.rs`) | old prefix | new prefix; every other message string unchanged |

**Rule for anything not listed:**

- `secretspec-operator` → `generated-secrets-operator`;
- `secretspec-system` → `generated-secrets-system`;
- `secretspec.jabbas.eu` → `generatedsecrets.jabbas.eu`;
- the kind / type `SecretSpec` → `GeneratedSecret`, also in prose ("a SecretSpec" → "a GeneratedSecret");
- snake_case `secretspec_` → `generated_secrets_`;
- `SECRETSPEC_` → `GENERATED_SECRETS_`.

**Why `owner-name` / `owner-uid` instead of keeping `spec-name` / `spec-uid`.** Under the new prefix, "spec" would read as a leftover of the old kind name. "Owner" means "the GeneratedSecret that manages this Secret", whether or not an ownerReference exists (with `deletionPolicy: Retain` there is none). `docs/api-reference.md` and `docs/operations.md` MUST say this in one sentence.

## 4. Fingerprints **[architect]**

- The header line of every canonical text changes from `secretspec-fingerprint-v1` to `generated-secrets-fingerprint-v1`. Every golden vector in `docs/fingerprints.md` and in the golden tests (`fingerprint_golden.rs`, `resolve_golden.rs`) is recomputed. The canonical text body (slot, mode, parameters) does not change.
- Why this is safe even though `docs/fingerprints.md` says the format is frozen forever:
  - the 0.2.0 operator reads fingerprints only from `generatedsecrets.jabbas.eu/fingerprints`, an annotation no 0.1.x Secret carries, so it can never compare against an old-header fingerprint;
  - no installation has managed Secrets.
- `docs/fingerprints.md` states in one sentence that the format is frozen from 0.2.0; the 0.1.x format belonged to the project's former name and is never read. The "frozen forever" promise applies from 0.2.0 on.

## 5. What keeps the old name (explicit exceptions) **[architect]**

- Historical records are not rewritten:
  - `docs/adr/0001`–`0005`;
  - `docs/plans/*`;
  - `docs/releases/*` (0.1.x home-lab reports);
  - `docs/design/2026-10-18-secretspec-operator-design.md`;
  - CHANGELOG entries for 0.1.x.
- A one-line note goes at the top of `docs/design/2026-10-18-secretspec-operator-design.md` and into `docs/adr/README.md`: the project was renamed in 0.2.0 (ADR 0006); names in older records are historical; the 2026-10-18 design remains the reference for behaviour, read through the name map of this document.
- Published artefacts stay as they are, so old versions stay installable:
  - GitHub releases and git tags 0.1.0–0.1.5 with their assets and signatures;
  - GitHub Container Registry (GHCR) packages `secretspec-operator` and `charts/secretspec-operator`;
  - the `secretspec-operator` entries in the jabbas/helm-charts `index.yaml`.
- The demo recording at the repository root (`secretspec-demo.cast`, `secretspec-demo.gif`) is untracked and stays untracked; it is not touched here and is re-recorded in the promotion project. **[architect]**

## 6. Repository and release infrastructure

- **[owner action — DONE 2026-11-15]** The repository was renamed in GitHub Settings to `jabbas/generated-secrets-operator`. GitHub keeps stars, issues, releases and tags, and redirects old web and git URLs.
- **Release freeze [architect]:** from the repository rename until the rename branch is merged, no release tag (release candidate or final) is pushed. The cosign identity and the OCI source annotation now derive from the new repository URL, while `release.yml` still compares them against the hard-coded old URL, so any release in between fails. Continuous integration on pull requests and `main` is not affected.
- After the rename, every local clone (the owner's and every agent session's) runs `git remote set-url origin` with the new URL. Renaming the local directory `~/Projects/secretspec-operator` is optional and not part of this project.
- GHCR: the new packages `generated-secrets-operator` and `charts/generated-secrets-operator` are created by the first push. After the first release candidate, verify anonymous pull of the image and of the OCI chart (for example `crane manifest` and `helm pull` without login). If a package is private, the owner sets it public and links it to the repository in the package settings. **[architect]** This is an owner checkpoint in the plan: if the `helm-index` job of `0.2.0-rc.1` fails only because its anonymous `crane manifest` read hit a private package, the owner switches visibility and re-runs that job; a green re-run counts as "rc.1 published", no `rc.2` is needed for that alone.
- The fine-grained personal access token in the `helm-charts` environment: its repository access is bound to the repository, which keeps its identity across a rename, so it is expected to keep working. The helm-index job of the first release candidate verifies this; if it fails, the owner regenerates the token including the renamed repository.
- `.github/scripts/helm-index.sh` has `CHART_NAME` hard-coded; change it to the new chart name. The second copy of this script in the homepage-dziegielew.ski repository is unaffected (it takes `CHART_NAME` as a parameter). Old index entries stay.
- `release.yml` and the scripts in `.github/scripts` follow the name map (`IMAGE`, chart paths, SBOM names, identities, source annotation). The registry lookup MUST treat "package does not exist yet" as "no published versions", because the first 0.2.0 release candidate is the first push to the new packages. Verify this against `registry-lookup-test.sh` and extend the test if that case is not covered.
- Release notes keep using the previous final tag (0.1.5) as their start; repository history is continuous.
- `renovate.json`: package names and paths follow the name map.
- Coordination **[owner]**: another agent session is currently developing a feature and changing the Renovate configuration in this repository. The rename starts only after that work is merged to `main`. It runs on its own branch, and no other large change runs in parallel, because the rename touches nearly every file.
- Renovate during the rename **[architect]**: before the rename branch is cut, merge or close every open Renovate pull request. Renovate is not paused: pull requests it opens while the rename branch is open are left alone until the rename is merged, after which Renovate rebases them by itself.
- Surfaces outside this repository **[architect]**, owner actions in the plan: update the GitHub repository description and topics, and, if `jabbas/helm-charts` has a README or chart list naming `secretspec-operator`, add the new chart there (the old entry may stay, marked as renamed).
- `CHANGELOG.md` gets a 0.2.0 entry marked BREAKING, with a short summary of the name map and a section "Moving from 0.1.x":
  1. There is no automatic migration.
  2. Uninstall the old chart.
  3. Delete the old CRD if it was kept: `kubectl delete crd secretspecs.secretspec.jabbas.eu` (the chart sets `helm.sh/resource-policy: keep` when `crds.keep` is true).
  4. Install the new chart (the documented namespace is now `generated-secrets-system`).
  5. Rewrite manifests (`apiVersion` and `kind`).

  Secrets created by 0.1.x are not adopted, because they carry the old labels. With `deletionPolicy: Retain` they remain as plain Secrets, and a GeneratedSecret with the same name reports `Conflict` until the old Secret is deleted or renamed.
- New ADR (architecture decision record) `docs/adr/0006-rename-to-generated-secrets-operator.md`, status Accepted, with the sections Title, Status, Date, Context, Decision, Alternatives considered and Consequences per `docs/adr/README.md`. It summarises §1, §4 and §5. Alternatives considered:
  - keep the name with a "not affiliated" note;
  - a distinctive brand name (for example Keysmith) with kind `GeneratedSecret`;
  - compatibility code that reads old labels.

  Add it to the ADR index.
- `NOTICE`: update the project name.

## 7. Home-lab cut-over **[owner]**

Done in the flux-home-system repository, not in this repository.

Precondition: `kubectl get secretspecs --all-namespaces` returns nothing.

1. Remove the `secretspec-operator` HelmRelease; Flux uninstalls it (Deployment, RBAC and Lease go with it).
2. Delete the CRD `secretspecs.secretspec.jabbas.eu` if it was kept.
3. Add a HelmRelease for `generated-secrets-operator` pinned to `0.2.0-rc.1` for the home-lab test, in the namespace `generated-secrets-system` (with the Pod Security `restricted` label, as documented in `docs/installation.md`); remove the old `secretspec-system` namespace if Flux created it.
4. After promotion, pin `0.2.0`.

## 8. Done criteria

- A case-insensitive search for "secretspec" across the tracked files finds only the exceptions in §5 and the history sentence in `docs/fingerprints.md`. This is a one-off check by the builder; no permanent CI guard is added (it would be a component without a lasting requirement).
- The unit, golden and end-to-end (chart test) suites are green in CI.
- The CRD is regenerated with `cargo xtask crdgen` and matches the chart file (existing assertion).
- `0.2.0-rc.1` is published: a multi-arch image signed with the new identity, the OCI chart, and the HTTP index entry. Anonymous pull works. The verification commands in `docs/installation.md` and `docs/security.md` work as written.
- The home-lab test procedure passes on `0.2.0-rc.1` with the new names; then 0.2.0 is promoted.
- `README.md`, `docs/installation.md`, `docs/api-reference.md`, `docs/operations.md`, `docs/security.md`, `docs/fingerprints.md`, `docs/development.md`, `examples/*`, the chart README and `NOTES.txt` use the new names.

## 9. Left to the builder

- The order of the mechanical edits and the commit granularity.
- Using `git mv` for renamed directories and files (to keep history).
- Exact documentation wording.
- Rust identifiers not listed in §3, following the rule under the name map.
