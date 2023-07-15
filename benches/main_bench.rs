use criterion::{criterion_group, criterion_main, Criterion};

use random_generator::{Xorshift128, Xorshift32, Xorshift64, XorshiftSize};
// use random_generator::{RandomGeneratorWithSeed, RandomGeneratable};

fn xorshift32(c: &mut Criterion) {
    let mut random = Xorshift32::new();
    c.bench_function("32", |b| b.iter(|| random.gen()));
}

fn xorshift64(c: &mut Criterion) {
    let mut random = Xorshift64::new();
    c.bench_function("64", |b| b.iter(|| random.gen()));
}

fn xorshift128(c: &mut Criterion) {
    let mut random = Xorshift128::new();
    c.bench_function("128", |b| b.iter(|| random.gen()));
}

fn xorshiftsize(c: &mut Criterion) {
    let mut random = XorshiftSize::new();
    c.bench_function("usize", |b| b.iter(|| random.gen()));
}

fn random(c: &mut Criterion) {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};

    let mut rng = SmallRng::seed_from_u64(42);

    c.bench_function("random", |b| b.iter(|| rng.gen::<u64>()));
}

criterion_group!(
    benches,
    xorshift32,
    xorshift64,
    xorshift128,
    xorshiftsize,
    random
);
criterion_main!(benches);
