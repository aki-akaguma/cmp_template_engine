# cmp_template_engine
research: compare template-engine for non-html

## The Benchmark Results

The sailfish is the best performance.

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams             |    0.194 uc |    0.453 uc |
| sailf_buf_teams         |    0.248 uc |    0.777 uc |
| sailf_buf_fmt_teams     |    0.621 uc |    1.559 uc |
| std_fmt_write_teams     |    0.711 uc |    1.694 uc |
| std_io_write_teams      |    0.741 uc |    1.734 uc |
| std_format_teams        |    1.025 uc |    2.617 uc |
| askama_teams            |    1.877 uc |    2.872 uc |
| tinytempl_teams         |    7.004 uc |   14.514 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_bigtable          |   16.094 uc |   16.916 uc |
| sailf_buf_bigtable      |   25.146 uc |   65.350 uc |
| sailf_buf_fmt_bigtable  |  146.840 uc |  263.270 uc |
| std_fmt_write_bigtable  |  152.610 uc |  265.060 uc |
| std_io_write_bigtable   |  169.020 uc |  290.800 uc |
| askama_bigtable         |  257.490 uc |  399.350 uc |
| std_format_bigtable     |  283.740 uc |  506.190 uc |
| tinytempl_bigtable      |  315.120 uc |  485.390 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams_so          |    0.072 uc |    0.412 uc |
| sailf_buf_teams_so      |    0.096 uc |    0.426 uc |
| sailf_buf_fmt_teams_so  |    0.324 uc |    0.868 uc |
| std_fmt_write_teams_so  |    0.333 uc |    0.883 uc |
| std_io_write_teams_so   |    0.366 uc |    0.918 uc |
| askama_teams_so         |    0.652 uc |    1.021 uc |
| std_format_teams_so     |    0.755 uc |    2.243 uc |
| tinytempl_teams_so      |    2.602 uc |    7.504 uc |

- rustc 1.51.0 (2fd73fabe 2021-03-23)

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams             |    0.194 uc |    0.441 uc |
| sailf_buf_teams         |    0.228 uc |    0.737 uc |
| std_io_write_teams      |    0.617 uc |    1.548 uc |
| sailf_buf_fmt_teams     |    0.628 uc |    1.539 uc |
| std_fmt_write_teams     |    0.659 uc |    1.516 uc |
| std_format_teams        |    0.991 uc |    2.539 uc |
| askama_teams            |    1.779 uc |    2.852 uc |
| tinytempl_teams         |    6.173 uc |   13.850 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_bigtable          |   16.494 uc |   16.358 uc |
| sailf_buf_bigtable      |   23.670 uc |   63.961 uc |
| std_fmt_write_bigtable  |  141.450 uc |  251.230 uc |
| sailf_buf_fmt_bigtable  |  148.690 uc |  263.430 uc |
| std_io_write_bigtable   |  164.080 uc |  275.190 uc |
| askama_bigtable         |  231.750 uc |  388.500 uc |
| std_format_bigtable     |  246.150 uc |  498.710 uc |
| tinytempl_bigtable      |  292.680 uc |  446.410 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_buf_teams_so      |    0.070 uc |    0.414 uc |
| sailf_teams_so          |    0.078 uc |    0.418 uc |
| std_fmt_write_teams_so  |    0.311 uc |    0.852 uc |
| sailf_buf_fmt_teams_so  |    0.318 uc |    0.875 uc |
| std_io_write_teams_so   |    0.327 uc |    1.005 uc |
| askama_teams_so         |    0.615 uc |    1.002 uc |
| std_format_teams_so     |    0.733 uc |    2.148 uc |
| tinytempl_teams_so      |    2.242 uc |    6.513 uc |

- rustc 1.53.0-nightly (bb491ed23 2021-04-23)
