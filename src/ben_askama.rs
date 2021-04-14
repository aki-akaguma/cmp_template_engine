use super::{Team, Teams};
use askama::Template;

pub fn do_te_askama_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    let ctx = BigTableTemplate { table: a_table };
    match ctx.render() {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}

pub fn do_te_askama_teams(a_teams: &Teams) -> anyhow::Result<String> {
    let ctx = TeamsTemplate {
        year: a_teams.year,
        teams: &a_teams.teams,
    };
    match ctx.render() {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}

#[derive(Template)]
#[template(path = "big-table.atpl", escape = "none")]
struct BigTableTemplate<'a> {
    table: &'a [Vec<usize>],
}

#[derive(Template)]
#[template(path = "teams.atpl", escape = "none")]
struct TeamsTemplate<'a> {
    year: u16,
    teams: &'a [Team],
}
