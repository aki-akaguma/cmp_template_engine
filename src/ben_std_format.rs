#[allow(unused_imports)]
use super::{Team, Teams, TeamSo,TeamsSo};

pub fn do_te_std_format_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    let mut s = String::with_capacity(13000);
    s += &format!(
        "# The Big Table

"
    );
    for row in a_table {
        for col in row {
            s += &format!("| {num} ", num = col);
        }
        s += &format!(
            "|
"
        );
    }
    //
    Ok(s)
}

pub fn do_te_std_format_teams(a_teams: &Teams) -> anyhow::Result<String> {
    let mut s = String::with_capacity(300);
    s += &format!(
        "# CSL {year}
=================

|num | name             | score             |
|---:|:-----------------|------------------:|
",
        year = a_teams.year
    );
    for (i, team) in a_teams.teams.iter().enumerate() {
        s += &format!(
            "| {num} | {name} | {score} |
",
            num = i + 1,
            name = team.name,
            score = team.score
        );
    }
    //
    Ok(s)
}

pub fn do_te_std_format_teams_so(a_teams: &TeamsSo) -> anyhow::Result<String> {
    let mut s = String::with_capacity(300);
    s += &format!(
        "# CSL
=================

| name             |
|:-----------------|
",
    );
    for team in a_teams.teams.iter() {
        s += &format!(
            "| {name} |
",
            name = team.name,
        );
    }
    //
    Ok(s)
}
