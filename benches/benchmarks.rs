use criterion::*;

macro_rules! benchmark {
    ($benchmark:ident; $($ecs:ident),+) => {
        fn $benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($benchmark));
            $(
                group.bench_function(stringify!($ecs), |b| {
                    let mut bench = ecs_bench_suite::$ecs::$benchmark::Benchmark::new();
                    b.iter(move || bench.run());
                });
            )+
        }
    };
}

benchmark!(add_remove;             bevy, edict, hecs, legion,                planck_ecs, shipyard, shipyard_git, sparsey,                 specs);
benchmark!(frag_iter;              bevy, edict, hecs, legion,                planck_ecs, shipyard, shipyard_git, sparsey,                 specs);
benchmark!(heavy_compute;          bevy,        hecs, legion, legion_packed,             shipyard, shipyard_git,                          specs);
benchmark!(parallel_light_compute; bevy,        hecs, legion, legion_packed,             shipyard, shipyard_git,                          specs);
benchmark!(schedule;               bevy,              legion, legion_packed, planck_ecs, shipyard, shipyard_git, sparsey, sparsey_packed, specs);
benchmark!(serialize_binary;                    hecs, legion);
benchmark!(serialize_text;                      hecs, legion);
benchmark!(simple_insert;          bevy, edict, hecs, legion,                planck_ecs, shipyard, shipyard_git, sparsey,                 specs);
benchmark!(simple_iter;            bevy, edict, hecs, legion, legion_packed,             shipyard, shipyard_git, sparsey, sparsey_packed, specs);

criterion_group!(
    benchmarks,
    add_remove,
    frag_iter,
    heavy_compute,
    parallel_light_compute,
    schedule,
    serialize_binary,
    serialize_text,
    simple_insert,
    simple_iter,
);
criterion_main!(benchmarks);
