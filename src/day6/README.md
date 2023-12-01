# Benchmark Results

*Note: the smaller ns per iteration, the better.*

| day6::tests::bench_part2_even_smarter_loop | 345 ns/iter     | (+/- 2)     |
|--------------------------------------|-----------------|-------------|
| day6::tests::bench_part2_hashset           | 358,860 ns/iter | (+/- 5,579) |
| day6::tests::bench_part2_naive_loop        | 3,332 ns/iter   | (+/- 2,727) |
| day6::tests::bench_part2_smart_loop        | 2,128 ns/iter   | (+/- 49)    |