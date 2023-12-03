use criterion::{criterion_group, criterion_main, Criterion};

use template_benchmarks_rs::{
    askama, fomat, handlebars, horrorshow, liquid, markup, maud, minijinja, ramhorns, ructe,
    sailfish, std_write, tera,
};

fn big_table(c: &mut Criterion) {
    let input = 100;
    let mut group = c.benchmark_group("Big table");

    group.bench_with_input("write", &input, std_write::big_table);
    group.bench_with_input("Askama", &input, askama::big_table);
    group.bench_with_input("Horrorshow", &input, horrorshow::big_table);
    group.bench_with_input("Ructe", &input, ructe::big_table);
    group.bench_with_input("fomat", &input, fomat::big_table);
    group.bench_with_input("Markup", &input, markup::big_table);
    group.bench_with_input("Maud", &input, maud::big_table);
    group.bench_with_input("Sailfish", &input, sailfish::big_table);

    group.bench_with_input("Handlebars", &input, handlebars::big_table);
    group.bench_with_input("Tera", &input, tera::big_table);
    group.bench_with_input("Liquid", &input, liquid::big_table);
    group.bench_with_input("MiniJinja", &input, minijinja::big_table);
    group.bench_with_input("Ramhorns", &input, ramhorns::big_table);

    group.finish();
}

fn teams(c: &mut Criterion) {
    let input = 0;
    let mut group = c.benchmark_group("Teams");

    group.bench_with_input("write", &input, std_write::teams);
    group.bench_with_input("Askama", &input, askama::teams);
    group.bench_with_input("Horrorshow", &input, horrorshow::teams);
    group.bench_with_input("Ructe", &input, ructe::teams);
    group.bench_with_input("fomat", &input, fomat::teams);
    group.bench_with_input("Markup", &input, markup::teams);
    group.bench_with_input("Maud", &input, maud::teams);
    group.bench_with_input("Sailfish", &input, sailfish::teams);

    group.bench_with_input("Handlebars", &input, handlebars::teams);
    group.bench_with_input("Tera", &input, tera::teams);
    group.bench_with_input("Liquid", &input, liquid::teams);
    group.bench_with_input("MiniJinja", &input, minijinja::teams);
    group.bench_with_input("Ramhorns", &input, ramhorns::teams);

    group.finish();
}

criterion_group!(benches, big_table, teams);
criterion_main!(benches);
