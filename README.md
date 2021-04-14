# cmp_template_engine
research: compare template-engine for non-html

## The Benchmark Results

The sailfish is the best performance.

|         `name`          |   `bench`   | `86_64:musl` | `arm64:musl` |
|:------------------------|------------:|------------:|------------:|
| sailfish_teams          |    0.220 uc |    0.629 uc |    2.510 uc |
| sailfish_buf_teams      |    0.248 uc |    0.812 uc |    2.764 uc |
| std_fmt_write_teams     |    0.654 uc |    1.634 uc |    8.476 uc |
| std_io_write_teams      |    0.750 uc |    1.659 uc |    9.639 uc |
| std_format_teams        |    1.005 uc |    2.503 uc |   14.570 uc |
| askama_teams            |    1.661 uc |    2.921 uc |   17.973 uc |
| tinytempl_teams         |    6.359 uc |   14.933 uc |   80.532 uc |

|         `name`          |   `bench`   | `86_64:musl` | `arm64:musl` |
|:------------------------|------------:|------------:|------------:|
| sailfish_bigtable       |   16.500 uc |   16.390 uc |  145.290 uc |
| sailfish_buf_bigtable   |   27.434 uc |   65.871 uc |  247.200 uc |
| std_fmt_write_bigtable  |  146.150 uc |  262.730 uc | 1474.900 uc |
| std_io_write_bigtable   |  170.600 uc |  292.220 uc | 1863.300 uc |
| askama_bigtable         |  248.830 uc |  399.960 uc | 2737.800 uc |
| std_format_bigtable     |  257.970 uc |  506.320 uc | 2482.900 uc |
| tinytempl_bigtable      |  319.690 uc |  470.880 uc | 3351.900 uc |

- rustc 1.51.0 (2fd73fabe 2021-03-23)
