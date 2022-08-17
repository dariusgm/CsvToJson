use std::fs;
use std::fs::File;
use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv_to_json::ApplicationOptions;

const BENCHMARK_PATH: &str = "benchmark.csv";

fn generate_data(records: u16) {
    let mut handler = File::create(BENCHMARK_PATH).unwrap();
    let _ = handler.write_all(String::from("col_1,col_2,co_3,col_4\n").as_bytes());

    for _ in 0..records {
        let _ = handler.write_all(String::from("1,2,3,4\n").as_bytes());
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    generate_data(10000);

    let _ = c.bench_function("transform", |b| {
        b.iter(|| {
            let options = ApplicationOptions { input: vec![BENCHMARK_PATH.to_owned()], output: None};
            let _ = csv_to_json::run_by_option(&options).unwrap();
            std::fs::remove_file("output.json")
        })
    });

    fs::remove_file("benchmark.csv").unwrap();
    fs::remove_file("benchmark.csv.json").unwrap();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
