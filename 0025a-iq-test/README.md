# 25A. IQ test

## Constraints

  - Time limit per test: 2 seconds
  - Memory limit per test: 256 megabytes
  - Input: standard input
  - Output: standard output

## Problem

[Link to Codeforces](https://codeforces.com/problemset/problem/25/A)

Bob is preparing to pass IQ test. The most frequent task in this test is to find out which one of the given n numbers differs from the others. Bob observed that one number usually differs from the others in evenness. Help Bob — to check his answers, he needs a program that among the given n numbers finds one that is different in evenness.

## Input

The first line contains integer $n$ $(3 \leq n \leq 100)$ — amount of numbers in the task. The second line contains n space-separated natural numbers, not exceeding 100. It is guaranteed, that exactly one of these numbers differs from the others in evenness.

## Output

Output index of number that differs from the others in evenness. Numbers are numbered from 1 in the input order.

## Examples

### Input

[Link to file](input-0.txt)

```
5
2 4 7 8 10
```

### Output

[Link to file](expected-0.txt)

```
3
```

### Input

[Link to file](input-1.txt)

```
4
1 2 1 1
```

### Output

[Link to file](expected-1.txt)

```
2
```

## Solutions

### Rust 1.66.0

[Link to file](solution.rs)

### GNU C++17 7.3.0

[Link to file](solution.cpp)

### Python 3.8.10

[Link to file](solution.py)
