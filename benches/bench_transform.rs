use std::fs::File;
use std::io::Write;

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use csv_to_json::Options;

const BENCHMARK_PATH: &str = "benchmark.csv";

fn generate_data(records: u8) {
    let mut handler = File::create(BENCHMARK_PATH).unwrap();
    let _ = handler.write_all(String::from("col_1,col_2,co_3,col_4\n").as_bytes());

    for _ in 0..records {
        let _ = handler.write_all(String::from("1,2,3,4\n").as_bytes());
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    use csv_to_json::*;
    generate_data(100);


    let _ = c.bench_function("transform", |b| b.iter(|| {
        let option = Options {
            input: String::from(BENCHMARK_PATH),
            output: String::from("output.json"),
            ..Options::default()
        };
        let _ = run(option);
        std::fs::remove_file("output.json")
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);