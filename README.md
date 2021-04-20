## Benchmarks

### Fully sequential:

60 Kb file (paths.csv):  
calc fares small file   time:   [790.66 us 794.63 us 799.24 us]

14 Mb file:  
calc fares medium file  time:   [187.27 ms 189.27 ms 191.88 ms] 

673 Mb file (large.csv):  
calc fares large file   time:   [8.3125 s 8.3546 s 8.4021 s]

### Parallel fare calculation:
60 Kb file (paths.csv):  
calc_fares_small_file   time:   [645.48 us 648.72 us 652.28 us]

14 Mb file:  
calc_fares_medium_file  time:   [116.31 ms 116.70 ms 117.12 ms]

673 Mb file (large.csv):  