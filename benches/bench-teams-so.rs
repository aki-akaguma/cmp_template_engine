use cmp_template_engine::TeamsSo;
use criterion::{criterion_group, criterion_main, Criterion};
use tinytemplate::TinyTemplate;

fn process_te_std_format_teams_so(a_teams_so: &TeamsSo) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_format_teams_so(a_teams_so)
}

fn process_te_std_io_write_teams_so(a_teams_so: &TeamsSo) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_io_write_teams_so(a_teams_so)
}

fn process_te_std_fmt_write_teams_so(a_teams_so: &TeamsSo) -> anyhow::Result<String> {
    cmp_template_engine::do_te_std_fmt_write_teams_so(a_teams_so)
}

fn process_te_askama_teams_so(a_teams_so: &TeamsSo) -> anyhow::Result<String> {
    cmp_template_engine::do_te_askama_teams_so(a_teams_so)
}

fn process_te_sailfish_teams_so(a_teams_so: &TeamsSo) -> anyhow::Result<String> {
    cmp_template_engine::do_te_sailfish_teams_so(a_teams_so)
}

fn process_te_sailfish_buf_teams_so(a_teams_so: &TeamsSo) -> anyhow::Result<String> {
    cmp_template_engine::do_te_sailfish_buf_teams_so(a_teams_so)
}

fn process_te_sailfish_buf_fmt_teams_so(a_teams_so: &TeamsSo) -> anyhow::Result<String> {
    cmp_template_engine::do_te_sailfish_buf_fmt_teams_so(a_teams_so)
}

fn process_te_tinytempl_teams_so<'a>(
    tinytemple: &TinyTemplate<'a>,
    a_teams_so: &TeamsSo,
) -> anyhow::Result<String> {
    cmp_template_engine::do_te_tinytempl_teams_so(tinytemple, a_teams_so)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (a_teams_so, res_s) = create_data::create_data_teams_so();
    //
    //eprintln!("{}", res_s.len());
    //std::process::exit(1);
    //
    let tinytemple = cmp_template_engine::create_templ_teams_so().unwrap();
    //
    match process_te_std_format_teams_so(criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_std_io_write_teams_so(criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_std_fmt_write_teams_so(criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_askama_teams_so(criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_sailfish_teams_so(criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_sailfish_buf_teams_so(criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_sailfish_buf_fmt_teams_so(criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_te_tinytempl_teams_so(&tinytemple, criterion::black_box(&a_teams_so)) {
        Ok(s) => {
            assert_eq!(s, res_s);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("std_format_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_std_format_teams_so(criterion::black_box(&a_teams_so));
        })
    });
    c.bench_function("std_io_write_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_std_io_write_teams_so(criterion::black_box(&a_teams_so));
        })
    });
    c.bench_function("std_fmt_write_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_std_fmt_write_teams_so(criterion::black_box(&a_teams_so));
        })
    });
    c.bench_function("askama_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_askama_teams_so(criterion::black_box(&a_teams_so));
        })
    });
    c.bench_function("sailf_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_sailfish_teams_so(criterion::black_box(&a_teams_so));
        })
    });
    c.bench_function("sailf_buf_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_sailfish_buf_teams_so(criterion::black_box(&a_teams_so));
        })
    });
    c.bench_function("sailf_buf_fmt_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_sailfish_buf_fmt_teams_so(criterion::black_box(&a_teams_so));
        })
    });
    c.bench_function("tinytempl_teams_so", |b| {
        b.iter(|| {
            let _r = process_te_tinytempl_teams_so(&tinytemple, criterion::black_box(&a_teams_so));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(1500));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
