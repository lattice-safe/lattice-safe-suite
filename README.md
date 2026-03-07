# lattice-safe-suite

> **Post-quantum cryptography suite for Rust** — all four NIST PQC standards in one crate.

[![crates.io](https://img.shields.io/crates/v/lattice-safe-suite.svg)](https://crates.io/crates/lattice-safe-suite)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## What's Included

| Crate | Standard | Algorithm | Type | NIST Levels | Version |
|-------|----------|-----------|------|-------------|---------|
| [`dilithium-rs`](https://crates.io/crates/dilithium-rs) | FIPS 204 | ML-DSA (Dilithium) | Signature | 2, 3, 5 | v0.2.0 |
| [`falcon-rs`](https://crates.io/crates/falcon-rs) | FIPS 206 | FN-DSA (Falcon) | Signature | I, V | v0.2.2 |
| [`lattice-kyber`](https://crates.io/crates/lattice-kyber) | FIPS 203 | ML-KEM (Kyber) | KEM | 1, 3, 5 | v0.1.2 |
| [`lattice-slh-dsa`](https://crates.io/crates/lattice-slh-dsa) | FIPS 205 | SLH-DSA (SPHINCS+) | Signature | 1, 3, 5 | v0.3.3 |

All implementations are:
- **Pure Rust** — zero C dependencies, `#![forbid(unsafe_code)]`
- **`no_std` / WASM ready**
- **NIST KAT validated** — bit-for-bit match with C reference
- **Security hardened** — zeroization, typed safe APIs, fuzz-tested
- **SIMD accelerated** — AVX2 (x86_64) + NEON (AArch64) for lattice schemes
- **1.4–1.9× faster** than C reference implementations

## Quick Start

```toml
[dependencies]
lattice-safe-suite = "0.4"
```

### ML-DSA (FIPS 204) — Digital Signatures

```rust
use lattice_safe_suite::dilithium::{MlDsaKeyPair, ML_DSA_44};

let kp = MlDsaKeyPair::generate(ML_DSA_44).unwrap();
let sig = kp.sign(b"message", b"").unwrap();
assert!(MlDsaKeyPair::verify(kp.public_key(), &sig, b"message", b"", ML_DSA_44));
```

### FN-DSA (FIPS 206) — Digital Signatures

```rust
use lattice_safe_suite::falcon::prelude::*;

let kp = FnDsaKeyPair::generate(9).unwrap();  // FN-DSA-512
let sig = kp.sign(b"message", &DomainSeparation::None).unwrap();
FnDsaSignature::verify(sig.to_bytes(), kp.public_key(), b"message", &DomainSeparation::None).unwrap();
```

### ML-KEM (FIPS 203) — Key Encapsulation

```rust
use lattice_safe_suite::kyber::{MlKemKeyPair, ML_KEM_768};
use lattice_safe_suite::kyber::safe_api::encaps_derand;

let coins = [0u8; 64]; // use MlKemKeyPair::generate() with getrandom in production!
let kp = MlKemKeyPair::generate_derand(ML_KEM_768, &coins);

let enc_coins = [1u8; 32];
let (ct, ss_sender) = encaps_derand(ML_KEM_768, kp.public_key(), &enc_coins).unwrap();
let ss_receiver = kp.decaps(&ct).unwrap();
assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());
```

### SLH-DSA (FIPS 205) — Hash-Based Signatures

```rust
use lattice_safe_suite::slh_dsa::{SlhDsaKeyPair, SlhDsaSignature};
use lattice_safe_suite::slh_dsa::params::SLH_DSA_SHAKE_128F;

let kp = SlhDsaKeyPair::generate(SLH_DSA_SHAKE_128F).unwrap();
let sig = kp.sign(b"message").unwrap();
assert!(SlhDsaSignature::verify(sig.to_bytes(), kp.public_key(), b"message", SLH_DSA_SHAKE_128F));
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `dilithium` | ✅ | ML-DSA (FIPS 204) signatures |
| `falcon` | ✅ | FN-DSA (FIPS 206) signatures |
| `kyber` | ✅ | ML-KEM (FIPS 203) key encapsulation |
| `slh-dsa` | ✅ | SLH-DSA (FIPS 205) hash-based signatures |
| `serde` | ❌ | Serialization for keys and signatures |
| `simd` | ❌ | AVX2/NEON NTT acceleration (ML-DSA + ML-KEM) |
| `getrandom` | ❌ | OS entropy for ML-KEM randomized API |

## Use Only One Scheme

```toml
# ML-DSA only
lattice-safe-suite = { version = "0.4", default-features = false, features = ["dilithium"] }

# SLH-DSA only
lattice-safe-suite = { version = "0.4", default-features = false, features = ["slh-dsa"] }
```

## License

MIT
