use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use sorting_algorithms::{bubble_sort_v1, bubble_sort_v2, bubble_sort_v3, selection_sort};
fn sorting_benchmarks(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    c.bench_function("bubble_sort v1", |b| {
        b.iter(|| {
            let mut array: Vec<i16> = (0..100).map(|_| rng.gen()).collect();
            bubble_sort_v1(black_box(&mut array));
        })
    });

    c.bench_function("bubble_sort v2", |b| {
        b.iter(|| {
            let mut array: Vec<i16> = (0..100).map(|_| rng.gen()).collect();
            bubble_sort_v2(black_box(&mut array));
        })
    });

    c.bench_function("bubble_sort v3", |b| {
        b.iter(|| {
            let mut array: Vec<i16> = (0..100).map(|_| rng.gen()).collect();
            bubble_sort_v3(black_box(&mut array));
        })
    });

    c.bench_function("selection_sort v1", |b| {
        b.iter(|| {
            let mut array: Vec<i16> = (0..100).map(|_| rng.gen()).collect();
            selection_sort(black_box(&mut array));
        })
    });
}

criterion_group!(benches, sorting_benchmarks);
criterion_main!(benches);
