//! # lattice-safe
//!
//! Post-quantum cryptography suite for Rust — digital signatures and key
//! encapsulation based on NIST-standardized lattice schemes.
//!
//! This crate re-exports the following implementations:
//!
//! | Crate | Standard | Algorithm | NIST Levels |
//! |-------|----------|-----------|-------------|
//! | [`dilithium`] | FIPS 204 | ML-DSA (Dilithium) | 2, 3, 5 |
//! | [`falcon`] | FIPS 206 | FN-DSA (Falcon) | I, V |
//! | [`kyber`] | FIPS 203 | ML-KEM (Kyber) | 1, 3, 5 |
//!
//! ## Quick Start
//!
//! ```rust
//! use lattice_safe::dilithium::{MlDsaKeyPair, ML_DSA_44};
//!
//! let kp = MlDsaKeyPair::generate(ML_DSA_44).unwrap();
//! let sig = kp.sign(b"Hello, post-quantum world!", b"").unwrap();
//! assert!(MlDsaKeyPair::verify(
//!     kp.public_key(), &sig, b"Hello, post-quantum world!", b"",
//!     ML_DSA_44
//! ));
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `dilithium` | ✅ | ML-DSA (FIPS 204) signatures |
//! | `falcon` | ✅ | FN-DSA (FIPS 206) signatures |
//! | `kyber` | ✅ | ML-KEM (FIPS 203) key encapsulation |
//! | `serde` | ❌ | Serialization for keys and signatures |
//! | `simd` | ❌ | AVX2/NEON NTT acceleration (ML-DSA only) |

#![cfg_attr(not(any(feature = "dilithium", feature = "falcon", feature = "kyber")), no_std)]

/// ML-DSA (FIPS 204) / CRYSTALS-Dilithium digital signatures.
///
/// Re-exported from the [`dilithium-rs`](https://crates.io/crates/dilithium-rs) crate.
#[cfg(feature = "dilithium")]
pub use dilithium;

/// FN-DSA (FIPS 206) / Falcon digital signatures.
///
/// Re-exported from the [`falcon-rs`](https://crates.io/crates/falcon-rs) crate.
#[cfg(feature = "falcon")]
pub use falcon;

/// ML-KEM (FIPS 203) / CRYSTALS-Kyber key encapsulation.
///
/// Re-exported from the [`lattice-kyber`](https://crates.io/crates/lattice-kyber) crate.
#[cfg(feature = "kyber")]
pub use kyber;
