use super::{Team, Teams};
use sailfish::TemplateOnce;

pub fn do_te_sailfish_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    let ctx = BigTableTemplate { table: a_table };
    match ctx.render_once() {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}

pub fn do_te_sailfish_teams(a_teams: &Teams) -> anyhow::Result<String> {
    let ctx = TeamsTemplate {
        year: a_teams.year,
        teams: &a_teams.teams,
    };
    match ctx.render_once() {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}

#[derive(TemplateOnce)]
#[template(path = "big-table.stpl")]
struct BigTableTemplate<'a> {
    table: &'a [Vec<usize>],
}

#[derive(TemplateOnce)]
#[template(path = "teams.stpl")]
struct TeamsTemplate<'a> {
    year: u16,
    teams: &'a [Team],
}
