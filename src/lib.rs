#[macro_use]
extern crate anyhow;

pub struct Teams {
    pub year: u16,
    pub teams: Vec<Team>,
}

pub struct Team {
    pub name: String,
    pub score: u8,
}

pub struct TeamsSo {
    pub teams: Vec<TeamSo>,
}

pub struct TeamSo {
    pub name: String,
}

mod ben_std_io_write;
pub use ben_std_io_write::do_te_std_io_write_big_table;
pub use ben_std_io_write::do_te_std_io_write_teams;
pub use ben_std_io_write::do_te_std_io_write_teams_so;

mod ben_std_fmt_write;
pub use ben_std_fmt_write::do_te_std_fmt_write_big_table;
pub use ben_std_fmt_write::do_te_std_fmt_write_teams;
pub use ben_std_fmt_write::do_te_std_fmt_write_teams_so;

mod ben_std_format;
pub use ben_std_format::do_te_std_format_big_table;
pub use ben_std_format::do_te_std_format_teams;
pub use ben_std_format::do_te_std_format_teams_so;

mod ben_askama;
pub use ben_askama::do_te_askama_big_table;
pub use ben_askama::do_te_askama_teams;
pub use ben_askama::do_te_askama_teams_so;

mod ben_sailfish;
pub use ben_sailfish::do_te_sailfish_big_table;
pub use ben_sailfish::do_te_sailfish_teams;
pub use ben_sailfish::do_te_sailfish_teams_so;

mod ben_sailfish_buf;
pub use ben_sailfish_buf::do_te_sailfish_buf_big_table;
pub use ben_sailfish_buf::do_te_sailfish_buf_teams;
pub use ben_sailfish_buf::do_te_sailfish_buf_teams_so;

mod ben_sailfish_buf_fmt;
pub use ben_sailfish_buf_fmt::do_te_sailfish_buf_fmt_big_table;
pub use ben_sailfish_buf_fmt::do_te_sailfish_buf_fmt_teams;
pub use ben_sailfish_buf_fmt::do_te_sailfish_buf_fmt_teams_so;

mod ben_tinytempl;
pub use ben_tinytempl::create_templ_big_table;
pub use ben_tinytempl::create_templ_teams;
pub use ben_tinytempl::do_te_tinytempl_big_table;
pub use ben_tinytempl::do_te_tinytempl_teams;
pub use ben_tinytempl::create_templ_teams_so;
pub use ben_tinytempl::do_te_tinytempl_teams_so;
