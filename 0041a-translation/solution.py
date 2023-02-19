def solution():
    s, t = input(), input()

    result = True
    if len(s) != len(t):
        result = False
    else:
        i, j = 0, len(t) - 1
        while i < len(s):
            if s[i] != t[j]:
                result = False
                break

            i += 1
            j -= 1

    answer = "YES" if result else "NO"
    print(answer)


if __name__ == "__main__":
    solution()
