## Benchmarks

Time to beat:
Small file:
682572 ns/op (683 us)

Medium file:
143.430250 ns/op (143 ms)

Large file:
6.004921609 ns/op (6 s)

### Fully sequential

60 Kb file (paths.csv):  
calc fares small file   time:   [790.66 us 794.63 us 799.24 us]

14 Mb file:  
calc fares medium file  time:   [187.27 ms 189.27 ms 191.88 ms]

673 Mb file (large.csv):  
calc fares large file   time:   [8.3125 s 8.3546 s 8.4021 s]

### Parallel fare calculation

60 Kb file (paths.csv):  
calc_fares_small_file   time:   [645.48 us 648.72 us 652.28 us]

14 Mb file:  
calc_fares_medium_file  time:   [116.31 ms 116.70 ms 117.12 ms]

673 Mb file (large.csv):  
calc_fares_large_file   time:   [5.1477 s 5.1674 s 5.1896 s]

### Streaming file data, but calculations and writing are sequential

60 Kb file (paths.csv):  
calc_fares_small_file   time:   [553.61 us 560.56 us 567.91 us]

14 Mb file:  
calc_fares_medium_file  time:   [90.362 ms 90.675 ms 91.024 ms]

673 Mb file (large.csv):  
calc_fares_large_file   time:   [3.7811 s 3.8324 s 3.8986 s]

### Streaming file data and fare calculations but writing is sequential

60 Kb file (paths.csv):  
calc_fares_small_file   time:   [519.33 us 523.99 us 529.29 us]

14 Mb file:  
calc_fares_medium_file  time:   [89.155 ms 89.808 ms 90.666 ms]

673 Mb file (large.csv):  
calc_fares_large_file   time:   [3.8476 s 3.8683 s 3.8929 s]                                     
