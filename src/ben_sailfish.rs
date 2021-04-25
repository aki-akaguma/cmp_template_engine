use super::{Team, Teams, TeamSo,TeamsSo};
use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "big-table.stpl")]
struct BigTableTemplate<'a> {
    table: &'a [Vec<usize>],
}

pub fn do_te_sailfish_big_table(a_table: &[Vec<usize>]) -> anyhow::Result<String> {
    let ctx = BigTableTemplate { table: a_table };
    match ctx.render_once() {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}

#[derive(TemplateOnce)]
#[template(path = "teams.stpl")]
struct TeamsTemplate<'a> {
    year: u16,
    teams: &'a [Team],
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
#[template(path = "teams-so.stpl")]
struct TeamsSoTemplate<'a> {
    teams: &'a [TeamSo],
}

pub fn do_te_sailfish_teams_so(a_teams: &TeamsSo) -> anyhow::Result<String> {
    let ctx = TeamsSoTemplate {
        teams: &a_teams.teams,
    };
    match ctx.render_once() {
        Ok(s) => Ok(s),
        Err(err) => Err(anyhow!("{}", err)),
    }
}
