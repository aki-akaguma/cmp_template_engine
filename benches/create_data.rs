use cmp_template_engine::{Team, Teams};

#[allow(dead_code)]
pub fn create_data_big_table() -> (Vec<Vec<usize>>, String) {
    let size: usize = 50;
    let mut table = Vec::with_capacity(size);
    for _i in 0..size {
        let mut inner = Vec::with_capacity(size);
        for j in 0..size {
            inner.push(j);
        }
        table.push(inner);
    }
    //
    let mut v: Vec<String> = Vec::new();
    v.push("# The Big Table\n\n".into());
    for _i in 0..size {
        let mut vv: Vec<String> = Vec::new();
        for j in 0..size {
            vv.push(format!("| {} ", j));
        }
        v.push(vv.join("") + "|\n");
    }
    let res = v.join("");
    //
    (table, res.into())
}

#[allow(dead_code)]
pub fn create_data_teams() -> (Teams, String) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };
    let res = r#"# CSL 2015
=================

|num | name             | score             |
|---:|:-----------------|------------------:|
| 1 | Jiangsu | 43 |
| 2 | Beijing | 27 |
| 3 | Guangzhou | 22 |
| 4 | Shandong | 12 |
"#;
    (teams, res.into())
}
