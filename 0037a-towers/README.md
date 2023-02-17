# 37A. Towers

## Constraints

  - Time limit per test: 2 seconds
  - Memory limit per test: 256 megabytes
  - Input: standard input
  - Output: standard output

## Problem

[Link to Codeforces](https://codeforces.com/problemset/problem/37/A)

Little Vasya has received a young builder’s kit. The kit consists of several wooden bars, the lengths of all of them are known. The bars can be put one on the top of the other if their lengths are the same.

Vasya wants to construct the minimal number of towers from the bars. Help Vasya to use the bars in the best way possible.

## Input

The first line contains an integer N (1 <= N <= 1000) — the number of bars at Vasya’s disposal. The second line contains N space-separated integers li — the lengths of the bars. All the lengths are natural numbers not exceeding 1000.

## Output

In one line output two numbers — the height of the largest tower and their total number. Remember that Vasya should use all the bars.

## Examples

### Input

[Link to file](input-0.txt)

```
3
1 2 3
```

### Output

[Link to file](expected-0.txt)

```
1 3
```

### Input

[Link to file](input-1.txt)

```
4
6 5 6 7
```

### Output

[Link to file](expected-1.txt)

```
2 3
```

## Solutions

### Rust 1.66.0

[Link to file](solution.rs)

### Python 3.8.10

[Link to file](solution.py)

### GNU C++17 7.3.0

[Link to file](solution.cpp)
