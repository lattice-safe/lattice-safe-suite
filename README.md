# lattice-safe-suite

> **Post-quantum cryptography suite for Rust** — all four NIST PQC standards in one crate.

[![crates.io](https://img.shields.io/crates/v/lattice-safe-suite.svg)](https://crates.io/crates/lattice-safe-suite)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## What's Included

| Crate | Standard | Algorithm | Type | NIST Levels | Status |
|-------|----------|-----------|------|-------------|--------|
| [`dilithium-rs`](https://crates.io/crates/dilithium-rs) | FIPS 204 | ML-DSA (Dilithium) | Signature | 2, 3, 5 | ✅ v0.2.0 |
| [`falcon-rs`](https://crates.io/crates/falcon-rs) | FIPS 206 | FN-DSA (Falcon) | Signature | I, V | ✅ v0.2.2 |
| [`lattice-kyber`](https://crates.io/crates/lattice-kyber) | FIPS 203 | ML-KEM (Kyber) | KEM | 1, 3, 5 | ✅ v0.1.1 |
| [`lattice-slh-dsa`](https://crates.io/crates/lattice-slh-dsa) | FIPS 205 | SLH-DSA (SPHINCS+) | Signature | 1, 3, 5 | ✅ v0.1.1 |

All implementations are:
- **Pure Rust** — zero C dependencies, zero assembly
- **`no_std` / WASM ready**
- **NIST KAT validated** — bit-for-bit match with C reference
- **Security audited** — zeroization, fuzz-tested
- **SIMD accelerated** — AVX2 (x86_64) + NEON (AArch64)
- **1.4–1.8× faster than C reference** implementations

## Quick Start

```toml
[dependencies]
lattice-safe-suite = "0.3"
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
use lattice_safe_suite::kyber::{kem, params::ML_KEM_768};

let coins = [0u8; 64]; // use real entropy!
let (pk, sk) = kem::keypair_derand(ML_KEM_768, &coins);

let enc_coins = [1u8; 32];
let (ct, ss_sender) = kem::encaps_derand(ML_KEM_768, &pk, &enc_coins);
let ss_receiver = kem::decaps(ML_KEM_768, &ct, &sk);
assert_eq!(ss_sender, ss_receiver);
```

### SLH-DSA (FIPS 205) — Hash-Based Signatures

```rust
use lattice_safe_suite::slh_dsa::params::SLH_DSA_SHAKE_128F;
use lattice_safe_suite::slh_dsa::sign::{keygen_seed, sign, verify};

let mode = SLH_DSA_SHAKE_128F;
let seed = vec![42u8; mode.seed_bytes()];
let (pk, sk) = keygen_seed(mode, &seed);
let sig = sign(&sk, b"message", mode);
assert!(verify(&pk, &sig, b"message", mode));
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `dilithium` | ✅ | ML-DSA (FIPS 204) signatures |
| `falcon` | ✅ | FN-DSA (FIPS 206) signatures |
| `kyber` | ✅ | ML-KEM (FIPS 203) key encapsulation |
| `slh-dsa` | ✅ | SLH-DSA (FIPS 205) hash-based signatures |
| `serde` | ❌ | Serialization for keys and signatures |
| `simd` | ❌ | AVX2/NEON NTT acceleration (ML-DSA) |

## Use Only One Scheme

```toml
# ML-DSA only
lattice-safe-suite = { version = "0.3", default-features = false, features = ["dilithium"] }

# SLH-DSA only
lattice-safe-suite = { version = "0.3", default-features = false, features = ["slh-dsa"] }
```

## License

MIT
