


// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use fluid::{Config, Fluid};



// fn benchmark_update_fluid(c: &mut Criterion) {
//     c.bench_function("update_fluid", |b| {
//         let config = Config::new();
//         let mut fluid = Fluid::construct(&config);

//         b.iter(|| {
//             fluid.update_fluid(black_box(true), black_box(true), black_box(true), black_box(true));
//         });
//     });
// }

// criterion_group!(benches, benchmark_update_fluid);
// criterion_main!(benches);
