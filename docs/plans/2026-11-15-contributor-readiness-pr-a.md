# Contributor Readiness, Pull Request A — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the repository ready for outside contributors: `CONTRIBUTING.md`, `SECURITY.md`, issue forms, a reproducible demo recording, labels, seed and roadmap issues (pull request A of the contributor-readiness design; the README rewrite is pull request B, after 0.2.0 final).

**Architecture:** One branch `docs/contributor-readiness` in the OpenCode worktree `/Users/jabbas/.local/share/opencode/worktree/74226b/contributor-readiness` (cut from `main` at `2f6c8b1`). Tasks 1–3 produce the files in the pull request; Task 4 is GitHub-side work through `gh` (issues and labels, not in the diff). Task 5 is the whole-branch review and the pull request. Task 6 is the owner's review of the recording, the merge and the post-merge checks. No operator code changes.

**Tech Stack:** Markdown, GitHub issue forms (YAML), Bash, kind (podman provider), Helm 4, kubectl, asciinema 3.x (`asciinema rec`), agg 1.9 (GIF), `gh`, `uvx check-jsonschema` (issue-form schema validation), `shellcheck` if installed.

**Spec:** `docs/design/2026-11-15-contributor-readiness-design.md` (approved by the owner, 2026-11-15) — §2.2a, §4, §5, §6, §7, §8, §9. §3 (README) is NOT part of this plan.

## Global Constraints

- English everywhere; no marketing language, no superlatives, no abbreviations without expansion on first use in prose (repository rule).
- Names: project "Generated Secrets Operator", kind `GeneratedSecret`, short name `gensecret`, API group `generatedsecrets.jabbas.eu`, chart `generated-secrets-operator`, namespace `generated-secrets-system`, repository `jabbas/generated-secrets-operator`, website (not live yet) `generatedsecrets.jabbas.eu`.
- No change to operator code, chart, workflows or release scripts.
- New shell scripts: `#!/usr/bin/env bash`, `# SPDX-License-Identifier: Apache-2.0`, `set -euo pipefail`, full descriptive variable names, every failure reported (never swallowed); follow the style of `.github/scripts/*.sh`.
- Never print or commit real Secret values. Values generated inside the throwaway demo cluster are not secrets and may appear in the recording.
- Never run `gh auth token`. No force pushes. No tags. Merge only in Task 6.
- Commit messages: English, Conventional Commits (`docs: …`, `feat(demo): …`).
- The main clone `/Users/jabbas/Projects/secretspec-operator` belongs to another session (release of 0.2.0): do not edit, switch or stash there, except deleting the two untracked old demo files in Task 6.

## Review Focus

1. **`demo/record.sh` on a machine missing a tool** (kind, helm, kubectl, kubecolor, yq, asciinema, agg) — expected: exits non-zero before creating anything, naming the missing tool. Test in Task 3.
2. **`demo/record.sh` without a chart version, or when the demo cluster already exists** — expected: usage error / refusal with a clear message, no cluster created or deleted. Test in Task 3.
3. **A scene's state change is slow** (operator restart, regeneration after a patch) — expected: the script waits with `kubectl wait` or a bounded poll that fails with a message on timeout; never a fixed sleep standing in for a state change, never a silently wrong recording. Checked in Task 3 (review of the script) and by the recorded output.
4. **A bug reporter pastes Secret data** — expected: the bug form only asks for commands whose output contains no Secret data (`kubectl describe gensecret …`, `kubectl get events …`, Secret metadata only), warns prominently never to paste values, and asks the reporter to replace every value under `spec.data` with `<redacted>` both in the pasted manifest and in `kubectl describe` output (which prints the spec and can echo those values). Test in Task 2.
5. **Broken links in `CONTRIBUTING.md` / `SECURITY.md`** (relative paths and anchors such as `docs/security.md#reporting-a-vulnerability`) — expected: every link resolves. Test in Task 1.

---

### Task 1: `CONTRIBUTING.md`, `SECURITY.md`, design-document fixes

**Files:**
- Create: `CONTRIBUTING.md`, `SECURITY.md`
- Modify: `docs/design/2026-11-15-contributor-readiness-design.md` (two fixes below)

**Interfaces:**
- Produces: `CONTRIBUTING.md` and `SECURITY.md` at the repository root; Task 2's `config.yml` and Task 4's issues link to them by their GitHub URLs (`https://github.com/jabbas/generated-secrets-operator/blob/main/CONTRIBUTING.md`, `https://github.com/jabbas/generated-secrets-operator/security/policy`).

- [ ] **Step 1:** Create the branch: `git switch -c docs/contributor-readiness` (the worktree is detached at `2f6c8b1`; confirm `git rev-parse HEAD` equals `origin/main` after `git fetch origin`, otherwise `git switch -c docs/contributor-readiness origin/main`).
- [ ] **Step 2:** Write `CONTRIBUTING.md` with exactly the content decided in design §4: welcome list [owner]; the seven "will not be accepted" items, each with its one-line reason; process (bug fixes and small improvements → pull request directly; new features → issue first with the desired YAML, because the API is public and hard to change); AI-assisted contributions (allowed; author must understand and have tested the change; pull requests that look generated without understanding are closed without long discussion); development setup → `docs/development.md`; commit messages in English; Apache-2.0, inbound = outbound. Also point to the issue forms (Task 2) and to `SECURITY.md` for vulnerabilities. Keep it to about one screen and a half.
- [ ] **Step 3:** Write `SECURITY.md` per design §6: report privately through GitHub private vulnerability reporting ("Report a vulnerability" on the Security tab; enabled 2026-11-15); never open a public issue for a vulnerability; supported versions = latest minor release (0.2.x once released); link `docs/security.md` (threat model, redaction guarantees, release signing). Reuse the facts of `docs/security.md` "Reporting a vulnerability" — do not contradict it; if that section says something different (for example an e-mail address), report the conflict instead of guessing.
- [ ] **Step 4:** Fix the design document: (a) move the stray bullet "Code changes to the operator." from the end of §2.2a back into the §2.2 non-goals list; (b) in §3 item 2 replace "no link for now (§2.2a)" with "linking to the recording on asciinema.org (§2.2a)".
- [ ] **Step 5 (Review Focus 5):** Verify every link: for each relative link in `CONTRIBUTING.md` and `SECURITY.md`, the target file exists; for each `#anchor`, the heading exists (GitHub slug: lowercase, spaces → `-`, punctuation dropped). Command example: `grep -n '^## Reporting a vulnerability' docs/security.md`. Expected: every target found; list them in the report.
- [ ] **Step 6:** Commit (this plan file travels with the branch): `git add CONTRIBUTING.md SECURITY.md docs/design/2026-11-15-contributor-readiness-design.md docs/plans/2026-11-15-contributor-readiness-pr-a.md && git commit -m "docs: add CONTRIBUTING.md and SECURITY.md"`.

### Task 2: Issue forms

**Files:**
- Create: `.github/ISSUE_TEMPLATE/bug_report.yml`, `.github/ISSUE_TEMPLATE/feature_request.yml`, `.github/ISSUE_TEMPLATE/question.yml`, `.github/ISSUE_TEMPLATE/config.yml`

**Interfaces:**
- Consumes: the URLs from Task 1.
- Produces: labels used by the forms — `bug`, `enhancement`, `question` (GitHub defaults; Task 4 verifies they exist).

- [ ] **Step 1:** `bug_report.yml` (label `bug`): a markdown block at the top with the warning, verbatim in substance: "Never paste Secret values. Generated Secrets Operator never needs them to diagnose a problem." Required fields: operator version; Kubernetes version; installation method (dropdown: Helm HTTP repository, Helm OCI, Flux, Argo CD, other); the `GeneratedSecret` manifest (render: yaml), with every value under `spec.data` replaced by `<redacted>` because those values are copied into the Secret; output of `kubectl describe gensecret <name> -n <namespace>` (conditions and Events), with `spec.data` values replaced by `<redacted>` too, because `describe` prints the spec and can echo them; what you expected / what happened. Optional: operator logs (`kubectl logs -n generated-secrets-system deployment/<deployment name>` — the Deployment is named after the release name if that contains `generated-secrets-operator`, otherwise `<release name>-generated-secrets-operator`; the operator never logs values). Do NOT ask for `kubectl get secret … -o yaml/json` or anything that prints `.data`.
- [ ] **Step 2:** `feature_request.yml` (label `enhancement`): use case (required), desired YAML sketch of the `GeneratedSecret` (required, render: yaml), alternatives considered, a note linking `CONTRIBUTING.md` "will not be accepted" list.
- [ ] **Step 3:** `question.yml` (label `question`): question (required), what you already read (optional), versions (optional).
- [ ] **Step 4:** `config.yml`: `blank_issues_enabled: false`; `contact_links`: "Documentation" → `https://github.com/jabbas/generated-secrets-operator/tree/main/docs`, "Report a security vulnerability" → `https://github.com/jabbas/generated-secrets-operator/security/policy`.
- [ ] **Step 5:** Validate against the published schemas: `uvx check-jsonschema --builtin-schema vendor.github-issue-forms .github/ISSUE_TEMPLATE/bug_report.yml .github/ISSUE_TEMPLATE/feature_request.yml .github/ISSUE_TEMPLATE/question.yml` and `uvx check-jsonschema --builtin-schema vendor.github-issue-config .github/ISSUE_TEMPLATE/config.yml`. Expected: `ok -- validation done`. (If `uvx` is unavailable, report it and validate YAML syntax with `yq` instead.)
- [ ] **Step 6 (Review Focus 4):** `grep -nE 'get secret|-o (yaml|json)|\.data' .github/ISSUE_TEMPLATE/*.yml`. Expected: no line asks the reporter to print Secret data (the only allowed hit is the warning text itself).
- [ ] **Step 7:** Commit: `feat(github): add issue forms`.

### Task 3: Demo script and recording

**Files:**
- Create: `demo/demo.sh` (the scenes; what is recorded), `demo/record.sh` (one command: cluster → install → record → GIF → cleanup), `demo/README.md`
- Create (outputs): `docs/assets/demo.cast`, `docs/assets/demo.gif`

**Interfaces:**
- Produces: `bash demo/record.sh <chart-version>` writes `docs/assets/demo.cast` and `docs/assets/demo.gif`. `demo/demo.sh` assumes the current kubectl context has the operator installed in `generated-secrets-system` and creates/uses namespace `demo`.

- [ ] **Step 1:** Write `demo/record.sh` with this contract:
  - Argument 1: chart version (required; for example `0.2.0-rc.2`). No argument → print usage, exit 2.
  - Checks before doing anything: `kind`, `helm`, `kubectl`, `asciinema`, `agg` on `PATH`; missing → `"<tool> not found on PATH"`, exit 1.
  - Cluster name `generated-secrets-demo`; if a kind cluster with that name already exists → refuse with a message (never delete a cluster the script did not create). Create it; a `trap` on EXIT deletes only the cluster this run created. The kind provider comes from the environment (podman on the owner's machine); the script does not set it.
  - Install: `helm install generated-secrets-operator oci://ghcr.io/jabbas/charts/generated-secrets-operator --version "$chart_version" --namespace generated-secrets-system --create-namespace --wait`; label the namespace for Pod Security `restricted` as `docs/installation.md` does.
  - Record: `asciinema rec` running `demo/demo.sh` with a 110×34 terminal, idle time limit 2 s, title "Generated Secrets Operator demo", output `docs/assets/demo.cast` (overwrite). Check the exact asciinema 3 flags with `asciinema rec --help`.
  - GIF: `agg` from the cast to `docs/assets/demo.gif`; choose theme/font size so the GIF is ≤ 3 MB (check `agg --help`).
- [ ] **Step 2:** Write `demo/demo.sh`:
  - A small `type_and_run` helper (simulated typing of a command, then run it) and a `comment` helper (dim typed comment), like the previous recording; pacing is the builder's choice.
  - **First frame [owner]:** clear the screen and print instantly (no typing) the title "Generated Secrets Operator: Kubernetes Secrets generated in-cluster" and the manifest of `examples/app-admin-password.yaml` without its comment header.
  - Scenes, in namespace `demo` [owner]: (1) apply `app-admin` (`deletionPolicy: Retain`), wait Ready, read password; delete the `GeneratedSecret`; show the Secret still exists; re-apply; wait Ready; read password — identical. (2) `kubectl rollout restart` of the operator Deployment in `generated-secrets-system` and `kubectl rollout status`; read password — identical. (3) apply `deploy-key` (`examples/ssh-deploy-key.yaml`), read the public key; delete the Secret; wait until it is recreated; read the public key — different. (4) apply `shared-token` (`examples/rotated-shared-token.yaml`), read the token; `kubectl patch gensecret shared-token --type merge` changing `length` from 48 to 64; wait until the value changes; read it; show Events with reason `Regenerated`.
  - Every wait for a state change uses `kubectl wait --for=…` or a bounded poll (for example 60 s) that prints a message and exits non-zero on timeout (Review Focus 3). Typing delays may use `sleep`; state changes may not.
  - Total length about 90 s at most after the 2 s idle limit.
- [ ] **Step 3 (Review Focus 1, 2):** Tests of the guard rails, without creating a cluster:
  - `bash demo/record.sh` → exit 2, usage on stderr.
  - `env PATH=/usr/bin:/bin bash demo/record.sh 0.2.0-rc.2` → exit 1, message names the first missing tool, `kind get clusters` unchanged.
  - With a cluster named `generated-secrets-demo` pre-created (`kind create cluster --name generated-secrets-demo`), `bash demo/record.sh 0.2.0-rc.2` → refuses, cluster still exists afterwards; then delete it.
  - `shellcheck demo/*.sh` if installed (otherwise `bash -n demo/*.sh`). Expected: clean.
- [ ] **Step 4:** Pick the chart version: `crane ls ghcr.io/jabbas/charts/generated-secrets-operator` — use `0.2.0` if it exists, else the highest `0.2.0-rc.N` (rc.2 is expected soon; rc.1 is functionally identical). Run `bash demo/record.sh <version>`. Expected: exit 0, both files written, cluster deleted.
- [ ] **Step 5:** Check the recording:
  - Duration with the idle limit: `tail -n +2 docs/assets/demo.cast | jq -s 'map(select(.[1]=="o") | .[0] | if . > 2 then 2 else . end) | add'` (asciicast v3 intervals; adjust if the file is v2). Expected ≤ ~90.
  - First frame: `tail -n +2 docs/assets/demo.cast | head -n 5 | grep -c 'GeneratedSecret'` ≥ 1.
  - `ls -l docs/assets/demo.gif` ≤ 3 MB. Look at the GIF's first and last frames (render a PNG of the first frame if a tool is at hand) and report.
  - Scene outcomes visible in the cast: password identical after re-create and after restart; SSH key different; token changed; `Regenerated` event present.
- [ ] **Step 6:** `demo/README.md`: prerequisites (tools, kind podman provider), the one command, outputs, how to upload to asciinema.org (`asciinema upload docs/assets/demo.cast` — done by the owner or with the owner's consent, Task 6), and that the GIF is used in the README and the cast by the website.
- [ ] **Step 7:** Commit: `feat(demo): add reproducible demo recording` (scripts, README, `docs/assets/demo.cast`, `docs/assets/demo.gif`).

### Task 4: GitHub-side work (labels, seed issues, roadmap issues)

**Files:** none in the diff (record what was filed in the pull request description in Task 6).

**Interfaces:**
- Consumes: Task 1 URLs; labels from Task 2.
- Produces: issue numbers of the roadmap issues (pull request B links them from the README); the list goes into the pull request description.

- [ ] **Step 1:** Verify settings already applied on 2026-11-15 (report, do not change): `gh repo view jabbas/generated-secrets-operator --json description,repositoryTopics` matches design §8; `gh api repos/jabbas/generated-secrets-operator/private-vulnerability-reporting` → `{"enabled":true}`.
- [ ] **Step 2:** Labels: `gh label list --repo jabbas/generated-secrets-operator`; create any of `bug`, `enhancement`, `question`, `good first issue`, `help wanted` that is missing (GitHub default colours and descriptions).
- [ ] **Step 3:** Seed issues (3–5, label `good first issue`): check each candidate is still real on `origin/main` before filing — the CRD `format: uint16` Helm 4 warning (grep the CRD under `charts/generated-secrets-operator/files/crd/` and run `helm lint` / `helm template` to see the warning); Renovate `customManagers` for pinned tool versions (cosign, syft, grype, cargo-auditable — check `renovate.json` and `.github/scripts/install-release-tools.sh`; skip if covered); documentation polish items you find while doing Tasks 1–3 or that are listed as deferred minors in `/Users/jabbas/Projects/secretspec-operator/.superpowers/sdd/2026-11-15-rename-to-generated-secrets-operator/progress.md` (read-only). Each issue body: problem, where the code is (paths), how to verify the fix, link to `CONTRIBUTING.md`. Search open issues first (`gh issue list`) to avoid duplicates.
- [ ] **Step 4:** Roadmap issues (label `enhancement`), one each, from the 2026-10-18 design "Future work": templates and derived keys; namespace label-selector scoping; additional generators; `v1beta1`. Each body: what it means, why it is not in v1alpha1 yet, an invitation to discuss the YAML first (`CONTRIBUTING.md` process).
- [ ] **Step 5:** Record the filed issue numbers and titles for Task 6.

### Task 5: Whole-branch review and pull request

- [ ] **Step 1:** Push the branch, open the pull request: `gh pr create --base main --head docs/contributor-readiness --title "docs: prepare the repository for outside contributors"`; body: summary per task, the filed issues (Task 4), the recording facts (version recorded, duration, GIF size), link to the design and this plan. Wait for CI (`gh pr checks --watch`).
- [ ] **Step 2:** Whole-branch review by `@reviewer` from a session located in this worktree (it can read files and run the Task 3 guard-rail tests). Fix findings; re-review fixes.

### Task 6: Owner review, merge, post-merge checks

- [ ] **Step 1 — STOP (owner):** show the owner `docs/assets/demo.gif` (path in the worktree) and the pull request link; wait for approval or change requests (re-record via `bash demo/record.sh <version>`).
- [ ] **Step 2:** Merge with a merge commit, remote branch deleted, without switching any local checkout: from a neutral directory, `gh pr merge <number> --repo jabbas/generated-secrets-operator --merge --match-head-commit <full sha> --delete-branch`. Confirm main CI green.
- [ ] **Step 3:** Post-merge: open `https://github.com/jabbas/generated-secrets-operator/issues/new/choose` (or `gh api repos/jabbas/generated-secrets-operator/contents/.github/ISSUE_TEMPLATE`) and confirm the three forms and two contact links are offered; the community profile (`gh api repos/jabbas/generated-secrets-operator/community/profile`) lists CONTRIBUTING and SECURITY (code of conduct and PR template missing is expected — design §2.2).
- [ ] **Step 4 — STOP (owner consent):** upload the new recording to asciinema.org (`asciinema upload docs/assets/demo.cast`, from the owner's machine where the asciinema install ID is linked to the owner's account) only with the owner's explicit yes; report the URL for pull request B. The owner deletes the old upload `https://asciinema.org/a/Mp19PiTeiHCqkGSC`.
- [ ] **Step 5:** Delete the old untracked `secretspec-demo.cast` and `secretspec-demo.gif` from `/Users/jabbas/Projects/secretspec-operator` (design §7) — only after Step 1 approval.
