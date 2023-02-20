# 4C. Registration system

## Constraints

  - Time limit per test: 5 seconds
  - Memory limit per test: 64 megabytes
  - Input: standard input
  - Output: standard output

## Problem

[Link to Codeforces](https://codeforces.com/problemset/problem/4/C)

A new e-mail service "Berlandesk" is going to be opened in Berland in the near future. The site administration wants to launch their project as soon as possible, that's why they ask you to help. You're suggested to implement the prototype of site registration system. The system should work on the following principle.

Each time a new user wants to register, he sends to the system a request with his name. If such a name does not exist in the system database, it is inserted into the database, and the user gets the response OK, confirming the successful registration. If the name already exists in the system database, the system makes up a new user name, sends it to the user as a prompt and also inserts the prompt into the database. The new name is formed by the following rule. Numbers, starting with 1, are appended one after another to name (name1, name2, ...), among these numbers the least $i$ is found so that namei does not yet exist in the database.

## Input

The first line contains number $n$ $(1 \leq n \leq 10^5)$. The following $n$ lines contain the requests to the system. Each request is a non-empty line, and consists of not more than 32 characters, which are all lowercase Latin letters.

## Output

Print $n$ lines, which are system responses to the requests: OK in case of successful registration, or a prompt with a new name, if the requested name is already taken.

## Examples

### Input

[Link to file](input-0.txt)

```
4
abacaba
acaba
abacaba
acab
```

### Output

[Link to file](expected-0.txt)

```
OK
OK
abacaba1
OK
```

### Input

[Link to file](input-1.txt)

```
6
first
first
second
second
third
third
```

### Output

[Link to file](expected-1.txt)

```
OK
first1
OK
second1
OK
third1
```

## Solutions

### Python 3.8.10

[Link to file](solution.py) (time: 1622 ms, memory: 1232 KB)

### GNU C++17 7.3.0

[Link to file](solution.cpp) (time: 216 ms, 900 KB)

### Rust 1.66.0

[Link to file](solution.rs) (time: 124 ms, memory: 1552 KB)
