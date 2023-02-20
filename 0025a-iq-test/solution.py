def solution():
    _ = int(input())
    counters, indices = [0, 0], [0, 0]

    numbers = [int(x) for x in input().split()]
    for num_index, number in enumerate(numbers):
        index = number % 2
        counters[index] += 1
        if counters[index] == 1:
            indices[index] = num_index + 1

        if counters[index] > 1 and counters[1 - index] == 1:
            print(indices[1 - index])
            break

        if counters[index] == 1 and counters[1 - index] > 1:
            print(indices[index])
            break


if __name__ == "__main__":
    solution()
