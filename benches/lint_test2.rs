use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benches::*;

fn index_bench(c: &mut Criterion) {
    c.bench_function("lint_test2_index", |b| {
        b.iter(|| {
            index2::foo(
                black_box(&mut vec![42; 42069]),
                black_box(&vec![42; 42069]),
                black_box(42),
            )
        })
    });
}

fn iter_bench(c: &mut Criterion) {
    c.bench_function("lint_test2_iter", |b| {
        b.iter(|| {
            iter2::foo(
                black_box(&mut vec![42; 42069]),
                black_box(&vec![42; 42069]),
                black_box(42),
            )
        })
    });
}

fn range_bench(c: &mut Criterion) {
    c.bench_function("lint_test2_range", |b| {
        b.iter(|| {
            range::foo(
                black_box(&mut vec![42; 42069]),
                black_box(&vec![42; 42069]),
                black_box(42),
            )
        })
    });
}

criterion_group!(benches, index_bench, iter_bench, range_bench);
criterion_main!(benches);
