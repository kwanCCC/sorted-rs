## Another algorithm base on SIMD which use to check slice is sorted or not

It both support `SSE2` and `AVX2` and much faster than regular implementations.

### generic

```
 [ a, b, c, d .... x, y, z]
 compare a and b,
 compare b and c,
 compare c and d
```

### SIMD

```
 [ a, b, c, d .... x, y, z]
 load [a, b, c, d] into chunk0,
 load [b, c, d, d] into chunk1,
 compare chunk0 and chunk1,
 load [d, e, f, g] into chunk0,
 load [e, f, g, g] into chunk1,
 compare chunk0 and chunk1 again
```

### Benchmark on AMD

```log

sorted small/ascending/4
                        time:   [299.56 ps 299.59 ps 299.63 ps]
                        thrpt:  [49.731 GiB/s 49.738 GiB/s 49.744 GiB/s]
sorted small/descending/4
                        time:   [300.11 ps 302.55 ps 305.78 ps]
                        thrpt:  [48.732 GiB/s 49.252 GiB/s 49.652 GiB/s]
sorted small/generic ascending/4
                        time:   [299.48 ps 300.23 ps 301.87 ps]
                        thrpt:  [49.362 GiB/s 49.632 GiB/s 49.756 GiB/s]
sorted small/generic descending/4
                        time:   [299.39 ps 300.10 ps 301.67 ps]
                        thrpt:  [49.396 GiB/s 49.654 GiB/s 49.771 GiB/s]
sorted small/ascending/32
                        time:   [40.438 ns 40.561 ns 40.818 ns]
                        thrpt:  [2.9205 GiB/s 2.9390 GiB/s 2.9479 GiB/s]
sorted small/descending/32
                        time:   [40.409 ns 40.449 ns 40.489 ns]
                        thrpt:  [2.9442 GiB/s 2.9472 GiB/s 2.9500 GiB/s]
sorted small/generic ascending/32
                        time:   [299.54 ps 299.58 ps 299.63 ps]
                        thrpt:  [397.85 GiB/s 397.92 GiB/s 397.97 GiB/s]
sorted small/generic descending/32
                        time:   [299.52 ps 300.97 ps 303.17 ps]
                        thrpt:  [393.20 GiB/s 396.09 GiB/s 398.00 GiB/s]
sorted small/ascending/64
                        time:   [71.797 ns 71.990 ns 72.342 ns]
                        thrpt:  [3.2957 GiB/s 3.3118 GiB/s 3.3207 GiB/s]
sorted small/descending/64
                        time:   [71.438 ns 71.492 ns 71.546 ns]
                        thrpt:  [3.3324 GiB/s 3.3349 GiB/s 3.3374 GiB/s]
sorted small/generic ascending/64
                        time:   [299.37 ps 299.39 ps 299.42 ps]
                        thrpt:  [796.26 GiB/s 796.34 GiB/s 796.40 GiB/s]
sorted small/generic descending/64
                        time:   [299.38 ps 299.41 ps 299.46 ps]
                        thrpt:  [796.17 GiB/s 796.28 GiB/s 796.37 GiB/s]
sorted small/ascending/128
                        time:   [144.82 ns 144.99 ns 145.15 ns]
                        thrpt:  [3.2851 GiB/s 3.2887 GiB/s 3.2926 GiB/s]
sorted small/descending/128
                        time:   [143.95 ns 144.42 ns 145.16 ns]
                        thrpt:  [3.2850 GiB/s 3.3017 GiB/s 3.3126 GiB/s]
sorted small/generic ascending/128
                        time:   [299.48 ps 300.14 ps 301.57 ps]
                        thrpt:  [1581.2 GiB/s 1588.7 GiB/s 1592.2 GiB/s]
sorted small/generic descending/128
                        time:   [299.45 ps 299.48 ps 299.52 ps]
                        thrpt:  [1592.0 GiB/s 1592.2 GiB/s 1592.4 GiB/s]
sorted small/ascending/256
                        time:   [269.24 ns 269.47 ns 269.70 ns]
                        thrpt:  [3.5360 GiB/s 3.5390 GiB/s 3.5421 GiB/s]
sorted small/descending/256
                        time:   [265.76 ns 266.01 ns 266.25 ns]
                        thrpt:  [3.5819 GiB/s 3.5851 GiB/s 3.5885 GiB/s]
sorted small/generic ascending/256
                        time:   [299.35 ps 299.38 ps 299.41 ps]
                        thrpt:  [3185.1 GiB/s 3185.5 GiB/s 3185.8 GiB/s]
sorted small/generic descending/256
                        time:   [299.59 ps 299.67 ps 299.78 ps]
                        thrpt:  [3181.2 GiB/s 3182.4 GiB/s 3183.3 GiB/s]
sorted large/ascending/1024
                        time:   [1.0169 µs 1.0175 µs 1.0181 µs]
                        thrpt:  [3.7468 GiB/s 3.7491 GiB/s 3.7514 GiB/s]
sorted large/descending/1024
                        time:   [1.0070 µs 1.0076 µs 1.0082 µs]
                        thrpt:  [3.7835 GiB/s 3.7858 GiB/s 3.7881 GiB/s]
sorted large/generic ascending/1024
                        time:   [299.47 ps 299.53 ps 299.60 ps]
                        thrpt:  [ 12733 GiB/s  12736 GiB/s  12738 GiB/s]
sorted large/generic descending/1024
                        time:   [299.62 ps 300.45 ps 302.12 ps]
                        thrpt:  [ 12626 GiB/s  12697 GiB/s  12732 GiB/s]
sorted large/ascending/4096
                        time:   [4.0074 µs 4.0086 µs 4.0100 µs]
                        thrpt:  [3.8052 GiB/s 3.8065 GiB/s 3.8077 GiB/s]
sorted large/descending/4096
                        time:   [3.9867 µs 3.9882 µs 3.9899 µs]
                        thrpt:  [3.8244 GiB/s 3.8260 GiB/s 3.8274 GiB/s]
sorted large/generic ascending/4096
                        time:   [300.24 ps 300.40 ps 300.57 ps]
                        thrpt:  [ 50767 GiB/s  50795 GiB/s  50821 GiB/s]
sorted large/generic descending/4096
                        time:   [299.59 ps 299.68 ps 299.80 ps]
                        thrpt:  [ 50897 GiB/s  50917 GiB/s  50933 GiB/s]
sorted large/ascending/16384
                        time:   [15.974 µs 16.032 µs 16.128 µs]
                        thrpt:  [3.7845 GiB/s 3.8070 GiB/s 3.8209 GiB/s]
sorted large/descending/16384
                        time:   [15.895 µs 15.940 µs 16.036 µs]
                        thrpt:  [3.8062 GiB/s 3.8291 GiB/s 3.8399 GiB/s]
sorted large/generic ascending/16384
                        time:   [299.51 ps 299.55 ps 299.60 ps]
                        thrpt:  [203725 GiB/s 203759 GiB/s 203785 GiB/s]
sorted large/generic descending/16384
                        time:   [299.46 ps 299.65 ps 299.87 ps]
                        thrpt:  [203537 GiB/s 203689 GiB/s 203814 GiB/s]
sorted large/ascending/65536
                        time:   [63.802 µs 63.813 µs 63.827 µs]
                        thrpt:  [3.8250 GiB/s 3.8259 GiB/s 3.8265 GiB/s]
sorted large/descending/65536
                        time:   [63.552 µs 63.572 µs 63.604 µs]
                        thrpt:  [3.8384 GiB/s 3.8404 GiB/s 3.8416 GiB/s]
sorted large/generic ascending/65536
                        time:   [299.63 ps 299.68 ps 299.74 ps]
                        thrpt:  [814501 GiB/s 814668 GiB/s 814817 GiB/s]
sorted large/generic descending/65536
                        time:   [299.59 ps 299.65 ps 299.72 ps]
                        thrpt:  [814570 GiB/s 814762 GiB/s 814909 GiB/s]
```
