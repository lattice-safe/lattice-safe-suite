# lattice-safe

> **Post-quantum cryptography suite for Rust** — NIST-standardized lattice-based digital signatures.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## What's Included

| Crate | Standard | Algorithm | NIST Levels | Status |
|-------|----------|-----------|-------------|--------|
| [`dilithium-rs`](https://crates.io/crates/dilithium-rs) | FIPS 204 | ML-DSA (Dilithium) | 2, 3, 5 | ✅ v0.2.0 |
| [`falcon-rs`](https://crates.io/crates/falcon-rs) | FIPS 206 | FN-DSA (Falcon) | I, V | ✅ v0.2.2 |

Both implementations are:
- **Pure Rust** — zero C dependencies, zero assembly
- **`no_std` / WASM ready**
- **NIST KAT validated** — bit-for-bit match with C reference
- **Security audited** — zeroization, fuzz-tested, cargo-deny clean
- **SIMD accelerated** — AVX2 (x86_64) + NEON (AArch64) for ML-DSA

## Quick Start

```toml
[dependencies]
lattice-safe = "0.1"
```

### ML-DSA (FIPS 204)

```rust
use lattice_safe::dilithium::{MlDsaKeyPair, ML_DSA_44};

let kp = MlDsaKeyPair::generate(ML_DSA_44).unwrap();
let sig = kp.sign(b"message", b"").unwrap();
assert!(MlDsaKeyPair::verify(kp.public_key(), &sig, b"message", b"", ML_DSA_44));
```

### FN-DSA (FIPS 206)

```rust
use lattice_safe::falcon::prelude::*;

let kp = FnDsaKeyPair::generate(9).unwrap();  // FN-DSA-512
let sig = kp.sign(b"message", &DomainSeparation::None).unwrap();
FnDsaSignature::verify(sig.to_bytes(), kp.public_key(), b"message", &DomainSeparation::None).unwrap();
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `dilithium` | ✅ | ML-DSA (FIPS 204) signatures |
| `falcon` | ✅ | FN-DSA (FIPS 206) signatures |
| `serde` | ❌ | Serialization for keys and signatures |
| `simd` | ❌ | AVX2/NEON NTT acceleration (ML-DSA) |

## Use Only One Scheme

```toml
# ML-DSA only
lattice-safe = { version = "0.1", default-features = false, features = ["dilithium"] }

# FN-DSA only
lattice-safe = { version = "0.1", default-features = false, features = ["falcon"] }
```

## License

MIT
