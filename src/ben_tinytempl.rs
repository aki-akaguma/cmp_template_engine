use super::Teams;
use serde::Serialize;
use tinytemplate::TinyTemplate;

const TEMPL_BIG_TABLE: &str = "# The Big Table
{{ for row in table }}
{{ for col in row }}| {col} {{ endfor }}|{{ endfor }}
";

pub fn create_templ_big_table<'a>() -> anyhow::Result<TinyTemplate<'a>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("one", TEMPL_BIG_TABLE)?;
    Ok(tt)
}

pub fn do_te_tinytempl_big_table<'a>(
    tt: &TinyTemplate<'a>,
    a_table: &[Vec<usize>],
) -> anyhow::Result<String> {
    //
    let context = BigTableTemplate { table: a_table };
    //
    match tt.render("one", &context) {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}

const TEMPL_TEAMS: &str = "# CSL {year}
=================

|num | name             | score             |
|---:|:-----------------|------------------:|
{{ for team in teams }}| {team.num} | {team.name} | {team.score} |
{{ endfor }}";

pub fn create_templ_teams<'a>() -> anyhow::Result<TinyTemplate<'a>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("one", TEMPL_TEAMS)?;
    Ok(tt)
}

pub fn do_te_tinytempl_teams<'a>(tt: &TinyTemplate<'a>, a_teams: &Teams) -> anyhow::Result<String> {
    //
    let mut v_teams = Vec::new();
    for (i, team) in a_teams.teams.iter().enumerate() {
        v_teams.push(TeamTemplate {
            num: i + 1,
            name: &team.name,
            score: team.score,
        });
    }
    let context = TeamsTemplate {
        year: a_teams.year,
        teams: &v_teams,
    };
    //
    match tt.render("one", &context) {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}

#[derive(Serialize)]
struct BigTableTemplate<'a> {
    table: &'a [Vec<usize>],
}

#[derive(Serialize)]
struct TeamsTemplate<'a> {
    year: u16,
    teams: &'a [TeamTemplate<'a>],
}

#[derive(Serialize)]
pub struct TeamTemplate<'a> {
    pub num: usize,
    pub name: &'a str,
    pub score: u8,
}
