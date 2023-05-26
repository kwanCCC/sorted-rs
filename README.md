[![build](https://github.com/kwanCCC/sorted-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/kwanCCC/sorted-rs/actions/workflows/rust.yml)

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

| size  | generic   | sse       | avx2      |  
|-------|-----------|-----------|-----------|
| 4     | 7.4974 ns | 8.3876 ns | 1.1984 ns |
| 32    | 65.302 ns | 40.632 ns | 16.469 ns |
| 64    | 121.92 ns | 71.861 ns | 30.551 ns |
| 128   | 237.09 ns | 144.08 ns | 66.191 ns |
| 256   | 466.77 ns | 273.48 ns | 99.640 ns |
| 1024  | 1.8486 µs | 1.0634 µs | 410.66 ns |
| 4096  | 7.3648 µs | 4.2080 µs | 6.1038 µs |
| 16384 | 29.458 µs | 16.702 µs | 6.1038 µs |
| 65536 | 117.82 µs | 66.982 µs | 24.355 µs |
