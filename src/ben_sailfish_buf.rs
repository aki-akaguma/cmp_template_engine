//use std::fmt::Write;

#[allow(unused_imports)]
use super::{Team, Teams};
use sailfish::runtime::Buffer;

pub fn do_te_sailfish_buf_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    let mut buf = Buffer::with_capacity(13000);
    buf += "# The Big Table\n\n";
    for row in a_table {
        buf += "| ";
        //
        let last_idx = row.len()-1;
        for col in row[..last_idx].iter() {
            //write!(&mut buf, "| {num} ", num = col)?;
            let _ = itoap::fmt(&mut buf, *col);
            buf += " | ";
        }
        let _ = itoap::fmt(&mut buf, row[last_idx]);
        buf += " |\n";
    }
    //
    Ok(buf.into_string())
}

pub fn do_te_sailfish_buf_teams(a_teams: &Teams) -> anyhow::Result<String> {
    let mut buf = Buffer::with_capacity(300);
    //write!(&mut buf, "# CSL {year}", year = a_teams.year)?;
    buf += "# CSL ";
    itoap::fmt(&mut buf, a_teams.year)?;
    buf += "
=================

|num | name             | score             |
|---:|:-----------------|------------------:|
";
    for (i, team) in a_teams.teams.iter().enumerate() {
        /*
        write!(
            &mut buf,
            "| {num} | {name} | {score} |\n",
            num = i + 1,
            name = team.name,
            score = team.score
        )?;
        */
        buf += "| ";
        let _ = itoap::fmt(&mut buf, i + 1);
        buf += " | ";
        buf += &team.name;
        buf += " | ";
        let _ = itoap::fmt(&mut buf, team.score);
        buf += " |\n";
    }
    //
    Ok(buf.into_string())
}
