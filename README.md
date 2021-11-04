# cmp_template_engine
research: compare template-engine for non-html

## The Benchmark Results

The sailfish is the best performance.

- rustc 1.56.1 (59eed8a2a 2021-11-01)

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams             |    0.191 uc |    0.446 uc |
| sailf_buf_teams         |    0.243 uc |    0.780 uc |
| sailf_buf_fmt_teams     |    0.636 uc |    1.682 uc |
| std_fmt_write_teams     |    0.656 uc |    1.511 uc |
| std_io_write_teams      |    0.697 uc |    1.585 uc |
| std_format_teams        |    1.096 uc |    2.584 uc |
| askama_teams            |    1.481 uc |    2.375 uc |
| tinytempl_teams         |    6.229 uc |   14.323 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_bigtable          |   16.312 uc |   15.279 uc |
| sailf_buf_bigtable      |   25.442 uc |   65.468 uc |
| std_fmt_write_bigtable  |  145.940 uc |  258.190 uc |
| sailf_buf_fmt_bigtable  |  151.220 uc |  271.180 uc |
| std_io_write_bigtable   |  161.530 uc |  274.870 uc |
| askama_bigtable         |  225.530 uc |  339.820 uc |
| std_format_bigtable     |  260.350 uc |  506.360 uc |
| tinytempl_bigtable      |  297.220 uc |  408.320 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams_so          |    0.078 uc |    0.428 uc |
| sailf_buf_teams_so      |    0.187 uc |    0.425 uc |
| std_fmt_write_teams_so  |    0.306 uc |    0.862 uc |
| std_io_write_teams_so   |    0.336 uc |    0.899 uc |
| sailf_buf_fmt_teams_so  |    0.339 uc |    0.903 uc |
| askama_teams_so         |    0.582 uc |    0.977 uc |
| std_format_teams_so     |    0.751 uc |    2.259 uc |
| tinytempl_teams_so      |    2.172 uc |    6.526 uc |

- rustc 1.53.0 (53cb7b09b 2021-06-17)

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams             |    0.207 uc |    0.461 uc |
| sailf_buf_teams         |    0.392 uc |    0.785 uc |
| std_fmt_write_teams     |    0.930 uc |    1.581 uc |
| std_io_write_teams      |    0.958 uc |    1.599 uc |
| sailf_buf_fmt_teams     |    0.962 uc |    1.555 uc |
| std_format_teams        |    1.339 uc |    2.557 uc |
| askama_teams            |    1.930 uc |    2.882 uc |
| tinytempl_teams         |    6.005 uc |   14.678 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_bigtable          |   16.746 uc |   16.199 uc |
| sailf_buf_bigtable      |   37.075 uc |   66.046 uc |
| std_fmt_write_bigtable  |  169.780 uc |  257.310 uc |
| sailf_buf_fmt_bigtable  |  176.180 uc |  259.830 uc |
| std_io_write_bigtable   |  197.430 uc |  273.570 uc |
| askama_bigtable         |  268.530 uc |  394.180 uc |
| std_format_bigtable     |  282.360 uc |  517.690 uc |
| tinytempl_bigtable      |  339.820 uc |  448.830 uc |

|         `name`          |   `bench`   |   `:musl`   |
|:------------------------|------------:|------------:|
| sailf_teams_so          |    0.110 uc |    0.412 uc |
| sailf_buf_teams_so      |    0.117 uc |    0.518 uc |
| std_fmt_write_teams_so  |    0.322 uc |    0.861 uc |
| sailf_buf_fmt_teams_so  |    0.374 uc |    0.856 uc |
| std_io_write_teams_so   |    0.377 uc |    0.885 uc |
| askama_teams_so         |    0.684 uc |    1.058 uc |
| std_format_teams_so     |    0.771 uc |    2.296 uc |
| tinytempl_teams_so      |    2.401 uc |    6.842 uc |

- rustc 1.52.0 (88f19c6da 2021-05-03)

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
