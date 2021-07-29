# Benchmark for lzd-rs

We measured the performances of lzd-rs and some other compressors on one core of octa-core Intel Core i7-10700 CPU @ 2.90GHz in a machine with 16GB of RAM running the 64-bit version of Ubuntu 20.04 LTS working on WSL2. The user CPU times for compression/decompression were measured by `/usr/bin/time` command.

## [Pizaa&Chili Text Corpus](http://pizzachili.dcc.uchile.cl/texts.html)

### english.100MB

| Compressor | Compress (sec) | Decompress (sec) | Compr. ratio |
| ---------- | -------------: | ---------------: | -----------: |
| lzd        |          19.69 |              1.7 |        32.1% |
| gzip       |           4.58 |              0.5 |        37.8% |
| bzip2      |            6.4 |             2.81 |        28.1% |
| lzma       |           48.8 |              0.9 |        22.4% |
| lz4 -1     |           0.24 |             0.05 |        62.7% |
| lz4 -9     |           4.59 |             0.04 |        43.3% |
| zstd -3    |           0.53 |             0.08 |        30.6% |
| zstd -19   |          43.57 |             0.12 |        22.7% |

### sources.100MB

| Compressor | Compress (sec) | Decompress (sec) | Compr. ratio |
| ---------- | -------------: | ---------------: | -----------: |
| lzd        |          14.69 |             1.39 |        28.2% |
| gzip       |           2.37 |             0.38 |        23.1% |
| bzip2      |           6.43 |             2.09 |        19.2% |
| lzma       |          31.33 |             0.81 |        16.2% |
| lz4 -1     |           0.17 |             0.03 |        37.4% |
| lz4 -9     |           2.67 |             0.05 |        27.3% |
| zstd -3    |           0.41 |             0.06 |        23.2% |
| zstd -19   |          33.53 |             0.13 |        16.8% |

### dblp.xml.100MB

| Compressor | Compress (sec) | Decompress (sec) | Compr. ratio |
| ---------- | -------------: | ---------------: | -----------: |
| lzd        |           7.86 |             1.17 |        14.8% |
| gzip       |            1.6 |             0.36 |        17.4% |
| bzip2      |           8.13 |             1.95 |        11.1% |
| lzma       |          25.81 |             0.51 |        11.7% |
| lz4 -1     |           0.14 |             0.02 |        26.3% |
| lz4 -9     |           1.55 |             0.02 |        19.2% |
| zstd -3    |           0.32 |             0.06 |        16.4% |
| zstd -19   |          39.31 |             0.05 |        11.7% |

## [Pizaa&Chili Repetitive Corpus](http://pizzachili.dcc.uchile.cl/repcorpus.html)

### Escherichia_Coli (107 MiB)

| Compressor | Compress (sec) | Decompress (sec) | Compr. ratio |
| ---------- | -------------: | ---------------: | -----------: |
| lzd        |           5.34 |             1.30 |        13.5% |
| gzip       |           9.28 |             0.46 |        28.9% |
| bzip2      |           7.09 |             3.12 |        27.0% |
| lzma       |          74.29 |             0.43 |         9.0% |
| lz4 -1     |           0.19 |             0.08 |        56.0% |
| lz4 -9     |          37.10 |             0.05 |        38.3% |
| zstd -3    |           0.55 |             0.11 |        30.2% |
| zstd -19   |          72.28 |             0.05 |         9.6% |

### einstein.de (88.5 MiB)

| Compressor | Compress (sec) | Decompress (sec) | Compr. ratio |
| ---------- | -------------: | ---------------: | -----------: |
| lzd        |           0.10 |             1.03 |         0.3% |
| gzip       |           2.96 |             0.41 |        31.2% |
| bzip2      |           7.11 |             1.54 |         4.3% |
| lzma       |           6.46 |             0.04 |         0.1% |
| lz4 -1     |           0.10 |             0.01 |        28.0% |
| lz4 -9     |           0.77 |             0.03 |        10.2% |
| zstd -3    |           0.02 |             0.03 |         0.6% |
| zstd -19   |           0.99 |             0.00 |         0.1% |

### world_leaders (44.8 MiB)

| Compressor | Compress (sec) | Decompress (sec) | Compr. ratio |
| ---------- | -------------: | ---------------: | -----------: |
| lzd        |           0.36 |             0.41 |         2.9% |
| gzip       |           0.75 |             0.14 |        17.9% |
| bzip2      |           1.79 |             0.54 |         6.9% |
| lzma       |           6.09 |             0.00 |         1.3% |
| lz4 -1     |           0.06 |             0.03 |        28.4% |
| lz4 -9     |           1.50 |             0.02 |        20.2% |
| zstd -3    |           0.04 |             0.00 |         4.8% |
| zstd -19   |           3.04 |             0.01 |         1.3% |

## [Silesia Compression Corpus](http://sun.aei.polsl.pl/~sdeor/index.php?page=silesia)

### silesia.tar (202 MiB)

| Compressor | Compress (sec) | Decompress (sec) | Compr. ratio |
| ---------- | -------------: | ---------------: | -----------: |
| lzd        |         160.81 |             2.79 |        40.4% |
| gzip       |           5.78 |             0.90 |        32.2% |
| bzip2      |          14.05 |             4.94 |        25.7% |
| lzma       |          67.55 |             2.35 |        23.2% |
| lz4 -1     |           0.28 |             0.07 |        47.6% |
| lz4 -9     |           5.98 |             0.05 |        36.8% |
| zstd -3    |           0.89 |             0.18 |        31.5% |
| zstd -19   |          68.86 |             0.24 |        25.0% |