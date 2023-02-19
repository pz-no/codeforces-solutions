# 41A. Translation

## Constraints

  - Time limit per test: 2 seconds
  - Memory limit per test: 256 megabytes
  - Input: standard input
  - Output: standard output

## Problem

[Link to Codeforces](https://codeforces.com/problemset/problem/41/A)

The translation from the Berland language into the Birland language is not an easy task. Those languages are very similar: a berlandish word differs from a birlandish word with the same meaning a little: it is spelled (and pronounced) reversely. For example, a Berlandish word code corresponds to a Birlandish word edoc. However, it's easy to make a mistake during the «translation». Vasya translated word s from Berlandish into Birlandish as t. Help him: find out if he translated the word correctly.

## Input

The first line contains word $s$, the second line contains word $t$. The words consist of lowercase Latin letters. The input data do not consist unnecessary spaces. The words are not empty and their lengths do not exceed 100 symbols.

## Output

If the word $t$ is a word $s$, written reversely, print YES, otherwise print NO.

## Examples

### Input

[Link to file](input-0.txt)

```
code
edoc
```

### Output

[Link to file](expected-0.txt)

```
YES
```

### Input

[Link to file](input-1.txt)

```
abb
aba
```

### Output

[Link to file](expected-1.txt)

```
NO
```

### Input

[Link to file](input-2.txt)

```
code
code
```

### Output

[Link to file](expected-2.txt)

```
NO
```

## Solutions

### GNU C++17 7.3.0

[Link to file](solution.cpp) (time: 30 ms, memory: 8 KB)

### Python 3.8.10

[Link to file](solution.py) (time: 92 ms, memory: 0 KB)
