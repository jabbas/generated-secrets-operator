// SPDX-License-Identifier: Apache-2.0

//! End-to-end golden tests: a minimal manifest goes through `resolve()` (which
//! applies the frozen defaults) and `expected_fingerprints()`, and must land on
//! the pinned golden vectors of `docs/fingerprints.md`.
//!
//! This catches a changed *default* as well as a changed canonical format:
//! either would rotate every credential created from such a manifest. The hex
//! literals were computed independently with `printf '<canonical>' | shasum -a
//! 256`. NEVER update them to make a test pass.
// Tests report through panics.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use generated_secrets_api::resolve;
use generated_secrets_api::v1alpha1::GeneratedSecretSpec;
use generated_secrets_core::{Slot, expected_fingerprints};

fn fingerprints_of(yaml: &str) -> Vec<(String, String)> {
    let spec: GeneratedSecretSpec = serde_norway::from_str(yaml).expect("manifest deserializes");
    let resolved = resolve(&spec, "name", "namespace").unwrap_or_else(|e| panic!("{yaml}: {e}"));
    expected_fingerprints(&resolved)
        .into_iter()
        .map(|(slot, fingerprint)| (slot.to_string(), fingerprint.as_str().to_owned()))
        .collect()
}

fn pinned(slot: &Slot, hex: &str) -> Vec<(String, String)> {
    vec![(slot.to_string(), hex.to_owned())]
}

#[test]
fn default_field_matches_golden_vector_1() {
    assert_eq!(
        fingerprints_of("fields: [{name: password}]"),
        pinned(
            &Slot::Field("password".to_owned()),
            "19ef76bf380a7179247b264a88a2abc1e8e0e9d8672216faffb95206bb96228c"
        )
    );
}

#[test]
fn default_ssh_key_pair_matches_golden_vector_6() {
    assert_eq!(
        fingerprints_of("sshKeyPair: {}"),
        pinned(
            &Slot::SshKeyPair,
            "99b364cb8f12c51d409eee6a5d5ff576a65b17649a8395f78a090f6688b16f1a"
        )
    );
}

#[test]
fn default_rsa_key_pair_matches_golden_vector_7() {
    assert_eq!(
        fingerprints_of("sshKeyPair: {algorithm: rsa}"),
        pinned(
            &Slot::SshKeyPair,
            "b62a71589326ab933858ae7a82a250f96aeb2a874db4e63eb00db1ce15d87142"
        )
    );
}

#[test]
fn default_basic_auth_password_matches_golden_vector_10() {
    assert_eq!(
        fingerprints_of("basicAuth: {username: admin}"),
        pinned(
            &Slot::BasicAuthPassword,
            "3c72fbc038665c762605f1c73dd470dabd342d0a72ed7feb2a3c3c3c7a94dd4a"
        )
    );
}
