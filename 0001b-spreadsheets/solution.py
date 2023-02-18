from typing import List


def solution():
    case_count = int(input())

    for _ in range(case_count):
        text = input()

        # R23C55 -> BC23
        pos = text.find("C")
        if text[0] == "R" and text[1].isdigit() and 1 < pos < len(text):
            row = int(text[1:pos])
            col = int(text[pos + 1:])

            text_list: List[str] = []
            while col > 0:
                if col % 26 == 0:
                    text_list.append("Z")
                    col = (col - 1) // 26
                else:
                    text_list.append(chr(ord("A") + col % 26 - 1))
                    col //= 26

            for c in reversed(text_list):
                print(c, end="")
            print(row)

        # BC23 -> R23C55
        else:
            for pos, c in enumerate(text):
                if c.isdigit():
                    break

            row, col = text[:pos], text[pos:]

            col_sum = 0
            for c in row:
                col_sum = col_sum * 26 + ord(c) - ord("A") + 1

            print(f"R{col}C{col_sum}")


if __name__ == "__main__":
    solution()
