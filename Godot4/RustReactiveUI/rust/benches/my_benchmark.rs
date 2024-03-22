use std::hint::black_box;
use std::time::Instant;

use criterion::{criterion_group, criterion_main, Criterion};

use gdext_example::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create dynamic", |b| {
        b.iter_custom(|iter| {
            let now = Instant::now();
            for _ in 0..iter {
                black_box(Dynamic::new(black_box(42)));
            }
            now.elapsed()
        })
    });

    c.bench_function("poll (no change)", |b| {
        b.iter_custom(|iter| {
            let dynamic = Dynamic::new(42);
            let consumer = dynamic.into_consumer();

            // Consume once outside timing loop
            consumer.on_change(|_x| {});

            let now = Instant::now();
            for _ in 0..iter {
                let mut outer = None;
                consumer.on_change(|x| {
                    outer = Some(*x);
                });
                assert!(black_box(outer).is_none());
            }
            now.elapsed()
        })
    });

    c.bench_function("update", |b| {
        b.iter_custom(|iter| {
            let dynamic = Dynamic::new(42);

            let now = Instant::now();
            for _ in 0..iter {
                black_box(dynamic.update(|x| x + 1));
            }
            now.elapsed()
        })
    });

    c.bench_function("update + poll", |b| {
        b.iter_custom(|iter| {
            let dynamic = Dynamic::new(42);
            let consumer = dynamic.into_consumer();

            let now = Instant::now();
            for _ in 0..iter {
                black_box(dynamic.update(|x| x + 1));

                let mut outer = None;
                consumer.on_change(|x| {
                    outer = Some(*x);
                });

                assert!(black_box(outer).is_some());
            }
            now.elapsed()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
