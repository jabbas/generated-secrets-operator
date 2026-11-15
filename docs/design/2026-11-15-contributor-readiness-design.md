# Contributor Readiness — Design

| | |
|---|---|
| **Status** | Approved by owner (2026-11-15) |
| **Date** | 2026-11-15 |
| **Depends on** | `docs/design/2026-11-15-rename-to-generated-secrets-operator-design.md` (MUST be merged first) |

Everything in this document uses the new names from the rename design: project Generated Secrets Operator, kind `GeneratedSecret`, short name `gensecret`, API group `generatedsecrets.jabbas.eu`, chart `generated-secrets-operator`, install namespace `generated-secrets-system`.

**Conventions.** The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are normative (RFC 2119 sense).
Every decision carries its origin tag, as in the 2026-10-18 design:

- **[owner]** — decided by the project owner.
- **[architect]** — architect's call; the owner may override it during review.

Section 10 lists what is deliberately left to the builder.

---

## 1. Intent

- This is part of a promotion campaign **[owner]**:
  1. rename;
  2. this project — a repository ready for strangers;
  3. Artifact Hub listing;
  4. project website `generatedsecrets.jabbas.eu` (same as the API group);
  5. Show HN (Hacker News) post.

  This document covers step 2 only.
- Success **[owner]**: people outside the project use it — GitHub stars, issues and pull requests from strangers.
- Audience **[owner + architect]**:
  - Kubernetes users, often platform teams, who need credentials nobody has to know in advance and today either encrypt and commit them (SOPS — Secrets OPerationS, Sealed Secrets) or build a pipeline just to generate them;
  - application developers deploying a new Kubernetes-native application that needs credentials;
  - people building CI/CD (continuous integration and continuous delivery) that must talk to Git (SSH — Secure Shell — deploy keys) or an OCI (Open Container Initiative) registry (htpasswd);
  - Helm chart authors whose `randAlphaNum` + `lookup` breaks in Argo CD and `helm template`;
  - users of abandoned secret generators;
  - preview-environment-per-pull-request setups;
  - home-lab GitOps users.
- The README is the landing page. Research on Show HN shows that posts linking a GitHub repository do better than posts linking a website, so the HN post will link the repository, and the README MUST convert a visitor in about two screens.
- Honest boundary **[architect]**: the operator is for secrets a machine can invent. It does not replace tools for secrets that come from outside (vendor API keys, passwords issued by someone else); SOPS, Sealed Secrets, External Secrets Operator and Vault stay the right tools there. Over-promising loses experienced readers.

## 2. Goals, non-goals, anti-goals

### 2.1 Goals

- README rewritten as a landing page (§3).
- `CONTRIBUTING.md` (§4).
- Issue templates (§5).
- `SECURITY.md` (§6).
- Roadmap section in the README (§3 item 9).
- Demo script in the repository and a re-recorded demo (§7).
- Repository settings and seed issues (§8).

### 2.2 Non-goals

- Website, Artifact Hub listing, HN post — separate projects.
- GitHub Discussions: questions go to issues; no second channel. **[architect]**
- A separate `ROADMAP.md` file: a README section is enough. **[architect]**
- A social preview image: deferred to the website project. **[architect]**
- A code of conduct and a pull-request template: deliberately left out ("if it doesn't exist, it can't break"); `CONTRIBUTING.md` carries the expectations. GitHub's community-profile checklist flagging them is accepted. **[architect]**

### 2.2a Sequencing and who does what **[architect]**

- **Two pull requests.**
  - **Pull request A**, after the rename branch is merged: `CONTRIBUTING.md`, `SECURITY.md`, issue forms, `demo/` with the recording (recorded against `0.2.0-rc.1` or a locally built image), repository settings, labels, seed and roadmap issues (§8).
  - **Pull request B**, after `0.2.0` final is promoted: the README rewrite (§3). The README pins only released versions (`docs/development.md`), so the "Try it in 5 minutes" block uses `0.2.0`, never a release candidate.
- **GitHub-side work (§8) is done by the agent with the `gh` command-line tool**, not by the owner, including enabling private vulnerability reporting. If the rename plan's owner step already set the description and topics, the agent only verifies them against §8.
- Roadmap and seed issues are filed in pull request A's phase, so their numbers exist when pull request B writes the README.
- **[owner]** The README demo GIF links to the recording on asciinema.org (the owner published the previous recording at https://asciinema.org/a/Mp19PiTeiHCqkGSC). That recording still shows the old names, so the new recording (§7) is uploaded to asciinema.org as well and the README links the new URL; the old upload is then deleted by the owner. The website keeps its own vendored player (no third-party requests).
- Code changes to the operator.

### 2.3 Anti-goals (a failure even if it "works")

- Marketing language, superlatives, badge walls — Hacker News readers punish them.
- Claims the documentation cannot back.
- An empty or broken first impression: a blank first frame of the demo, commands that do not work when copied.

## 3. README structure **[architect, approved by owner]**

In this order; items 1–6 SHOULD fit in about two screens.

1. **Name and one sentence**, for example: "Generate Kubernetes Secrets in-cluster: passwords, tokens, SSH keys. Declared next to your app, never stored in Git." A few badges only: latest release, CI status, licence. The Artifact Hub badge is added later by project 3.
2. **Demo GIF** (Graphics Interchange Format) — `docs/assets/demo.gif` (§7); no link for now (§2.2a).
3. **The smallest possible manifest** — about 6 lines, kind `GeneratedSecret`.
4. **"Why"** — three pains the reader recognises:
   - Helm `randAlphaNum` + `lookup` regenerates values under Argo CD and `helm template`;
   - encrypting with SOPS a password no human needs to know;
   - a pipeline built only to generate a key.

   Then the one promise: a value changes only when you decide (definition change, deleting the Secret or key, configured rotation) — never on operator restart, upgrade, cache loss or re-apply.
5. **"Try it in 5 minutes"** — copy-paste commands with no repository clone:
   - `kind create cluster`;
   - `helm repo add` + `helm install` (HTTP repository) into `generated-secrets-system`;
   - `kubectl apply -f - <<EOF … EOF` with an inline manifest;
   - `kubectl wait` for `Ready`;
   - read the generated value.

   The builder MUST run the block verbatim on a fresh kind cluster before merging.
6. **"When to use it, and when not"** — an honest comparison. Facts to use (verified 2026-11-14; the builder re-checks versions and dates when writing):
   - **Sealed Secrets / SOPS:** encrypt values a human already has and commit them to Git. Right for externally issued secrets; for machine-invented ones they add a key-management step for nothing.
   - **External Secrets Operator:** syncs secrets from external stores. Right when the source of truth is Vault, AWS (Amazon Web Services) or similar. Its generators (`Password`, `SSHKey`, …) produce a new value on every invocation by default ("The generator does not keep track of the produced values. Every invocation produces a new set of values." — https://external-secrets.io/latest/guides/generator/). Stable values need `refreshPolicy: CreatedOnce` plus `target.immutable` plus `creationPolicy: Orphan`.
   - **HashiCorp Vault / OpenBao:** a full secrets platform; heavy if all you need is a few generated passwords.
   - **Helm `randAlphaNum` + `lookup`:** `lookup` returns nothing during `helm template` and client dry-run (https://helm.sh/docs/chart_template_guide/functions_and_pipelines/), and Argo CD does not support it (https://github.com/argoproj/argo-cd/issues/5202), so the value regenerates.
   - **mittwald/kubernetes-secret-generator:** the best-known earlier generator (about 400 stars); one release since 2022 (v3.4.1, August 2025); annotation-driven on plain Secrets.
   - **Carvel secretgen-controller:** actively maintained, broader scope (secret export and import across namespaces, certificates) and Carvel-oriented.
   - **When NOT to use Generated Secrets Operator:**
     - secrets issued by someone else;
     - a value that must be known before deployment or must be identical across clusters;
     - generated values live only in the cluster (etcd), so back up the Secrets (for example with Velero) if losing the cluster must not lose them. This is the disaster-recovery question HN readers will ask.

   Keep it factual and polite about other projects.
7. **Features** — the current list, shortened.
8. **Status** — `v1alpha1`; frozen defaults and fingerprint format (upgrades never rotate credentials); cosign-signed multi-architecture releases with SBOM (software bill of materials); every release candidate passes a test on a real cluster before promotion.
9. **Roadmap** — a short section from the "Future work" list of the 2026-10-18 design: templates and derived keys, namespace label-selector scoping, additional generators, `v1beta1`. Each item SHOULD link an issue (§8).
10. **Documentation table, Contributing** (link to `CONTRIBUTING.md`), **Licence**.

## 4. `CONTRIBUTING.md` **[owner + architect]**

- **Welcome [owner]:** bug reports and fixes, new features, things to fix (documentation, tests, examples).
- **Will not be accepted [architect]** — each with its reason, taken from the anti-goals of the 2026-10-18 design:
  - annotation-driven generation on plain Secrets (the only input is `GeneratedSecret`, the output is a Secret);
  - deterministic or seed-derived values;
  - anything that makes a value change on its own (restart, upgrade, re-apply, cache loss, leader change) or changes frozen defaults;
  - broader permissions (`delete` on Secrets, caching all Secrets in the cluster);
  - operator logic that knows about Helm, Flux, Argo CD or other deployment tools;
  - secret values in logs, Events, status, metrics or error messages;
  - fetching secrets from external systems (that is External Secrets Operator's job).
- **Process [architect]:**
  - bug fixes and small improvements — open a pull request directly;
  - new features — open an issue first with the desired YAML, because the API is public and hard to change.
- **AI-assisted contributions [architect]:** allowed. The author MUST understand and have tested what they submit. Pull requests that look generated without understanding are closed without long discussion.
- **Development setup:** link `docs/development.md`; commit messages in English; licence Apache-2.0, contributions under the same licence (inbound = outbound).

## 5. Issue templates **[architect]**

`.github/ISSUE_TEMPLATE/` with three issue forms:

- **Bug report:** operator version, Kubernetes version, installation method, the `GeneratedSecret` manifest, conditions and Events from `kubectl describe gensecret`, and a prominent warning never to paste Secret values.
- **Feature request:** use case, desired YAML sketch, alternatives considered.
- **Question.**

`config.yml` links to the documentation, and to `SECURITY.md` for vulnerabilities.

## 6. `SECURITY.md` **[architect]**

A short file at the repository root:

- how to report a vulnerability privately (GitHub private vulnerability reporting, enabled in repository settings — §8);
- supported versions (latest minor);
- link to `docs/security.md`.

Justification: this is a security-sensitive tool; strangers look for this file before reporting or adopting.

## 7. Demo **[owner + architect]**

- **Script in the repository:** `demo/demo.sh` plus `demo/README.md` describing how to record. Every future version MUST be re-recordable with one command; the original `demo.sh` was lost.
- **Recorded by an agent, not the owner:** kind cluster (podman provider), operator installed (`0.2.0-rc.1` or a locally built image), `asciinema rec` running the script, GIF made with `agg`. The owner reviews the result.
- **First frame [owner]:** immediately shows the title ("Generated Secrets Operator: Kubernetes Secrets generated in-cluster") and a short manifest, printed instantly rather than typed, so a static first frame is never an empty terminal.
- **Scenes [owner]:**
  1. Password with `deletionPolicy: Retain`: delete the `GeneratedSecret`, the Secret stays; recreate it, same password.
  2. NEW: `kubectl rollout restart` of the operator, same password — the strongest proof of the promise.
  3. SSH key: delete the Secret, a new key pair appears.
  4. Rotated token: changing the definition yields a new value; Events show `Regenerated`.
- **Format:** about 90 seconds at most; terminal 110×34 like the previous recording; GIF small enough to load quickly (aim for ≤ 3 MB).
- **Outputs:** `docs/assets/demo.gif` (used in the README) and `docs/assets/demo.cast` (for the website project).
- The old untracked `secretspec-demo.cast` and `secretspec-demo.gif` at the repository root are deleted locally once the new recording is accepted.

## 8. Repository settings and seed issues **[architect]**

Done by the agent with the `gh` command-line tool (§2.2a).

- **Description:** one sentence matching README item 1.
- **Topics:** `kubernetes`, `kubernetes-operator`, `secrets`, `secret-generator`, `password-generator`, `rust`, `helm`, `gitops`.
- **Homepage URL:** set once the website exists (project 4).
- Enable private vulnerability reporting.
- **Labels:** `good first issue`, `help wanted` (GitHub defaults; create them if missing).
- **Seed issues:** 3–5 issues labelled `good first issue` from the known backlog, so a first-time contributor has somewhere to start. Each issue states the problem, where the code is, and how to verify. Candidates (verify each is still open before filing):
  - the CRD (custom resource definition) `format: uint16` triggers a Helm 4 warning;
  - Renovate `customManagers` for pinned tool versions (cosign, syft, grype, cargo-auditable) — skip if already done;
  - documentation polish items.
- **Roadmap issues:** the items from §3 item 9 filed as issues (label `enhancement`), so the README can link them.

## 9. Done criteria

- README items 1–10 are present; the "Try it in 5 minutes" block was run verbatim on a fresh kind cluster and works.
- `CONTRIBUTING.md`, `SECURITY.md`, the three issue forms and `config.yml` are merged; the forms render on GitHub's "New issue" page.
- `demo/demo.sh` reproduces the recording; `docs/assets/demo.gif` and `docs/assets/demo.cast` are committed; the first frame is not empty; length ≤ about 90 seconds.
- Repository description, topics, private vulnerability reporting, labels and seed issues are in place.

## 10. Left to the builder

- Exact wording of all texts.
- Badge selection within the limit in §3 item 1.
- Issue form field details.
- Demo pacing and colours.
- Which backlog items become `good first issue` after checking they are still open.
