# Contributing

Thank you for your interest in Generated Secrets Operator.

## What is welcome

- Bug reports and bug fixes.
- New features (see "How to contribute" below: open an issue first).
- Things to fix: documentation, tests, examples.

## What will not be accepted

The operator has a deliberately narrow scope. These changes will not be accepted, each for the reason given:

- **Annotation-driven generation on plain Secrets.** The only input is a `GeneratedSecret`; the output is a Secret.
- **Deterministic or seed-derived values.** A generated value must be random, so nobody can compute it from anything else.
- **Anything that makes a value change on its own** (operator restart, upgrade, re-apply, cache loss, leader change), or that changes the frozen defaults. A value changes only when the user decides.
- **Broader permissions**, such as `delete` on Secrets or caching all Secrets in the cluster. The operator keeps the smallest permissions that work.
- **Operator logic that knows about Helm, Flux, Argo CD or other deployment tools.** The operator works the same way under any of them.
- **Secret values in logs, Events, status, metrics or error messages.** Values must never leave the Secret.
- **Fetching secrets from external systems.** That is the job of External Secrets Operator.

## How to contribute

- **Bug fixes and small improvements:** open a pull request directly.
- **New features:** open an issue first and include the YAML you would like to write. The API is public and hard to change, so the shape of the feature is agreed before any code is written.

Use the issue forms for bug reports, feature requests and questions. Never paste Secret values into an issue or pull request.

To report a security vulnerability, do not open an issue or pull request. Follow [SECURITY.md](SECURITY.md).

## Contributions assisted by artificial intelligence

Contributions assisted by artificial intelligence (AI) are allowed. The author must understand the change and must have tested it. Pull requests that look generated without understanding are closed without long discussion.

## Development setup

Build, test and check instructions are in [docs/development.md](docs/development.md).

Write commit messages in English.

## Licence

The project is licensed under Apache-2.0. Contributions are accepted under the same licence (inbound = outbound).
