# ADR 0006: Rename to Generated Secrets Operator

- **Status:** Accepted
- **Date:** 2026-11-15
- **Decided by:** the project owner

## Context

- The project was called "SecretSpec Operator", with the custom resource kind `SecretSpec`. That name collides with cachix/secretspec (secretspec.dev), a Rust secrets tool with about 1.6k GitHub stars and its own ecosystem: packages on crates.io, npm and PyPI, a GitHub Action, an IDE plugin, "declarative secret generation" since its 0.7 (February 2026), a Kubernetes provider since 0.20, and several Hacker News front-page posts. Search results and discussion would conflate the two projects.
- The rename has to happen before any public promotion (project website, Artifact Hub listing, Show HN post). The project is still 0.x and `v1alpha1`, and there are no installations with resources: the owner's home lab runs operator 0.1.5 through Flux but has no `SecretSpec` objects. This is the cheapest moment the rename will ever have.
- The kind `GeneratedSecret` was used before by four small projects (1–5 stars). krystal/generated-secrets (archived 2023) even used the title "Generated Secrets Operator"; containerinfra/kube-secrets-operator is active. There is no technical collision, because the API group differs.

## Decision

The project is renamed **Generated Secrets Operator** in release 0.2.0, and the kind `SecretSpec` becomes `GeneratedSecret`. The name follows the ecosystem convention External Secrets Operator / `ExternalSecret`, Sealed Secrets / `SealedSecret`, and serves the original design goal of a self-explaining API.

- Every user-visible name moves according to the name map of the [2026-11-15 rename design](../design/2026-11-15-rename-to-generated-secrets-operator-design.md): repository, API group (`generatedsecrets.jabbas.eu`), kind, plural, short name (`gensecret`), CRD, crates, binary, image, charts, labels, annotations, field manager, install namespace (`generated-secrets-system`), metrics prefix, environment variables and release asset names. API shape and behaviour stay identical apart from names.
- The label and annotation that name the managing resource become `generatedsecrets.jabbas.eu/owner-name` and `owner-uid` ("owner" means the GeneratedSecret that manages the Secret, whether or not an ownerReference exists).
- **Fingerprints.** The header line of every canonical text changes from `secretspec-fingerprint-v1` to `generated-secrets-fingerprint-v1`, and the golden vectors are recomputed. This is safe although the format was declared frozen: the 0.2.0 operator reads fingerprints only from the `generatedsecrets.jabbas.eu/fingerprints` annotation, which no 0.1.x Secret carries, so it can never compare against an old-header fingerprint. The "frozen" promise applies from 0.2.0 on.
- **No migration and no compatibility code.** The 0.2.0 operator does not recognise anything created by 0.1.x. The changelog has a "Moving from 0.1.x" section.
- **Historical records keep the old name:** ADRs 0001–0005, the implementation plans, the 0.1.x home-lab reports, the 2026-10-18 design and the 0.1.x changelog entries. A one-line note in the ADR index and in the 2026-10-18 design points here.
- **Published artefacts stay as they are**, so old versions stay installable: GitHub releases and tags 0.1.0–0.1.5, the GitHub Container Registry packages `secretspec-operator` and `charts/secretspec-operator`, and the `secretspec-operator` entries of the Helm repository index. The new packages are created by the first 0.2.0 release candidate.

## Alternatives considered

- **Keep the name, add a "not affiliated with cachix/secretspec" note.** Rejected: a disclaimer does not fix search results, package-registry searches or discussion threads, and the collision would grow with the other project's reach.
- **A distinctive brand name (for example Keysmith) with kind `GeneratedSecret`.** Rejected: a brand name has to be learned, whereas `GeneratedSecret` next to `ExternalSecret` and `SealedSecret` explains itself. A brand that differs from the kind would also split the name between the project and the API.
- **Compatibility code that reads the old labels and annotations (or serves both API groups).** Rejected: there are no installations with resources to protect, so the code would be a permanent component without a requirement, and it would keep "secretspec" alive on user-visible surfaces.

## Consequences

- Release 0.2.0 is **breaking**. There is no automatic migration; the steps are in the changelog ("Moving from 0.1.x"). Secrets created by 0.1.x are not adopted, because they carry the old labels. With `deletionPolicy: Retain` they remain as plain Secrets, and a GeneratedSecret with the same name reports `Conflict` until the old Secret is deleted or renamed.
- Accepted risk: a cluster that runs another operator with a `GeneratedSecret` kind needs the fully qualified form `kubectl get generatedsecrets.generatedsecrets.jabbas.eu`.
- Release freeze: from the repository rename until the rename branch is merged, no release tag is pushed. At the time of the decision the reason was that the signing identity and the image source annotation derive from the new repository URL while the release workflow still compared them against the old one. The rename branch has since updated that workflow, but the freeze holds until the rename is merged.
- Release 0.2.0 goes through the normal process: release candidate, home-lab test, promotion ([ADR 0005](0005-promote-release-candidates.md)). Anonymous pull of the new image and chart is verified after the first release candidate.
- Old links keep working: GitHub redirects the former repository URLs, and old chart versions stay installable.
- Promotion work (landing-page README, website, Artifact Hub listing, new demo recording) is separate follow-up work.
