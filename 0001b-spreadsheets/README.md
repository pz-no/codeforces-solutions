# 1B. Spreadsheets

## Constraints

  - Time limit per test: 10 seconds
  - Memory limit per test: 64 megabytes
  - Input: standard input
  - Output: standard output

## Problem

[Link to Codeforces](https://codeforces.com/problemset/problem/1/B)

In the popular spreadsheets systems (for example, in Excel) the following numeration of columns is used. The first column has number A, the second — number B, etc. till column 26 that is marked by Z. Then there are two-letter numbers: column 27 has number AA, 28 — AB, column 52 is marked by AZ. After ZZ there follow three-letter numbers, etc.

The rows are marked by integer numbers starting with 1. The cell name is the concatenation of the column and the row numbers. For example, BC23 is the name for the cell that is in column 55, row 23.

Sometimes another numeration system is used: RXCY, where X and Y are integer numbers, showing the column and the row numbers respectfully. For instance, R23C55 is the cell from the previous example.

Your task is to write a program that reads the given sequence of cell coordinates and produce each item written according to the rules of another numeration system.

## Input

The first line of the input contains integer number $n$ $(1 \leq n \leq 10^5)$, the number of coordinates in the test. Then there follow n lines, each of them contains coordinates. All the coordinates are correct, there are no cells with the column and/or the row numbers larger than $10^6$.

## Output

Write $n$ lines, each line should contain a cell coordinates in the other numeration system.

## Examples

### Input

[Link to file](input-0.txt)

```
2
R23C55
BC23
```

### Output

[Link to file](expected-0.txt)

```
BC23
R23C55
```

## Solutions

### Python 3.8.10

[Link to file](solution.py) (time: 2774 ms, memory: 668 KB)

### GNU C++17 7.3.0

[Link to file](solution.cpp) (time: 156 ms, memory: 144 KB)
