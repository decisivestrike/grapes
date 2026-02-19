use criterion::{Criterion, criterion_group, criterion_main};
use grapes::{derived, state};
use gtk::glib::clone;
use std::{hint::black_box, time::Duration};

fn reactivity_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reactivity");
    group.warm_up_time(Duration::from_millis(1));

    group.bench_function("double", |b| {
        let counter = state(0);
        let doubled = derived(clone!(
            #[strong]
            counter,
            move || counter.get() * 2
        ));

        b.iter(|| {
            black_box(counter.get());
            black_box(doubled.get());

            counter.update(black_box(|v: &mut i32| *v += 1));
        });
    });

    group.bench_function("sum", |b| {
        let first = state(0);
        let second = state(0);
        let sum = derived(clone!(
            #[strong]
            first,
            #[strong]
            second,
            move || first.get() + second.get()
        ));

        b.iter(|| {
            black_box(first.get());
            black_box(second.get());
            black_box(sum.get());

            first.update(black_box(|v: &mut i32| *v += 1));
            second.update(black_box(|v: &mut i32| *v += 1));
        });
    });

    group.bench_function("100x effect", |b| {
        let counter = state(0);
        let doubles: Vec<_> = (0..100)
            .map(|_| {
                derived(clone!(
                    #[strong]
                    counter,
                    move || counter.get() * 2
                ))
            })
            .collect();

        b.iter(|| {
            black_box(counter.get());
            black_box(doubles[0].get());

            counter.update(black_box(|v: &mut i32| *v += 1));
        });
    });

    group.bench_function("1000x effect", |b| {
        let counter = state(0);
        let doubles: Vec<_> = (0..1000)
            .map(|_| {
                derived(clone!(
                    #[strong]
                    counter,
                    move || counter.get() * 2
                ))
            })
            .collect();

        b.iter(|| {
            black_box(counter.get());
            black_box(doubles[0].get());

            counter.update(black_box(|v: &mut i32| *v += 1));
        });
    });

    group.finish();
}

criterion_group!(benches, reactivity_benches);
criterion_main!(benches);
