# bio-jtools-rs
A suite of bioinformatics tools for interacting with high throughput sequencing (HTS) data, written entirely in Rust

## Benchmarking

Benchmarks on run in a Windows 10 computer, Intel i5 750 @ 2.67 GHz processor with 12 GB of RAM.
Times are listed +/- standard deviation.

`hyperfine --warmup 2 'bjt info <fq>'`:

| File | # Reads | t<sub>gzip</sub> (s) | t<sub>plain</sub> (s) |
| ---------------------- | ------- | -------------------- | --------------------- |
| [`M_abscessus_HiSeq.fq`](https://lh3.github.io/2020/05/17/fast-high-level-programming-languages) | 5 682 010 | 12.323 +/- 0.227 | 0.9515 + 0.0056 |
