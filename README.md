# Generated Secrets Operator

[![Latest release](https://img.shields.io/github/v/release/jabbas/generated-secrets-operator)](https://github.com/jabbas/generated-secrets-operator/releases/latest)
[![CI](https://github.com/jabbas/generated-secrets-operator/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/jabbas/generated-secrets-operator/actions/workflows/ci.yml)
[![Licence: Apache-2.0](https://img.shields.io/github/license/jabbas/generated-secrets-operator)](LICENSE)

**Generate Kubernetes Secrets in-cluster: passwords, tokens, SSH keys.
Declared next to your app, never stored in Git.**

[![Terminal recording: a generated password survives deleting the GeneratedSecret and restarting the operator](docs/assets/demo.gif)](https://asciinema.org/a/RECORDING_ID)

You write a small `GeneratedSecret` next to your application. The operator
creates the `Secret` with the values, once, and then leaves them alone:

```yaml
apiVersion: generatedsecrets.jabbas.eu/v1alpha1
kind: GeneratedSecret
metadata:
  name: app-admin            # the Secret gets the same name and namespace
spec:
  fields:
    - name: password         # 32 random alphanumeric characters by default
```

## Why

- Helm's `randAlphaNum` with `lookup` generates a new value under Argo CD and
  `helm template`, because `lookup` returns nothing there.
- Encrypting a password with SOPS (Secrets OPerationS) or Sealed Secrets is
  pointless when no human needs to know it.
- A pipeline that exists only to generate a key is a pipeline to maintain.

The one promise: **a value changes only when you decide** — you change its
definition, delete the Secret (or one key), or configure a rotation. It does
not change when the operator restarts or is upgraded, when its cache is lost,
or when you re-apply the same manifest.

## Try it in 5 minutes

You need `kind`, `helm` and `kubectl`. No clone of this repository is needed.

```sh
kind create cluster
helm repo add jabbas https://jabbas.github.io/helm-charts
helm install generated-secrets-operator jabbas/generated-secrets-operator --version 0.2.0 --namespace generated-secrets-system --create-namespace --wait
kubectl apply -f - <<'EOF'
apiVersion: generatedsecrets.jabbas.eu/v1alpha1
kind: GeneratedSecret
metadata:
  name: app-admin
spec:
  deletionPolicy: Retain
  data:
    username: admin
  fields:
    - name: password
EOF
kubectl wait --for=condition=Ready gensecret/app-admin --timeout=60s
kubectl get secret app-admin -o jsonpath='{.data.password}' | base64 -d; echo
```

Apply the manifest again, or run `kubectl rollout restart deployment -n
generated-secrets-system`, and read the password again: it is the same. Clean
up with `kind delete cluster`.

The HTTP chart repository is not signed. For the cosign-signed chart, install
from `oci://ghcr.io/jabbas/charts/generated-secrets-operator`; both channels
and how to verify them are in [Installation](docs/installation.md).

## When to use it, and when not

Generated Secrets Operator is for secrets a machine can invent: passwords,
tokens, random keys, SSH key pairs, basic-auth credentials. Other tools fit
other jobs:

| Tool | What it does | How it relates |
|---|---|---|
| [Sealed Secrets](https://github.com/bitnami-labs/sealed-secrets), [SOPS](https://github.com/getsops/sops) | Encrypt values a human already has, so they can be committed to Git. | The right choice for externally issued secrets. For machine-invented ones they add key management for no benefit. |
| [External Secrets Operator](https://external-secrets.io) | Syncs secrets from an external store (Vault, Amazon Web Services and others). | The right choice when the source of truth is such a store. Its [generators](https://external-secrets.io/latest/guides/generator/) say: "The generator does not keep track of the produced values. Every invocation produces a new set of values." Stable values need extra settings (`refreshPolicy: CreatedOnce` and an immutable, orphaned target Secret). |
| [HashiCorp Vault](https://www.vaultproject.io), [OpenBao](https://openbao.org) | A full secrets platform. | Heavy if all you need is a few generated passwords. |
| Helm `randAlphaNum` + `lookup` | Generates a value at render time. | [`lookup` returns nothing during `helm template` and client dry-run](https://helm.sh/docs/chart_template_guide/functions_and_pipelines/), and [Argo CD does not support it](https://github.com/argoproj/argo-cd/issues/5202), so the value is regenerated. |
| [mittwald/kubernetes-secret-generator](https://github.com/mittwald/kubernetes-secret-generator) | The best-known earlier generator; annotation-driven, on plain Secrets. | Its latest release is v3.4.1 (August 2025); the one before dates from April 2022. |
| [Carvel secretgen-controller](https://github.com/carvel-dev/secretgen-controller) | Maintained; broader scope (secret export and import across namespaces, certificates), oriented to Carvel. | Choose it if you already use Carvel. |

**When not to use Generated Secrets Operator:**

- The secret is issued by someone else (a vendor API key, a password from
  another team).
- The value must be known before deployment, or must be identical across
  clusters.
- You cannot lose the values with the cluster. Generated values live only in
  the cluster (in etcd, the Kubernetes data store), so back up the Secrets, for
  example with [Velero](https://velero.io), if losing the cluster must not lose
  them.

## Features

- **Safe by default.** Values are generated once and stay stable; see the
  [behaviour table](docs/api-reference.md#behaviour-table).
- **Three kinds of secret**, one per `GeneratedSecret`: `fields` (random
  strings, tokens and encoded random bytes), `sshKeyPair` (ed25519, RSA or
  ECDSA) and `basicAuth` (username, password and a bcrypt `htpasswd` line).
- **Per-key control.** Length, character set, excluded characters, minimum
  counts per character class, prefix and suffix; or N random bytes in hex,
  base64, base64url or base32.
- **Optional scheduled rotation** (`rotation.interval`), and manual rotation by
  deleting the Secret.
- **`deletionPolicy`:** `Delete` (default, garbage-collected with the
  `GeneratedSecret`) or `Retain` (the Secret survives, and a re-created
  `GeneratedSecret` adopts it and keeps its values).
- **Mistakes surface where you look.** A bad manifest is rejected at
  `kubectl apply` / `helm install` time by the CRD schema and CEL rules; runtime
  problems appear in `status.conditions` and Events.
- **Least privilege.** No `delete` on Secrets, only metadata of managed Secrets
  is cached, values never reach logs, Events, status or metrics, and the
  operator never touches a Secret it did not create. Cluster-wide or
  per-namespace RBAC (role-based access control; a real boundary).
- **Production packaging.** Multi-arch image (`linux/amd64`, `linux/arm64`),
  OCI Helm chart, leader election for a highly available deployment, Prometheus
  metrics, Pod Security `restricted`, cosign-signed releases with an SBOM
  (software bill of materials).

## Status

The API is `generatedsecrets.jabbas.eu/v1alpha1`. Defaults and the fingerprint
format are frozen for this version, so upgrades never rotate your credentials.
Breaking changes before `v1beta1` are possible but are always called out in the
[changelog](CHANGELOG.md). Releases are cosign-signed, multi-architecture and
carry an SBOM; every release candidate passes a test on a real cluster before
it is promoted. Supported Kubernetes: 1.34 and newer.

The project was renamed from SecretSpec Operator in 0.2.0, a breaking change;
see [Moving from 0.1.x](CHANGELOG.md#moving-from-01x) and
[ADR 0006](docs/adr/0006-rename-to-generated-secrets-operator.md).

## Roadmap

- [Templates and derived keys](https://github.com/jabbas/generated-secrets-operator/issues/27)
- [Namespace scoping by label selector](https://github.com/jabbas/generated-secrets-operator/issues/28)
- [Additional generators](https://github.com/jabbas/generated-secrets-operator/issues/29)
- [The `v1beta1` API](https://github.com/jabbas/generated-secrets-operator/issues/30)

New here? Issues labelled
[`good first issue`](https://github.com/jabbas/generated-secrets-operator/labels/good%20first%20issue)
are a good place to start.

## Documentation

| Document | Content |
|---|---|
| [Installation](docs/installation.md) | Install, upgrade, uninstall, values, scope modes, mirrors, CRD lifecycle, signature verification |
| [API reference](docs/api-reference.md) | Every field, default and validation; the generated Secret; behaviour table |
| [Operations](docs/operations.md) | Rotation, reloading consumers, `deletionPolicy`, troubleshooting, metrics, logging |
| [Security](docs/security.md) | Threat model, RBAC, redaction guarantees, release signing and SBOM |
| [Examples](examples/README.md) | Seven copy-paste `GeneratedSecret`s |
| [Fingerprint format](docs/fingerprints.md) | The frozen format that decides when a value is regenerated |
| [Development](docs/development.md) | Building, testing and running the operator locally |
| [Contributing](CONTRIBUTING.md) | What is welcome, what will not be accepted, how to send a change |
| [Security policy](SECURITY.md) | How to report a vulnerability privately; supported versions |

## Contributing

Bug reports, fixes, features and documentation improvements are welcome. Read
[CONTRIBUTING.md](CONTRIBUTING.md) first; for a new feature, open an issue with
the YAML you would like to write before you write code.

## Licence

Apache-2.0; see [LICENSE](LICENSE) and [NOTICE](NOTICE).

## Acknowledgement

Generated Secrets Operator was inspired by earlier Kubernetes secret-generator projects.
