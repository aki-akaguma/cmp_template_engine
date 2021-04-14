use cmp_template_engine::Teams;
use criterion::{criterion_group, criterion_main, Criterion};
use tinytemplate::TinyTemplate;

fn process_te_std_format_teams(a_teams: &Teams) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_format_teams(a_teams)
}

fn process_te_std_io_write_teams(a_teams: &Teams) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_io_write_teams(a_teams)
}

fn process_te_std_fmt_write_teams(a_teams: &Teams) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_fmt_write_teams(a_teams)
}

fn process_te_askama_teams(a_teams: &Teams) -> anyhow::Result<String> {
    cmp_template_engine::do_te_askama_teams(a_teams)
}

fn process_te_sailfish_teams(a_teams: &Teams) -> anyhow::Result<String> {
    cmp_template_engine::do_te_sailfish_teams(a_teams)
}

fn process_te_sailfish_buf_teams(a_teams: &Teams) -> anyhow::Result<String> {
    cmp_template_engine::do_te_sailfish_buf_teams(a_teams)
}

fn process_te_tinytempl_teams<'a>(
    tinytemple: &TinyTemplate<'a>,
    a_teams: &Teams,
) -> anyhow::Result<String> {
    cmp_template_engine::do_te_tinytempl_teams(tinytemple, a_teams)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (a_teams, res_s) = create_data::create_data_teams();
    //
    //eprintln!("{}", res_s.len());
    //std::process::exit(1);
    //
    let tinytemple = cmp_template_engine::create_templ_teams().unwrap();
    //
    match process_te_std_format_teams(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_std_io_write_teams(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_std_fmt_write_teams(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_askama_teams(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_sailfish_teams(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_sailfish_buf_teams(criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_tinytempl_teams(&tinytemple, criterion::black_box(&a_teams)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("std_format_teams", |b| {
        b.iter(|| {
            let _r = process_te_std_format_teams(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("std_io_write_teams", |b| {
        b.iter(|| {
            let _r = process_te_std_io_write_teams(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("std_fmt_write_teams", |b| {
        b.iter(|| {
            let _r = process_te_std_fmt_write_teams(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("askama_teams", |b| {
        b.iter(|| {
            let _r = process_te_askama_teams(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("sailfish_teams", |b| {
        b.iter(|| {
            let _r = process_te_sailfish_teams(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("sailfish_buf_teams", |b| {
        b.iter(|| {
            let _r = process_te_sailfish_buf_teams(criterion::black_box(&a_teams));
        })
    });
    c.bench_function("tinytempl_teams", |b| {
        b.iter(|| {
            let _r = process_te_tinytempl_teams(&tinytemple, criterion::black_box(&a_teams));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
