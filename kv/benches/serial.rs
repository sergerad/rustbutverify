use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use kv::DurableKv;

fn serial(c: &mut Criterion) {
    let mut group = c.benchmark_group("serial");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("put", |bench| {
        let file_path = std::path::Path::new("./kv.db");
        let kv: DurableKv<i32, i32> = DurableKv::new(file_path).unwrap();
        bench.iter(move || kv.put(1, 1));
    });

    group.bench_function("get", |bench| {
        let file_path = std::path::Path::new("./kv.db");
        let kv: DurableKv<i32, i32> = DurableKv::new(file_path).unwrap();
        kv.put(1, 1);
        bench.iter(move || kv.get(1));
    });

    group.bench_function("commit", |bench| {
        let file_path = std::path::Path::new("./kv.db");
        bench.iter(move || {
            let kv: DurableKv<i32, i32> = DurableKv::new(file_path).unwrap();
            (0..1000).for_each(|i| {
                kv.put(i, i);
            })
        });
    });

    group.finish();
}

criterion_group!(serial_group, serial);
criterion_main!(serial_group);
