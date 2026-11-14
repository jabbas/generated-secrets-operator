// SPDX-License-Identifier: Apache-2.0

//! Golden tests for the frozen v1 fingerprint format.
//!
//! The hex literals below were computed independently with
//! `printf '<canonical>' | shasum -a 256`. NEVER update them: a mismatch means
//! the format changed, which would rotate every credential in every cluster.

use std::collections::BTreeMap;

use secretspec_api::resolve::{
    BytesParams, CharsParams, Generator, Require, ResolvedBasicAuth, ResolvedField, ResolvedKind,
    ResolvedSpec, ResolvedSsh, SshAlgorithmParams,
};
use secretspec_api::v1alpha1::{DeletionPolicy, EcdsaCurve, Encoding};
use secretspec_core::{
    Fingerprint, FingerprintMap, FingerprintParseError, ParsedAnnotation, SecretValue, Slot,
    canonical_bytes, canonical_chars, canonical_htpasswd, canonical_ssh, expected_fingerprints,
    fingerprint_of,
};

/// Golden vector 10: `basic-auth/password` with every default.
const BASIC_PASSWORD_DEFAULT: &str =
    "3c72fbc038665c762605f1c73dd470dabd342d0a72ed7feb2a3c3c3c7a94dd4a";

const ALNUM: &str = "303132333435363738394142434445464748494a4b4c4d4e4f505152535455565758595a6162636465666768696a6b6c6d6e6f707172737475767778797a";

fn alnum() -> Vec<u8> {
    let mut v: Vec<u8> = (b'0'..=b'9')
        .chain(b'A'..=b'Z')
        .chain(b'a'..=b'z')
        .collect();
    v.sort_unstable();
    v
}

fn chars(length: u16, alphabet: Vec<u8>) -> CharsParams {
    CharsParams {
        length,
        alphabet,
        require: Require::default(),
        prefix: String::new(),
        suffix: String::new(),
    }
}

fn bytes(count: u16, encoding: Encoding) -> BytesParams {
    BytesParams {
        count,
        encoding,
        prefix: String::new(),
        suffix: String::new(),
    }
}

fn field(name: &str) -> Slot {
    Slot::Field(name.to_owned())
}

fn password_canonical() -> String {
    canonical_chars(&field("password"), &chars(32, alnum()))
}

fn rpc_canonical() -> String {
    canonical_bytes(&field("rpc-secret"), &bytes(32, Encoding::Hex))
}

fn hex_prefix_canonical() -> String {
    let p = CharsParams {
        prefix: "GK".to_owned(),
        ..chars(24, b"0123456789abcdef".to_vec())
    };
    canonical_chars(&field("access-key-id"), &p)
}

fn token_canonical() -> String {
    let p = BytesParams {
        prefix: "tg6_".to_owned(),
        suffix: "-x".to_owned(),
        ..bytes(48, Encoding::Base64url)
    };
    canonical_bytes(&field("token"), &p)
}

fn basic_password_canonical() -> String {
    let mut p = chars(32, alnum());
    p.require = Require {
        lower: 1,
        upper: 2,
        digits: 3,
        symbols: 4,
    };
    canonical_chars(&Slot::BasicAuthPassword, &p)
}

/// The documented example htpasswd line: `htpasswd -nbB -C 12 admin secret`.
const EXAMPLE_HTPASSWD_LINE: &str =
    "admin:$2y$12$0DlAzt5T2HhGhCfza8hh9uTBlWS.K.Fiorr.aH/bnWF6xDZES4D.K";

fn htpasswd_canonical() -> String {
    canonical_htpasswd(
        "admin",
        &SecretValue::new(b"secret".to_vec()),
        &SecretValue::new(EXAMPLE_HTPASSWD_LINE.as_bytes().to_vec()),
        12,
    )
}

fn basic_password_default_canonical() -> String {
    canonical_chars(&Slot::BasicAuthPassword, &chars(32, alnum()))
}

#[test]
fn canonical_chars_default_field() {
    assert_eq!(
        password_canonical(),
        format!(
            "generated-secrets-fingerprint-v1\nslot=field/password\nmode=chars\nlength=32\nalphabet={ALNUM}\nrequire=0,0,0,0\nprefix=\nsuffix=\n"
        )
    );
}

#[test]
fn canonical_bytes_hex_32() {
    assert_eq!(
        rpc_canonical(),
        "generated-secrets-fingerprint-v1\nslot=field/rpc-secret\nmode=bytes\ncount=32\nencoding=hex\nprefix=\nsuffix=\n"
    );
}

#[test]
fn canonical_hex_prefix() {
    assert_eq!(
        hex_prefix_canonical(),
        "generated-secrets-fingerprint-v1\nslot=field/access-key-id\nmode=chars\nlength=24\nalphabet=30313233343536373839616263646566\nrequire=0,0,0,0\nprefix=474b\nsuffix=\n"
    );
}

#[test]
fn canonical_encoding_uses_api_wire_strings() {
    for (encoding, wire) in [
        (Encoding::Hex, "hex"),
        (Encoding::Base64, "base64"),
        (Encoding::Base64url, "base64url"),
        (Encoding::Base32, "base32"),
    ] {
        let line = format!("\nencoding={wire}\n");
        assert!(canonical_bytes(&field("k"), &bytes(1, encoding)).contains(&line));
    }
}

#[test]
fn canonical_ssh_variants() {
    assert_eq!(
        canonical_ssh(&SshAlgorithmParams::Ed25519),
        "generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=ed25519\n"
    );
    assert_eq!(
        canonical_ssh(&SshAlgorithmParams::Rsa { bits: 4096 }),
        "generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=rsa\nbits=4096\n"
    );
    for (curve, wire) in [
        (EcdsaCurve::P256, "P-256"),
        (EcdsaCurve::P384, "P-384"),
        (EcdsaCurve::P521, "P-521"),
    ] {
        assert_eq!(
            canonical_ssh(&SshAlgorithmParams::Ecdsa { curve }),
            format!(
                "generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=ecdsa\ncurve={wire}\n"
            )
        );
    }
}

#[test]
fn fingerprint_header_is_generated_secrets() {
    let canonical = canonical_ssh(&SshAlgorithmParams::Ed25519);
    assert_eq!(
        canonical,
        "generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=ed25519\n"
    );
    assert!(!canonical.contains("secretspec"));
}

#[test]
fn canonical_htpasswd_format() {
    assert_eq!(
        htpasswd_canonical(),
        "generated-secrets-fingerprint-v1\nslot=basic-auth/htpasswd\nusername=61646d696e\npassword-mac=6364d747107a94c67b469b00f1839829f863bddf086718b2c7f883d8a35d2e2d\ncost=12\n"
    );
    // The unsalted SHA-256 of the password must never appear.
    assert!(
        !htpasswd_canonical()
            .contains("2bb80d537b1da3e38bd30361aa855686bde0eacd7162fef6a25fe97bf527a25b")
    );
}

#[test]
fn canonical_basic_auth_password_uses_chars_form() {
    assert_eq!(
        basic_password_canonical(),
        format!(
            "generated-secrets-fingerprint-v1\nslot=basic-auth/password\nmode=chars\nlength=32\nalphabet={ALNUM}\nrequire=1,2,3,4\nprefix=\nsuffix=\n"
        )
    );
}

#[test]
fn golden_fingerprint_values() {
    let cases = [
        (
            "chars default",
            password_canonical(),
            "19ef76bf380a7179247b264a88a2abc1e8e0e9d8672216faffb95206bb96228c",
        ),
        (
            "bytes hex",
            rpc_canonical(),
            "38178d495c71ed16b11f068cb3cd580a0e942c0b395682378308422402eb284a",
        ),
        (
            "hex prefix",
            hex_prefix_canonical(),
            "1f933a465e283c74614ba434794fead46cd2fac6af392ef0245f5c991a5ca1f0",
        ),
        (
            "ssh ed25519",
            canonical_ssh(&SshAlgorithmParams::Ed25519),
            "99b364cb8f12c51d409eee6a5d5ff576a65b17649a8395f78a090f6688b16f1a",
        ),
        (
            "ssh rsa 4096",
            canonical_ssh(&SshAlgorithmParams::Rsa { bits: 4096 }),
            "b62a71589326ab933858ae7a82a250f96aeb2a874db4e63eb00db1ce15d87142",
        ),
        (
            "ssh ecdsa P-384",
            canonical_ssh(&SshAlgorithmParams::Ecdsa {
                curve: EcdsaCurve::P384,
            }),
            "6c48b18050d1d38663ce901f6f8706307dd5a041e11dee521a2e3fef2ad92819",
        ),
        (
            "htpasswd",
            htpasswd_canonical(),
            "430a2529e335c06d4f8aae2429ad1a27bbbf8c3fb57e5eb642f219b51d04556e",
        ),
        (
            "basic-auth password defaults",
            basic_password_default_canonical(),
            BASIC_PASSWORD_DEFAULT,
        ),
        (
            "bytes base64url prefix suffix",
            token_canonical(),
            "09240767b4cebefe03a6d1d93eb315e464a8b4efe7217b84c6f5d82c621324e0",
        ),
        (
            "basic-auth password require",
            basic_password_canonical(),
            "630341f1a39d07c89e9152a5d0f51ca88813ab65531ba73cf6237cabbcfb49f2",
        ),
    ];
    for (name, canonical, expected) in cases {
        assert_eq!(fingerprint_of(&canonical).as_str(), expected, "{name}");
    }
}

fn spec(kind: ResolvedKind) -> ResolvedSpec {
    ResolvedSpec {
        kind,
        data: BTreeMap::new(),
        deletion_policy: DeletionPolicy::Delete,
        rotation: None,
        secret_type: "Opaque".to_owned(),
        labels: BTreeMap::new(),
        annotations: BTreeMap::new(),
    }
}

fn fp_chars(p: &CharsParams) -> Fingerprint {
    fingerprint_of(&canonical_chars(&field("f"), p))
}

fn fp_bytes(p: &BytesParams) -> Fingerprint {
    fingerprint_of(&canonical_bytes(&field("f"), p))
}

fn fp_ssh(a: SshAlgorithmParams) -> Fingerprint {
    fingerprint_of(&canonical_ssh(&a))
}

#[test]
fn fingerprint_independent_of_alphabet_order_and_duplicates() {
    use secretspec_api::resolve::resolve;
    use secretspec_api::v1alpha1::SecretSpecSpec;

    let make = |alphabet: &str| {
        let spec: SecretSpecSpec = serde_json::from_value(serde_json::json!({
            "fields": [{"name": "f", "charset": "custom", "alphabet": alphabet}],
        }))
        .expect("spec json");
        let resolved = resolve(&spec, "n", "ns").expect("valid spec");
        expected_fingerprints(&resolved)
    };
    assert_eq!(make("cba"), make("abcabc"));
}

#[test]
fn fingerprint_ignores_comment_and_key_names() {
    let ssh = |comment: &str, private: &str, public: &str| {
        spec(ResolvedKind::SshKeyPair(ResolvedSsh {
            algorithm: SshAlgorithmParams::Ed25519,
            comment: comment.to_owned(),
            private_key_key: private.to_owned(),
            public_key_key: public.to_owned(),
        }))
    };
    assert_eq!(
        expected_fingerprints(&ssh("a@b", "ssh-privatekey", "ssh-publickey")),
        expected_fingerprints(&ssh("other", "id", "id.pub")),
    );

    let basic = |user_key: &str, pass_key: &str, htpasswd_key: &str| {
        spec(ResolvedKind::BasicAuth(ResolvedBasicAuth {
            username: "admin".to_owned(),
            password: chars(32, alnum()),
            username_key: user_key.to_owned(),
            password_key: pass_key.to_owned(),
            htpasswd_key: htpasswd_key.to_owned(),
        }))
    };
    assert_eq!(
        expected_fingerprints(&basic("username", "password", "auth")),
        expected_fingerprints(&basic("u", "p", "users")),
    );
}

#[test]
fn expected_fingerprints_slots() {
    let fields = spec(ResolvedKind::Fields(vec![
        ResolvedField {
            name: "a".to_owned(),
            generator: Generator::Chars(chars(8, alnum())),
        },
        ResolvedField {
            name: "b".to_owned(),
            generator: Generator::Bytes(bytes(8, Encoding::Hex)),
        },
    ]));
    let slots: Vec<Slot> = expected_fingerprints(&fields).into_keys().collect();
    assert_eq!(slots, vec![field("a"), field("b")]);

    let ssh = spec(ResolvedKind::SshKeyPair(ResolvedSsh {
        algorithm: SshAlgorithmParams::Ed25519,
        comment: String::new(),
        private_key_key: "a".to_owned(),
        public_key_key: "b".to_owned(),
    }));
    let got = expected_fingerprints(&ssh);
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[&Slot::SshKeyPair].as_str(),
        "99b364cb8f12c51d409eee6a5d5ff576a65b17649a8395f78a090f6688b16f1a"
    );

    // htpasswd depends on the generated password, so it is never "expected".
    let basic = spec(ResolvedKind::BasicAuth(ResolvedBasicAuth {
        username: "admin".to_owned(),
        password: chars(32, alnum()),
        username_key: "username".to_owned(),
        password_key: "password".to_owned(),
        htpasswd_key: "auth".to_owned(),
    }));
    let slots: Vec<Slot> = expected_fingerprints(&basic).into_keys().collect();
    assert_eq!(slots, vec![Slot::BasicAuthPassword]);
}

#[test]
fn fingerprint_changes_with_each_parameter() {
    let base = chars(32, alnum());
    let base_fp = fp_chars(&base);
    let require = |lower, upper, digits, symbols| Require {
        lower,
        upper,
        digits,
        symbols,
    };
    let variants = [
        CharsParams {
            length: 33,
            ..base.clone()
        },
        CharsParams {
            alphabet: b"abc".to_vec(),
            ..base.clone()
        },
        CharsParams {
            require: require(1, 0, 0, 0),
            ..base.clone()
        },
        CharsParams {
            require: require(0, 1, 0, 0),
            ..base.clone()
        },
        CharsParams {
            require: require(0, 0, 1, 0),
            ..base.clone()
        },
        CharsParams {
            require: require(0, 0, 0, 1),
            ..base.clone()
        },
        CharsParams {
            prefix: "p".to_owned(),
            ..base.clone()
        },
        CharsParams {
            suffix: "s".to_owned(),
            ..base.clone()
        },
    ];
    for variant in &variants {
        assert_ne!(fp_chars(variant), base_fp, "{variant:?}");
    }

    let base = bytes(32, Encoding::Hex);
    let base_fp = fp_bytes(&base);
    let variants = [
        BytesParams {
            count: 33,
            ..base.clone()
        },
        BytesParams {
            encoding: Encoding::Base64,
            ..base.clone()
        },
        BytesParams {
            prefix: "p".to_owned(),
            ..base.clone()
        },
        BytesParams {
            suffix: "s".to_owned(),
            ..base.clone()
        },
    ];
    for variant in &variants {
        assert_ne!(fp_bytes(variant), base_fp, "{variant:?}");
    }

    let algorithms = [
        SshAlgorithmParams::Ed25519,
        SshAlgorithmParams::Rsa { bits: 2048 },
        SshAlgorithmParams::Rsa { bits: 3072 },
        SshAlgorithmParams::Ecdsa {
            curve: EcdsaCurve::P256,
        },
        SshAlgorithmParams::Ecdsa {
            curve: EcdsaCurve::P384,
        },
    ];
    for (i, a) in algorithms.iter().enumerate() {
        for b in &algorithms[i + 1..] {
            assert_ne!(fp_ssh(*a), fp_ssh(*b), "{a:?} vs {b:?}");
        }
    }
}

#[test]
fn fingerprint_changes_with_slot_and_with_htpasswd_inputs() {
    let p = chars(32, alnum());
    assert_ne!(
        fingerprint_of(&canonical_chars(&field("a"), &p)),
        fingerprint_of(&canonical_chars(&field("b"), &p)),
    );
    let h = |user: &str, pw: &[u8], line: &str, cost: u32| {
        fingerprint_of(&canonical_htpasswd(
            user,
            &SecretValue::new(pw.to_vec()),
            &SecretValue::new(line.as_bytes().to_vec()),
            cost,
        ))
    };
    let line = EXAMPLE_HTPASSWD_LINE;
    let base = h("admin", b"secret", line, 12);
    assert_ne!(base, h("root", b"secret", line, 12));
    assert_ne!(base, h("admin", b"other", line, 12));
    assert_ne!(base, h("admin", b"secret", line, 10));
    // A new bcrypt salt (a new line) changes the fingerprint too.
    assert_ne!(base, h("admin", b"secret", &line.replace("0Dl", "1Dl"), 12));
}

const HEX_A: &str = "19ef76bf380a7179247b264a88a2abc1e8e0e9d8672216faffb95206bb96228c";
const HEX_B: &str = "38178d495c71ed16b11f068cb3cd580a0e942c0b395682378308422402eb284a";

#[test]
fn fingerprint_map_annotation_is_sorted_compact_json() {
    let mut map = FingerprintMap::default();
    map.slots
        .insert(field("b"), Fingerprint::parse(HEX_B).expect("hex"));
    map.slots
        .insert(field("a"), Fingerprint::parse(HEX_A).expect("hex"));
    let annotation = map.to_annotation();
    assert_eq!(
        annotation,
        format!(r#"{{"field/a":"{HEX_A}","field/b":"{HEX_B}"}}"#)
    );
    assert_eq!(
        FingerprintMap::from_annotation(&annotation).expect("round trip"),
        ParsedAnnotation {
            map,
            problems: Vec::new()
        }
    );
}

#[test]
fn fingerprint_map_annotation_keys_sort_by_string() {
    let mut map = FingerprintMap::default();
    for slot in [
        Slot::SshKeyPair,
        Slot::BasicAuthPassword,
        Slot::BasicAuthHtpasswd,
        field("z"),
    ] {
        map.slots
            .insert(slot, Fingerprint::parse(HEX_A).expect("hex"));
    }
    let keys: Vec<String> = map
        .to_annotation()
        .split('"')
        .skip(1)
        .step_by(4)
        .map(str::to_owned)
        .collect();
    assert_eq!(
        keys,
        [
            "basic-auth/htpasswd",
            "basic-auth/password",
            "field/z",
            "ssh/keypair"
        ]
    );
    assert_eq!(FingerprintMap::default().to_annotation(), "{}");
}

#[test]
fn fingerprint_map_annotation_rejects_only_non_object_input() {
    for annotation in ["not json", "[]", "42", r#""str""#, "null", "", "{"] {
        assert!(
            matches!(
                FingerprintMap::from_annotation(annotation),
                Err(FingerprintParseError::Json(_))
            ),
            "{annotation:?}"
        );
    }
    let empty = FingerprintMap::from_annotation("{}").expect("empty object");
    assert_eq!(empty, ParsedAnnotation::default());
}

#[test]
fn fingerprint_map_annotation_keeps_unknown_slots() {
    let annotation = format!(
        r#"{{"field/a":"{HEX_A}","future/slot":"v2:anything","field/bad name":"x","FIELD/a":"{HEX_B}","field/..":"{HEX_B}"}}"#
    );
    let parsed = FingerprintMap::from_annotation(&annotation).expect("object");
    assert!(parsed.problems.is_empty(), "{:?}", parsed.problems);
    assert_eq!(
        parsed.map.slots.keys().cloned().collect::<Vec<_>>(),
        vec![field("a")]
    );
    let unknown: Vec<(&str, &str)> = parsed
        .map
        .unknown
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    assert_eq!(
        unknown,
        vec![
            ("FIELD/a", HEX_B),
            ("field/..", HEX_B),
            ("field/bad name", "x"),
            ("future/slot", "v2:anything"),
        ]
    );
    // A writer carries unknown entries through, merged in sorted order.
    assert_eq!(
        parsed.map.to_annotation(),
        format!(
            r#"{{"FIELD/a":"{HEX_B}","field/..":"{HEX_B}","field/a":"{HEX_A}","field/bad name":"x","future/slot":"v2:anything"}}"#
        )
    );
}

#[test]
fn fingerprint_map_annotation_invalid_value_invalidates_only_its_slot() {
    let bad_values = [
        format!(r#""{}""#, HEX_A.to_uppercase()),
        format!(r#""{}""#, &HEX_A[..63]),
        format!(r#""{HEX_A}0""#),
        r#""gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg""#.to_owned(),
        r#""""#.to_owned(),
        "42".to_owned(),
        "null".to_owned(),
        format!(r#"["{HEX_A}"]"#),
    ];
    for bad in bad_values {
        let annotation = format!(r#"{{"field/a":{bad},"field/b":"{HEX_B}","ssh/keypair":{bad}}}"#);
        let parsed = FingerprintMap::from_annotation(&annotation)
            .unwrap_or_else(|e| panic!("{annotation}: {e}"));
        assert_eq!(
            parsed.map.slots.keys().cloned().collect::<Vec<_>>(),
            vec![field("b")],
            "{annotation}"
        );
        assert_eq!(parsed.map.slots[&field("b")].as_str(), HEX_B);
        assert!(parsed.map.unknown.is_empty(), "{annotation}");
        assert_eq!(parsed.problems.len(), 2, "{annotation}");
        assert!(
            parsed
                .problems
                .iter()
                .all(|p| matches!(p, FingerprintParseError::InvalidFingerprint(_))),
            "{:?}",
            parsed.problems
        );
    }
}

#[test]
fn fingerprint_map_annotation_drops_non_string_unknown_entries() {
    let annotation = format!(r#"{{"future/slot":{{"nested":1}},"field/a":"{HEX_A}"}}"#);
    let parsed = FingerprintMap::from_annotation(&annotation).expect("object");
    assert_eq!(parsed.map.slots.len(), 1);
    assert!(parsed.map.unknown.is_empty());
    assert_eq!(
        parsed.problems,
        vec![FingerprintParseError::InvalidUnknownEntry(
            "future/slot".to_owned()
        )]
    );
}

#[test]
fn slot_display_and_from_str() {
    for (slot, text) in [
        (field("password"), "field/password"),
        (Slot::SshKeyPair, "ssh/keypair"),
        (Slot::BasicAuthPassword, "basic-auth/password"),
        (Slot::BasicAuthHtpasswd, "basic-auth/htpasswd"),
    ] {
        assert_eq!(slot.to_string(), text);
        assert_eq!(text.parse::<Slot>().expect("parse"), slot);
    }
    for bad in [
        "",
        "field",
        "field/",
        "ssh",
        "ssh/keypair/",
        "field/a b",
        "FIELD/a",
    ] {
        assert!(bad.parse::<Slot>().is_err(), "{bad:?}");
    }
}

#[test]
fn secret_value_redacts() {
    let v = SecretValue::new(b"hunter2-very-secret".to_vec());
    let shown = format!("{v:?} {v}");
    assert!(shown.contains("<redacted>"));
    assert!(!shown.contains("hunter2"));
    assert_eq!(v.expose(), b"hunter2-very-secret");
}

#[test]
fn slot_name_validation_matches_api_key_name_rule() {
    use secretspec_api::resolve::resolve;
    use secretspec_api::v1alpha1::SecretSpecSpec;

    let long = "a".repeat(253);
    let too_long = "a".repeat(254);
    for name in [
        "a", "A-b_c.d", ".", "..", "..x", ".x", "x..", "a b", "a/b", "ä", "", &long, &too_long,
    ] {
        let spec: SecretSpecSpec = serde_json::from_value(serde_json::json!({
            "fields": [{"name": name}],
        }))
        .expect("spec json");
        let api_accepts = resolve(&spec, "n", "ns").is_ok();
        let slot_accepts = format!("field/{name}").parse::<Slot>().is_ok();
        assert_eq!(api_accepts, slot_accepts, "name {name:?}");
    }
}
