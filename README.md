# AOC 2023

This was my first year doing Advent of Code day-by-day as the problems came out.
I'd played with Rust a little in the past, but thought I'd use this year as an opportunity to get more familiar with the language.
Inspired by [this article](https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/), I decided that this will also be my first year trying to optimise solutions to run everything under a second, instead of settling for "good enough".

Compared to the previous years I did in other languages, this year felt particularly math-heavy, but I'm glad I learned some new graph theory and linear algebra from the kind people in the [AOC subreddit](https://www.reddit.com/r/adventofcode/), or I never would have even solved some of the last few problems!

## Timings

Rustc 1.74.0, release build, Intel i7-8700

```text
Day  1 Part 1: 56049           | Time: 73.4µs
Day  1 Part 2: 54530           | Time: 614.2µs
Day  2 Part 1: 2810            | Time: 51.8µs
Day  2 Part 2: 69110           | Time: 72.3µs
Day  3 Part 1: 535351          | Time: 244.6µs
Day  3 Part 2: 87287096        | Time: 163.3µs
Day  4 Part 1: 26443           | Time: 315.1µs
Day  4 Part 2: 6284877         | Time: 317µs
Day  5 Part 1: 836040384       | Time: 41.5µs
Day  5 Part 2: 10834440        | Time: 205.7µs
Day  6 Part 1: 32076           | Time: 4.5µs
Day  6 Part 2: 34278221        | Time: 1.6µs
Day  7 Part 1: 248217452       | Time: 381µs
Day  7 Part 2: 245576185       | Time: 400.2µs
Day  8 Part 1: 16043           | Time: 324.2µs
Day  8 Part 2: 15726453850399  | Time: 1.7717ms
Day  9 Part 1: 1916822650      | Time: 156.7µs
Day  9 Part 2: 966             | Time: 154µs
Day 10 Part 1: 6907            | Time: 105.6µs
Day 10 Part 2: 541             | Time: 163µs
Day 11 Part 1: 10033566        | Time: 229.6µs
Day 11 Part 2: 560822911938    | Time: 182.4µs
Day 12 Part 1: 7251            | Time: 1.9886ms
Day 12 Part 2: 2128386729962   | Time: 27.3864ms
Day 13 Part 1: 28895           | Time: 151.2µs
Day 13 Part 2: 31603           | Time: 568.6µs
Day 14 Part 1: 109385          | Time: 96.9µs
Day 14 Part 2: 93102           | Time: 45.8512ms
Day 15 Part 1: 506437          | Time: 92.5µs
Day 15 Part 2: 288521          | Time: 347.3µs
Day 16 Part 1: 8901            | Time: 1.0926ms
Day 16 Part 2: 9064            | Time: 160.0389ms
Day 17 Part 1: 635             | Time: 8.4699ms
Day 17 Part 2: 734             | Time: 16.1579ms
Day 18 Part 1: 31171           | Time: 38µs
Day 18 Part 2: 131431655002266 | Time: 59.3µs
Day 19 Part 1: 331208          | Time: 196µs
Day 19 Part 2: 121464316215623 | Time: 175.3µs
Day 20 Part 1: 821985143       | Time: 2.0715ms
Day 20 Part 2: 240853834793347 | Time: 16.9894ms
Day 21 Part 1: 3782            | Time: 525.1µs
Day 21 Part 2: 630661863455116 | Time: 53.263ms
Day 22 Part 1: 530             | Time: 472.2µs
Day 22 Part 2: 93292           | Time: 7.8387ms
Day 23 Part 1: 2442            | Time: 323µs
Day 23 Part 2: 6898            | Time: 237.5629ms
Day 24 Part 1: 19976           | Time: 493.5µs
Day 24 Part 2: 849377770236905 | Time: 162.5µs
Day 25 Part 1: 592171          | Time: 3.7393ms
Total Time: 593.0204ms
```
