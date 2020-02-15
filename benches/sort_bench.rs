#[macro_use]
extern crate criterion;

use simd_sort::quick_sort;
use criterion::Criterion;


fn rust_sort_benchmark(c: &mut Criterion) {
    c.bench_function("Rust sort", |b| b.iter(|| {
        let mut data = vec![5, 2, 1, 4, 3];
        data.sort_unstable();
    }));
}
fn rust_unstable_sort_benchmark(c: &mut Criterion) {
    c.bench_function("Rust unstable_sort", |b| b.iter(|| {
        let mut data = vec![5, 2, 1, 4, 3];
        data.sort_unstable();
    }));
}


fn quicksort_benchmark(c: &mut Criterion) {
    c.bench_function("quicksort", |b| b.iter(|| {
        let mut data = vec![5, 2, 1, 4, 3];
        quick_sort(&mut data);
    }));
}

criterion_group!(benches,
                 rust_sort_benchmark, rust_unstable_sort_benchmark,
                 quicksort_benchmark);
criterion_main!(benches);