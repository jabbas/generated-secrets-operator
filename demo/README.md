# Demo recording

The demo shown in the project README (a GIF) and on the project website (an
asciicast) is recorded by script, so every release can be re-recorded with one
command.

## Prerequisites

On `PATH`:

- [`kind`](https://kind.sigs.k8s.io/), with a container provider. The provider
  comes from your environment; this repository's recordings use podman
  (`KIND_EXPERIMENTAL_PROVIDER=podman`). The script does not set it.
- `helm` 4 and `kubectl` 1.31 or newer (the demo uses `kubectl wait --for=create`)
- [`asciinema`](https://docs.asciinema.org/) 3.x
- [`agg`](https://github.com/asciinema/agg) (asciinema GIF generator)

Network access to `ghcr.io` is needed to pull the published chart and image.

## Record

From the repository root:

```sh
bash demo/record.sh <chart-version>      # e.g. bash demo/record.sh 0.2.0
```

The script:

1. refuses to run if a kind cluster named `generated-secrets-demo` already
   exists (it never deletes a cluster it did not create);
2. creates that cluster with its own kubeconfig, so your kubeconfig and current
   context stay untouched;
3. installs the released chart from `oci://ghcr.io/jabbas/charts/generated-secrets-operator`
   into `generated-secrets-system`, labelled for Pod Security `restricted`;
4. records `demo/demo.sh` with `asciinema rec` (110×34 terminal, idle time
   limited to 2 seconds);
5. renders the GIF with `agg`;
6. deletes the cluster, whatever the outcome.

`demo/demo.sh` is what is recorded. It also runs on its own against any cluster
that has the operator installed with the release name `generated-secrets-operator`
in the namespace `generated-secrets-system` (the Deployment is named after the
release, so another release name does not work). It creates the namespace `demo`
and refuses to run if that namespace already exists, or if it cannot tell
(for example an unreachable cluster). Every wait
for a state change is a `kubectl wait` with a 60-second limit, so a demo that
does not behave as shown fails instead of being recorded.

The scenes: (1) delete a `GeneratedSecret` with `deletionPolicy: Retain`, the
password survives; (2) restart the operator, the password is the same;
(3) delete an SSH key Secret, a new key pair appears; (4) change a token's
length, a new value appears and the Events show `Regenerated`.

## Outputs

- `docs/assets/demo.gif` is used in the project README.
- `docs/assets/demo.cast` is used by the project website.

Values shown in the recording are generated inside the throwaway cluster and
are not secrets.

## Upload to asciinema.org (optional)

```sh
asciinema upload docs/assets/demo.cast
```

The owner does this, or an agent does it with the owner's consent: the upload
publishes the recording under the owner's asciinema.org account.
