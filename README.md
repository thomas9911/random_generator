# simple nostd random generator

tldr; use rand

## Bench

```sh
cargo bench --features benchmarking
```

On windows with AMD 7950X (random here is from the rand crate):

```txt
32                      time:   [1.1450 ns 1.1466 ns 1.1482 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

64                      time:   [1.1700 ns 1.1780 ns 1.1865 ns]
Found 11 outliers among 100 measurements (11.00%)
  11 (11.00%) high mild
128                     time:   [1.5425 ns 1.5492 ns 1.5571 ns]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

usize                   time:   [1.1486 ns 1.1507 ns 1.1532 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

random                  time:   [587.95 ps 590.28 ps 592.64 ps]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
```
