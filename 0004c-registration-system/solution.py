from typing import Dict


def solution():
    name_count = int(input())

    name_map: Dict[str, int] = {}
    for _ in range(name_count):
        name = input()
        count = name_map.get(name, 0)
        if count == 0:
            print("OK")
        else:
            print(f"{name}{count}")

        name_map[name] = count + 1


if __name__ == "__main__":
    solution()
