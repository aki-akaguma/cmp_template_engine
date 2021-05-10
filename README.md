# cmp_template_engine
research: compare template-engine for non-html

## The Benchmark Results

The sailfish is the best performance.

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_buf_teams         |    0.243 uc |    0.745 uc |
| sailf_teams             |    0.296 uc |    0.445 uc |
| std_fmt_write_teams     |    0.645 uc |    1.710 uc |
| sailf_buf_fmt_teams     |    0.684 uc |    1.581 uc |
| std_io_write_teams      |    0.742 uc |    1.733 uc |
| std_format_teams        |    1.010 uc |    2.607 uc |
| askama_teams            |    1.649 uc |    2.838 uc |
| tinytempl_teams         |    6.260 uc |   14.020 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_bigtable          |   16.231 uc |   15.607 uc |
| sailf_buf_bigtable      |   27.213 uc |   65.530 uc |
| std_fmt_write_bigtable  |  144.790 uc |  261.650 uc |
| sailf_buf_fmt_bigtable  |  145.580 uc |  257.940 uc |
| std_io_write_bigtable   |  167.340 uc |  285.520 uc |
| askama_bigtable         |  237.780 uc |  387.520 uc |
| std_format_bigtable     |  248.950 uc |  500.130 uc |
| tinytempl_bigtable      |  310.300 uc |  461.780 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams_so          |    0.077 uc |    0.355 uc |
| sailf_buf_teams_so      |    0.096 uc |    0.416 uc |
| sailf_buf_fmt_teams_so  |    0.306 uc |    0.862 uc |
| std_fmt_write_teams_so  |    0.307 uc |    0.862 uc |
| std_io_write_teams_so   |    0.366 uc |    0.894 uc |
| askama_teams_so         |    0.620 uc |    0.996 uc |
| std_format_teams_so     |    0.740 uc |    2.153 uc |
| tinytempl_teams_so      |    2.084 uc |    6.960 uc |

- rustc 1.52.0 (88f19c6da 2021-05-03)
