use criterion::{criterion_group, criterion_main, Criterion};
use tinytemplate::TinyTemplate;

fn process_te_std_format_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_format_big_table(a_table)
}

fn process_te_std_io_write_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_io_write_big_table(a_table)
}

fn process_te_std_fmt_write_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_fmt_write_big_table(a_table)
}

fn process_te_askama_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    cmp_template_engine::do_te_askama_big_table(a_table)
}

fn process_te_sailfish_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    cmp_template_engine::do_te_sailfish_big_table(a_table)
}

fn process_te_sailfish_buf_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    cmp_template_engine::do_te_sailfish_buf_big_table(a_table)
}

fn process_te_tinytempl_big_table<'a>(
    tinytemple: &TinyTemplate<'a>,
    a_table: &[Vec<usize>],
) -> anyhow::Result<String> {
    cmp_template_engine::do_te_tinytempl_big_table(tinytemple, a_table)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (a_teams, res_s) = create_data::create_data_big_table();
    //
    //eprintln!("{}", res_s.len());
    //std::process::exit(1);
    //
    let tinytemple = cmp_template_engine::create_templ_big_table().unwrap();
    //
    match process_te_std_format_big_table(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_std_io_write_big_table(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_std_fmt_write_big_table(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_askama_big_table(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_sailfish_big_table(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_sailfish_buf_big_table(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_tinytempl_big_table(&tinytemple, criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("std_format_bigtable", |b| {
        b.iter(|| {
            let _r = process_te_std_format_big_table(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("std_io_write_bigtable", |b| {
        b.iter(|| {
            let _r = process_te_std_io_write_big_table(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("std_fmt_write_bigtable", |b| {
        b.iter(|| {
            let _r = process_te_std_fmt_write_big_table(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("askama_bigtable", |b| {
        b.iter(|| {
            let _r = process_te_askama_big_table(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("sailfish_bigtable", |b| {
        b.iter(|| {
            let _r = process_te_sailfish_big_table(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("sailfish_buf_bigtable", |b| {
        b.iter(|| {
            let _r = process_te_sailfish_buf_big_table(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("tinytempl_bigtable", |b| {
        b.iter(|| {
            let _r = process_te_tinytempl_big_table(&tinytemple, criterion::black_box(&a_teams));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
