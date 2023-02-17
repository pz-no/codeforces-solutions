#include <iostream>
#include <vector>
#include <algorithm>

void solution() {
    int bar_count;
    std::cin >> bar_count;

    int bar;
    std::vector<int> bars;
    for (int bar_index = 0; bar_index < bar_count; ++bar_index) {
        std::cin >> bar;
        bars.push_back(bar);
    }

    std::sort(bars.begin(), bars.end());

    int length = -1, max_length = -1, max_value = -1, height = 1;

    std::vector<int>::const_iterator p, q;
    for (p = bars.begin(); p != bars.end(); ++p) {
        for (q = p + 1; q != bars.end(); ++q) {
            if (*p != *q) {
                ++height;
                break;
            }
        }

        length = q - p;
        if (length > max_length) {
            max_length = length;
            max_value = *p;
        }

        p = q - 1;
    }

    std::cout << max_length << ' ' << height << '\n';
}


void setup() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);
}


int main() {
    setup();
    solution();
}
