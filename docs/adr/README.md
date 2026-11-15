# Architecture Decision Records

> **Note:** the project was renamed to Generated Secrets Operator in 0.2.0 ([ADR 0006](0006-rename-to-generated-secrets-operator.md)). Names in older records are historical; the [2026-10-18 design](../design/2026-10-18-secretspec-operator-design.md) remains the reference for behaviour, read through the name map of the [2026-11-15 rename design](../design/2026-11-15-rename-to-generated-secrets-operator-design.md).

An ADR records one significant decision: the context that forced it, the decision itself, the alternatives that were rejected, and the consequences. Each record is a short Markdown file named `NNNN-title.md` (numbered in order, never reused) with the sections Title, Status, Date, Context, Decision, Alternatives considered and Consequences. An accepted ADR is not rewritten; a later decision supersedes it with a new ADR, and the old status is changed to `Superseded by NNNN`.

| ADR | Title | Status |
|---|---|---|
| [0001](0001-semver-release-tags.md) | Semver release tags without the `v` prefix | Accepted |
| [0002](0002-http-helm-repository.md) | HTTP Helm repository channel | Accepted |
| [0003](0003-scratch-base-image.md) | Build the image from scratch | Accepted |
| [0004](0004-release-gate-from-main-ci.md) | Release gate from main CI | Accepted |
| [0005](0005-promote-release-candidates.md) | Promote release candidates | Accepted |
| [0006](0006-rename-to-generated-secrets-operator.md) | Rename to Generated Secrets Operator | Accepted |
