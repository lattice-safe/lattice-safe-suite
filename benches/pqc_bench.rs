//! Cross-crate PQC benchmark suite.
//!
//! Benchmarks keygen, sign/encaps, verify/decaps for all NIST PQC standards.
//!
//! Run: cargo bench --bench pqc_bench

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// ============================================================
// ML-DSA (Dilithium) — FIPS 204
// ============================================================

fn bench_ml_dsa(c: &mut Criterion) {
    use dilithium::params::{ML_DSA_44, ML_DSA_65, ML_DSA_87};
    use dilithium::safe_api::MlDsaKeyPair;

    let modes = [
        ("ML-DSA-44", ML_DSA_44),
        ("ML-DSA-65", ML_DSA_65),
        ("ML-DSA-87", ML_DSA_87),
    ];

    let mut g = c.benchmark_group("ML-DSA");
    let msg = b"Benchmark message for ML-DSA";

    for (name, mode) in &modes {
        g.bench_with_input(BenchmarkId::new("keygen", name), mode, |b, mode| {
            b.iter(|| MlDsaKeyPair::generate(*mode).unwrap())
        });

        let kp = MlDsaKeyPair::generate(*mode).unwrap();
        g.bench_with_input(BenchmarkId::new("sign", name), mode, |b, _mode| {
            b.iter(|| kp.sign(msg, b"").unwrap())
        });

        let sig = kp.sign(msg, b"").unwrap();
        let pk = kp.public_key().to_vec();
        g.bench_with_input(BenchmarkId::new("verify", name), mode, |b, mode| {
            b.iter(|| MlDsaKeyPair::verify(&pk, &sig, msg, b"", *mode))
        });
    }
    g.finish();
}

// ============================================================
// FN-DSA (Falcon) — FIPS 206
// ============================================================

fn bench_fn_dsa(c: &mut Criterion) {
    use falcon::safe_api::{DomainSeparation, FnDsaKeyPair, FnDsaSignature};

    let mut g = c.benchmark_group("FN-DSA");
    let msg = b"Benchmark message for FN-DSA";
    let domain = DomainSeparation::None;

    for (name, logn) in [("Falcon-512", 9u32), ("Falcon-1024", 10u32)] {
        g.bench_with_input(BenchmarkId::new("keygen", name), &logn, |b, logn| {
            b.iter(|| FnDsaKeyPair::generate(*logn).unwrap())
        });

        let kp = FnDsaKeyPair::generate(logn).unwrap();
        g.bench_with_input(BenchmarkId::new("sign", name), &logn, |b, _logn| {
            b.iter(|| kp.sign(msg, &domain).unwrap())
        });

        let sig = kp.sign(msg, &domain).unwrap();
        let sig_bytes = sig.to_bytes().to_vec();
        let pk = kp.public_key().to_vec();
        g.bench_with_input(BenchmarkId::new("verify", name), &logn, |b, _logn| {
            b.iter(|| FnDsaSignature::verify(&sig_bytes, &pk, msg, &domain))
        });
    }
    g.finish();
}

// ============================================================
// ML-KEM (Kyber) — FIPS 203
// ============================================================

fn bench_ml_kem(c: &mut Criterion) {
    use kyber::kem;
    use kyber::params::KyberMode;

    let modes = [
        ("ML-KEM-512", KyberMode::Kyber512),
        ("ML-KEM-768", KyberMode::Kyber768),
        ("ML-KEM-1024", KyberMode::Kyber1024),
    ];

    let mut g = c.benchmark_group("ML-KEM");

    for (name, mode) in &modes {
        let seed = [0x42u8; 64];
        g.bench_with_input(BenchmarkId::new("keygen", name), mode, |b, mode| {
            b.iter(|| kem::keypair_derand(*mode, &seed))
        });

        let (pk, sk) = kem::keypair_derand(*mode, &seed);
        let coins = [0x42u8; 32];
        g.bench_with_input(BenchmarkId::new("encapsulate", name), mode, |b, mode| {
            b.iter(|| kem::encaps_derand(*mode, &pk, &coins))
        });

        let (ct, _ss) = kem::encaps_derand(*mode, &pk, &coins);
        g.bench_with_input(BenchmarkId::new("decapsulate", name), mode, |b, mode| {
            b.iter(|| kem::decaps(*mode, &ct, &sk))
        });
    }
    g.finish();
}

// ============================================================
// SLH-DSA (SPHINCS+) — FIPS 205
// ============================================================

fn bench_slh_dsa(c: &mut Criterion) {
    use slh_dsa::params::{SLH_DSA_SHA2_128F, SLH_DSA_SHAKE_128F, SLH_DSA_SHAKE_192F};
    use slh_dsa::safe_api::{SlhDsaKeyPair, SlhDsaSignature};

    let modes = [
        ("SHAKE-128f", SLH_DSA_SHAKE_128F),
        ("SHA2-128f", SLH_DSA_SHA2_128F),
        ("SHAKE-192f", SLH_DSA_SHAKE_192F),
    ];

    let mut g = c.benchmark_group("SLH-DSA");
    let msg = b"Benchmark message for SLH-DSA";

    for (name, mode) in &modes {
        g.bench_with_input(BenchmarkId::new("keygen", name), mode, |b, mode| {
            b.iter(|| SlhDsaKeyPair::generate(*mode).unwrap())
        });

        let kp = SlhDsaKeyPair::generate(*mode).unwrap();
        g.bench_with_input(BenchmarkId::new("sign", name), mode, |b, _mode| {
            b.iter(|| kp.sign(msg).unwrap())
        });

        let sig = kp.sign(msg).unwrap();
        let sig_bytes = sig.to_bytes().to_vec();
        let pk = kp.public_key().to_vec();
        g.bench_with_input(BenchmarkId::new("verify", name), mode, |b, mode| {
            b.iter(|| SlhDsaSignature::verify(&sig_bytes, &pk, msg, *mode))
        });
    }
    g.finish();
}

criterion_group!(
    benches,
    bench_ml_dsa,
    bench_fn_dsa,
    bench_ml_kem,
    bench_slh_dsa
);
criterion_main!(benches);
