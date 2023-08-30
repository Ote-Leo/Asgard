use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fuzzy_text::levenshtein_distance;

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let bench_data: Vec<&str> = black_box(include_str!("samples.txt").lines().collect());
    let expected_result: Vec<usize> = black_box(
        include_str!("expected.txt")
            .split(",")
            .map(|l| l.parse::<usize>().unwrap())
            .collect(),
    );

    criterion.bench_function("levenshtein_distance", |bench| {
        bench.iter(|| {
            let mut last_value = black_box("");
            for (line, &res) in bench_data.iter().zip(expected_result.iter()) {
                let dist = levenshtein_distance(black_box(last_value), black_box(line));
                last_value = line;
                assert_eq!(res, dist);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
