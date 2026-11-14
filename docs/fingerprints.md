# Fingerprint format (v1)

This document is **normative**. It defines how the operator decides that a
generated value is still valid or must be regenerated.

> **The format is frozen forever.** Changing any byte of the canonical text
> below — a field name, a separator, the case of a hex digit, the order of
> lines, a trailing newline — changes the fingerprint of every existing
> Secret. The operator would then see every stored fingerprint as stale and
> **rotate every credential in every cluster** on upgrade. A new format needs
> a new version header (`generated-secrets-fingerprint-v2`), an explicit migration
> that recognises both, and must never silently replace v1.
>
> The format is frozen from 0.2.0; the 0.1.x format (header
> `secretspec-fingerprint-v1`) belonged to the project's former name and is
> never read.

## Concepts

- A **slot** is a stable logical name for one generated value. Slots do not
  depend on Secret key names, so renaming `keys.*` moves a value to a new key
  without regenerating it.
- The **fingerprint** of a slot is the SHA-256 (lowercase hex, 64 characters)
  of the slot's *canonical text*, a function of the slot's **fully resolved**
  generation parameters (every default applied; see the stability rule below).
- The operator stores all fingerprints of a Secret as a JSON object in the
  annotation `secretspec.jabbas.eu/fingerprints`. A value is kept while its
  stored fingerprint equals the expected one and regenerated otherwise.

## Slots

| Slot | Meaning |
|---|---|
| `field/<name>` | The `fields[]` entry called `<name>`. `<name>` is a valid Secret key name: `[-._a-zA-Z0-9]+`, 1-253 characters, not `.`, not starting with `..`. |
| `ssh/keypair` | The SSH key pair (private and public key together). |
| `basic-auth/password` | The `basicAuth` password. |
| `basic-auth/htpasswd` | The `basicAuth` htpasswd line. |

`basic-auth/htpasswd` is value-dependent (it covers the generated password and
the stored htpasswd line), so it cannot be computed from the spec alone; the
other slots can.

## Canonical text

Rules for every form:

- Lines are separated by a single `\n` (LF). **The last line also ends with
  `\n`.** There are no carriage returns, no spaces around `=`, no blank lines.
- Hexadecimal is **lowercase**, two digits per byte.
- An empty value is written as `key=` (nothing after the `=`).
- Integers are written in decimal without padding or sign.
- The line order is exactly as listed.
- The input of SHA-256 is the UTF-8 (in practice ASCII) bytes of the text.

### Characters mode (`field/<name>` and `basic-auth/password`)

```
generated-secrets-fingerprint-v1
slot=<slot>
mode=chars
length=<n>
alphabet=<hex of sorted unique alphabet bytes>
require=<lower>,<upper>,<digits>,<symbols>
prefix=<hex of prefix bytes>
suffix=<hex of suffix bytes>
```

- `<slot>` is `field/<name>` or `basic-auth/password`.
- `length` is the resolved number of random characters (without prefix/suffix).
- `alphabet` is the **effective** alphabet: the charset (or the custom
  alphabet), minus `exclude`, with duplicates removed and the bytes sorted
  ascending. The order or duplicates the user wrote therefore never matter.
- `require` is the resolved minimum count per class, `0` for an unset class.

### Random-bytes mode (`field/<name>`)

```
generated-secrets-fingerprint-v1
slot=field/<name>
mode=bytes
count=<n>
encoding=<hex|base64|base64url|base32>
prefix=<hex of prefix bytes>
suffix=<hex of suffix bytes>
```

`encoding` is the API wire string (`base64url`, never `Base64url`).

### SSH key pair (`ssh/keypair`)

```
generated-secrets-fingerprint-v1
slot=ssh/keypair
algorithm=ed25519
```

```
generated-secrets-fingerprint-v1
slot=ssh/keypair
algorithm=rsa
bits=<2048|3072|4096>
```

```
generated-secrets-fingerprint-v1
slot=ssh/keypair
algorithm=ecdsa
curve=<P-256|P-384|P-521>
```

The curve is the API wire string (`P-384`).

### htpasswd (`basic-auth/htpasswd`)

```
generated-secrets-fingerprint-v1
slot=basic-auth/htpasswd
username=<hex of username bytes>
password-mac=<hex of HMAC-SHA256(key = htpasswd line bytes, message = password bytes)>
cost=<n>
```

- The HMAC key is the stored htpasswd line `<username>:<bcrypt hash>` exactly
  as it is in the Secret data (no trailing newline); the message is the
  password bytes. HMAC-SHA256 is RFC 2104 with SHA-256; the MAC is written as
  64 lowercase hex characters.
- `cost` is the bcrypt cost (currently `12`).

The line is regenerated only when the username, the password or the cost
changes; bcrypt is salted, so it must not be recomputed on every reconcile.
Because the key is the line itself, a regenerated line (new salt) also yields
a new fingerprint.

**Why a MAC and not a hash of the password.** The fingerprints annotation is
visible to everyone who can read the Secret's metadata, which includes people
who cannot read its data. A plain `sha256(password)` there would be an
unsalted password hash that anyone could test guesses against offline. Keyed
with the salt-bearing htpasswd line, which lives only in the Secret data, the
value cannot be used to test password guesses without access to the data
itself.

## Not fingerprinted

These inputs do **not** enter any canonical text. Changing them never
regenerates a value:

- the SSH `comment` (changing it rewrites only the public-key line);
- every `keys.*` name (renaming a key moves the value);
- the `data` literals;
- `deletionPolicy`;
- everything under `secret.*` (type, labels, annotations);
- `rotation` (it triggers regeneration on its own schedule, not through the
  fingerprint).

## Annotation format

`secretspec.jabbas.eu/fingerprints` is a compact JSON object (no whitespace)
from slot string to fingerprint, **keys sorted by their string form** (byte
order):

```json
{"field/a":"<64 hex>","field/b":"<64 hex>"}
```

An empty map is `{}`.

### Reading the annotation (normative)

A reader MUST be tolerant, so that a rollback or a downgrade, or one damaged
entry, never makes the operator treat every slot as stale (which would rotate
every value of the Secret):

1. **Whole-annotation error only for non-objects.** If the text is not JSON,
   or is JSON but not an object, the reader reports an error for the whole
   annotation. This is the only case in which no entry is usable.
2. **Unknown keys are ignored but preserved.** A key that is not a known slot
   form (see the table above; this includes `field/<name>` where `<name>` is
   not a valid Secret key name) is not interpreted. If its value is a string,
   the reader keeps the entry verbatim and a writer MUST re-emit it unchanged,
   merged with its own slots in the sorted order above. (A known slot wins over
   a preserved entry with the same key.) An unknown key whose value is not a
   string cannot be preserved; it is dropped and reported.
3. **An invalid value invalidates only its own slot.** If a known slot's value
   is not a string of exactly 64 lowercase hex characters, that slot counts as
   having no stored fingerprint (so, as for lost metadata in design §7.2 row
   16, a present value is kept and its fingerprint re-stamped with a Warning
   `FingerprintsReset`, and a missing value is generated) and the problem is
   reported. All other entries are used normally.

The reference implementation is `FingerprintMap::from_annotation`, which
returns the usable map plus the list of problems.

## Stability rule

Fingerprints are always computed from **fully resolved** parameters, never from
the raw spec. Every default that feeds a canonical text (`length` 32,
`charset` alphanumeric, empty `prefix`/`suffix`, `require` all zero, `rsaBits`
default, `ecdsaCurve` default, ...) is therefore part of the API-version
contract: **defaults and this format MUST NOT change within an apiVersion.**
The golden vectors below (and `crates/secretspec-core/tests/fingerprint_golden.rs`)
guard this. A mismatch is a release blocker; the expected values are never
updated to make a test pass.

## Golden test vectors

Anyone can verify an implementation with `printf '<text>' | shasum -a 256`
(`printf`, not `echo`; `\n` is a line feed). `ALNUM` below stands for the hex of
the alphanumeric alphabet:

```
ALNUM = 303132333435363738394142434445464748494a4b4c4d4e4f505152535455565758595a6162636465666768696a6b6c6d6e6f707172737475767778797a
```

(`ALNUM` is the hex of `0-9A-Za-z` in byte order. In the canonical text it is
written out in full.)

| # | Case | Canonical text (`\n` = LF) | SHA-256 |
|---|---|---|---|
| 1 | `field/password`, 32 alphanumeric characters | `generated-secrets-fingerprint-v1\nslot=field/password\nmode=chars\nlength=32\nalphabet=ALNUM\nrequire=0,0,0,0\nprefix=\nsuffix=\n` | `19ef76bf380a7179247b264a88a2abc1e8e0e9d8672216faffb95206bb96228c` |
| 2 | `field/rpc-secret`, 32 bytes hex | `generated-secrets-fingerprint-v1\nslot=field/rpc-secret\nmode=bytes\ncount=32\nencoding=hex\nprefix=\nsuffix=\n` | `38178d495c71ed16b11f068cb3cd580a0e942c0b395682378308422402eb284a` |
| 3 | `field/access-key-id`, 24 hex characters, prefix `GK` | `generated-secrets-fingerprint-v1\nslot=field/access-key-id\nmode=chars\nlength=24\nalphabet=30313233343536373839616263646566\nrequire=0,0,0,0\nprefix=474b\nsuffix=\n` | `1f933a465e283c74614ba434794fead46cd2fac6af392ef0245f5c991a5ca1f0` |
| 4 | `field/token`, 48 bytes base64url, prefix `tg6_`, suffix `-x` | `generated-secrets-fingerprint-v1\nslot=field/token\nmode=bytes\ncount=48\nencoding=base64url\nprefix=7467365f\nsuffix=2d78\n` | `09240767b4cebefe03a6d1d93eb315e464a8b4efe7217b84c6f5d82c621324e0` |
| 5 | `basic-auth/password`, 32 alphanumeric, require 1,2,3,4 | `generated-secrets-fingerprint-v1\nslot=basic-auth/password\nmode=chars\nlength=32\nalphabet=ALNUM\nrequire=1,2,3,4\nprefix=\nsuffix=\n` | `630341f1a39d07c89e9152a5d0f51ca88813ab65531ba73cf6237cabbcfb49f2` |
| 6 | `ssh/keypair`, ed25519 | `generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=ed25519\n` | `99b364cb8f12c51d409eee6a5d5ff576a65b17649a8395f78a090f6688b16f1a` |
| 7 | `ssh/keypair`, RSA 4096 | `generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=rsa\nbits=4096\n` | `b62a71589326ab933858ae7a82a250f96aeb2a874db4e63eb00db1ce15d87142` |
| 8 | `ssh/keypair`, ECDSA P-384 | `generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=ecdsa\ncurve=P-384\n` | `6c48b18050d1d38663ce901f6f8706307dd5a041e11dee521a2e3fef2ad92819` |
| 9 | `basic-auth/htpasswd`, username `admin`, password `secret`, htpasswd line `LINE` (below), cost 12 | `generated-secrets-fingerprint-v1\nslot=basic-auth/htpasswd\nusername=61646d696e\npassword-mac=6364d747107a94c67b469b00f1839829f863bddf086718b2c7f883d8a35d2e2d\ncost=12\n` | `430a2529e335c06d4f8aae2429ad1a27bbbf8c3fb57e5eb642f219b51d04556e` |
| 10 | `basic-auth/password`, every default (32 alphanumeric, require 0,0,0,0, no prefix/suffix) | `generated-secrets-fingerprint-v1\nslot=basic-auth/password\nmode=chars\nlength=32\nalphabet=ALNUM\nrequire=0,0,0,0\nprefix=\nsuffix=\n` | `3c72fbc038665c762605f1c73dd470dabd342d0a72ed7feb2a3c3c3c7a94dd4a` |

In vector 9, `LINE` is the example htpasswd line produced by
`htpasswd -nbB -C 12 admin secret` (a real bcrypt hash of `secret`):

```
admin:$2y$12$0DlAzt5T2HhGhCfza8hh9uTBlWS.K.Fiorr.aH/bnWF6xDZES4D.K
```

and the `password-mac` value is

```sh
LINE='admin:$2y$12$0DlAzt5T2HhGhCfza8hh9uTBlWS.K.Fiorr.aH/bnWF6xDZES4D.K'
printf '%s' secret | openssl dgst -sha256 -mac HMAC -macopt key:"$LINE"
# SHA2-256(stdin)= 6364d747107a94c67b469b00f1839829f863bddf086718b2c7f883d8a35d2e2d
python3 -c 'import hmac,hashlib,sys; print(hmac.new(sys.argv[1].encode(), b"secret", hashlib.sha256).hexdigest())' "$LINE"
# 6364d747107a94c67b469b00f1839829f863bddf086718b2c7f883d8a35d2e2d
```

Vectors 1, 6, 7 and 10 are also what a minimal manifest resolves to
(`fields: [{name: password}]`, `sshKeyPair: {}`, `sshKeyPair: {algorithm: rsa}`,
`basicAuth: {username: admin}`); `crates/secretspec-core/tests/resolve_golden.rs`
pins that end to end, so a changed default fails as loudly as a changed format.

Reproduce vector 6:

```sh
printf 'generated-secrets-fingerprint-v1\nslot=ssh/keypair\nalgorithm=ed25519\n' | shasum -a 256
# 99b364cb8f12c51d409eee6a5d5ff576a65b17649a8395f78a090f6688b16f1a  -
```

In vectors 1, 5 and 10 replace `ALNUM` with the full hex string above before
hashing.
