use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benches::*;

fn mulsi3_bench(c: &mut Criterion) {
    c.bench_function("mulsi3_paolo", |b| {
        b.iter(|| mulsi3::mulsi3(black_box(u32::MAX), black_box(u32::MAX)))
    });
}
fn mulsi3_iter_bench(c: &mut Criterion) {
    c.bench_function("mulsi3_iter", |b| {
        b.iter(|| mulsi3_iter::mulsi3(black_box(u32::MAX), black_box(u32::MAX)))
    });
}
fn mulsi3_old_bench(c: &mut Criterion) {
    c.bench_function("mulsi3_old", |b| {
        b.iter(|| mulsi3_old::mulsi3(black_box(u32::MAX), black_box(u32::MAX)))
    });
}

criterion_group!(benches, mulsi3_bench, mulsi3_iter_bench, mulsi3_old_bench);
criterion_main!(benches);
