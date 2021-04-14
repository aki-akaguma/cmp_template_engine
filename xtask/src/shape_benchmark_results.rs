use anyhow::Context;
use std::io::BufRead;

pub fn run(_program: &str, _args: &[&str]) -> anyhow::Result<()> {
    let bench_vec_11 = get_bench("z.bench.1.log")?;
    let bench_vec_21 = get_bench("z.bench.2.log")?;
    //
    let mut bench_vec_12 = get_bench("z.musl.bench.1.log")?;
    let mut bench_vec_22 = get_bench("z.musl.bench.2.log")?;
    bench_vec_12.sort_by(|a, b| a.name.cmp(&b.name));
    bench_vec_22.sort_by(|a, b| a.name.cmp(&b.name));
    //
    let mut bench_vec_13 = get_bench("z.arm64.musl.bench.1.log")?;
    let mut bench_vec_23 = get_bench("z.arm64.musl.bench.2.log")?;
    bench_vec_13.sort_by(|a, b| a.name.cmp(&b.name));
    bench_vec_23.sort_by(|a, b| a.name.cmp(&b.name));
    //
    if bench_vec_13.is_empty() {
        output2(bench_vec_11, bench_vec_12)?;
        println!("");
        output2(bench_vec_21, bench_vec_22)?;
    } else {
        output3(bench_vec_11, bench_vec_12, bench_vec_13)?;
        println!("");
        output3(bench_vec_21, bench_vec_22, bench_vec_23)?;
    }
    //
    Ok(())
}

fn output2(bench_vec_1: Vec<BenchStr>, bench_vec_2: Vec<BenchStr>) -> anyhow::Result<()> {
    println!(
        "| {:^23} | {:^11} | {:^11} |",
        "`name`", "`bench`", "`:musl`"
    );
    println!(
        "|:{:<23}-|-{:>11}:|-{:>11}:|",
        "-".repeat(23),
        "-".repeat(11),
        "-".repeat(11),
    );
    for bench1 in bench_vec_1 {
        let idx2 = bench_vec_2
            .binary_search_by(|item| item.name.cmp(&bench1.name))
            .unwrap();
        let bench2 = &bench_vec_2[idx2];
        if bench1.is_cycle {
            println!(
                "| {:<23} | {:>8.3} kc | {:>8.3} kc |",
                bench1.name,
                bench1.time / 1000.0,
                bench2.time / 1000.0,
            );
        } else {
            println!(
                "| {:<23} | {:>8.3} uc | {:>8.3} uc |",
                bench1.name,
                bench1.time / 0.000001,
                bench2.time / 0.000001,
            );
        }
    }
    //
    Ok(())
}

fn output3(
    bench_vec_1: Vec<BenchStr>,
    bench_vec_2: Vec<BenchStr>,
    bench_vec_3: Vec<BenchStr>,
) -> anyhow::Result<()> {
    println!(
        "| {:^23} | {:^11} | {:^11} | {:^11} |",
        "`name`", "`bench`", "`86_64:musl`", "`arm64:musl`"
    );
    println!(
        "|:{:<23}-|-{:>11}:|-{:>11}:|-{:>11}:|",
        "-".repeat(23),
        "-".repeat(11),
        "-".repeat(11),
        "-".repeat(11),
    );
    for bench1 in bench_vec_1 {
        let idx2 = bench_vec_2
            .binary_search_by(|item| item.name.cmp(&bench1.name))
            .unwrap();
        let bench2 = &bench_vec_2[idx2];
        let idx3 = bench_vec_3
            .binary_search_by(|item| item.name.cmp(&bench1.name))
            .unwrap();
        let bench3 = &bench_vec_3[idx3];
        if bench1.is_cycle {
            println!(
                "| {:<23} | {:>8.3} kc | {:>8.3} kc | {:>8.3} kc |",
                bench1.name,
                bench1.time / 1000.0,
                bench2.time / 1000.0,
                bench3.time / 1000.0,
            );
        } else {
            println!(
                "| {:<23} | {:>8.3} uc | {:>8.3} uc | {:>8.3} uc |",
                bench1.name,
                bench1.time / 0.000001,
                bench2.time / 0.000001,
                bench3.time / 0.000001,
            );
        }
    }
    //
    Ok(())
}

fn _output(bench_vec: Vec<BenchStr>) -> anyhow::Result<()> {
    println!("| {:^22} | {:^11} |", "`name`", "`bench`");
    println!("|:{:<22}-|-{:>11}:|", "-".repeat(22), "-".repeat(11),);
    for bench in bench_vec {
        if bench.is_cycle {
            println!("| {:<22} | {:>8.3} kc |", bench.name, bench.time / 1000.0,);
        } else {
            println!("| {:<22} | {:>8.3} uc |", bench.name, bench.time / 0.000001,);
        }
    }
    //
    Ok(())
}

#[rustfmt::skip]
#[derive(Default)]
pub struct BenchStr {
    pub name: String,   // name
    pub time: f64,      // seconds
    pub is_cycle: bool, // cycles
    pub time_1k: f64,   // seconds per 1k
    pub size: u64,      // bytes
    pub oh_time: f64,   // seconds
    pub oh_size: u64,   // bytes
}

fn _set_size(bench_vec: &mut Vec<BenchStr>, in_file: &str) -> anyhow::Result<()> {
    let mut base_time = 0f64;
    let mut base_size = 0u64;
    let re_1 = regex::Regex::new(r"^ *(\d+)\t.*\t([^ ]+)$").unwrap();
    let reader = std::io::BufReader::new(
        std::fs::File::open(in_file)
            .with_context(|| format!("could not open file `{}`", in_file))?,
    );
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re_1.captures(&line) {
            //  934281	  26312	    736	 961329	  eab31	cmp_structopt-curl
            let size_s = &caps[1];
            let name_s = &caps[2];
            let name = if name_s.ends_with("-curl") {
                &name_s[0..(name_s.len() - 5)]
            } else {
                name_s
            };
            let i = match bench_vec.iter().position(|x| x.name == name) {
                Some(i) => i,
                None => {
                    let msg = format!("can not find size: {}", name);
                    return Err(anyhow::Error::msg(msg));
                }
            };
            bench_vec[i].size = size_s.parse::<u64>()?;
            if name == "cmp_null_void" {
                base_time = bench_vec[i].time;
                base_size = bench_vec[i].size;
            }
        }
    }
    //
    for bench in bench_vec {
        bench.oh_time = bench.time - base_time;
        bench.oh_size = bench.size - base_size;
    }
    //
    Ok(())
}

fn get_bench(in_file: &str) -> anyhow::Result<Vec<BenchStr>> {
    let mut vec_benchstr: Vec<BenchStr> = Vec::new();
    if !std::path::Path::new(in_file).exists() {
        return Ok(vec_benchstr);
    }
    //
    let re_1 =
        regex::Regex::new(r"^([^ ]+) +time: +[\[][^ ]+ [^ ]+ ([^ ]+) ([^ ]+) [^ ]+ [^ ]+[\]]$")
            .unwrap();
    //
    let reader = std::io::BufReader::new(
        std::fs::File::open(in_file)
            .with_context(|| format!("could not open file `{}`", in_file))?,
    );
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re_1.captures(&line) {
            // cmp_structopt::curl::   time:   [302.50 us 302.87 us 303.34 us]
            // cmp_structopt::curl::   time:   [714991.6559 cycles 715483.2743 cycles 716029.3928 cycles]
            let nm = normalize_name(&caps[1])?;
            let tm = normalize_time(&caps[2], &caps[3])?;
            let is_cycle = if &caps[3] == "cycles" { true } else { false };
            let time_1k = if nm.ends_with("^01k") {
                tm
            } else if nm.ends_with("^08k") {
                tm / 8.0
            } else if nm.ends_with("^90k") {
                tm / 90.0
            } else {
                0.0
            };
            //
            vec_benchstr.push(BenchStr {
                name: nm,
                time: tm,
                is_cycle: is_cycle,
                time_1k: time_1k,
                ..BenchStr::default()
            });
        }
    }
    vec_benchstr.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    //vec_benchstr.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    //
    Ok(vec_benchstr)
}

fn normalize_name(name_s: &str) -> anyhow::Result<String> {
    Ok(name_s.to_string())
}

fn normalize_time(num_s: &str, unit_s: &str) -> anyhow::Result<f64> {
    let num: f64 = num_s.parse::<f64>()?;
    let unit: f64 = match unit_s {
        "ms" => 0.001,
        "us" => 0.000001,
        "ns" => 0.000000001,
        "ps" => 0.000000000001,
        "cycles" => 1.0,
        _ => {
            let msg = format!("can not convert unit: {}", unit_s);
            return Err(anyhow::Error::msg(msg));
        }
    };
    Ok(num * unit)
}
