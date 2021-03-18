use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benches::*;

fn index_bench(c: &mut Criterion) {
    c.bench_function("lint_test_index", |b| {
        b.iter(|| {
            index::foo(
                black_box(&mut vec![false; 42069]),
                black_box(&mut vec![42; 42069]),
            )
        })
    });
}

fn iter_bench(c: &mut Criterion) {
    c.bench_function("lint_test_iter", |b| {
        b.iter(|| {
            iter::foo(
                black_box(&mut vec![false; 42069]),
                black_box(&mut vec![42; 42069]),
            )
        })
    });
}

criterion_group!(benches, index_bench, iter_bench);
criterion_main!(benches);
