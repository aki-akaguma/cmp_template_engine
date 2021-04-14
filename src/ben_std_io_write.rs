use std::io::Write;

#[allow(unused_imports)]
use super::{Team, Teams};

pub fn do_te_std_io_write_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    let mut output = Vec::with_capacity(13000);
    write!(
        &mut output,
        "# The Big Table

"
    )?;
    for row in a_table {
        for col in row {
            write!(&mut output, "| {num} ", num = col)?;
        }
        write!(
            &mut output,
            "|
"
        )?;
    }
    //let s = String::from_utf8(output)?;
    let s = unsafe { String::from_utf8_unchecked(output) };
    //
    Ok(s)
}

pub fn do_te_std_io_write_teams(a_teams: &Teams) -> anyhow::Result<String> {
    let mut output = Vec::with_capacity(300);
    write!(
        &mut output,
        "# CSL {year}
=================

|num | name             | score             |
|---:|:-----------------|------------------:|
",
        year = a_teams.year
    )?;
    for (i, team) in a_teams.teams.iter().enumerate() {
        write!(
            &mut output,
            "| {num} | {name} | {score} |
",
            num = i + 1,
            name = team.name,
            score = team.score
        )?;
    }
    //let s = String::from_utf8(output)?;
    let s = unsafe { String::from_utf8_unchecked(output) };
    //
    Ok(s)
}
