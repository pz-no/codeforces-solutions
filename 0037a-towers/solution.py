def solution():
    bar_count = int(input())
    bars = sorted(int(x) for x in input().split())

    length, max_length, height, i, j = -1, -1, 1, 0, 0
    while i < bar_count:
        j = i + 1
        while j < bar_count:
            if bars[i] != bars[j]:
                height += 1
                break

            j += 1

        length = j - i
        if length > max_length:
            max_length = length

        i = j

    print(max_length, height)


if __name__ == '__main__':
    solution()