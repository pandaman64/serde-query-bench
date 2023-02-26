
```shell
$ hyperfine --export-markdown result.md -N -w 100 './target/release/serde-query-bench' 'jq --raw-output ".[].sha" commits.json'
Benchmark 1: ./target/release/serde-query-bench
  Time (mean ± σ):       1.4 ms ±   0.2 ms    [User: 1.2 ms, System: 0.1 ms]
  Range (min … max):     1.2 ms …   3.8 ms    2015 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 2: jq --raw-output ".[].sha" commits.json
  Time (mean ± σ):      35.9 ms ±   1.1 ms    [User: 34.7 ms, System: 1.0 ms]
  Range (min … max):    34.6 ms …  41.1 ms    84 runs
 
Summary
  './target/release/serde-query-bench' ran
   25.83 ± 3.14 times faster than 'jq --raw-output ".[].sha" commits.json
```
