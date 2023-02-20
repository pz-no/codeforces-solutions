#include <iostream>

void solution() {
    int num_count;
    std::cin >> num_count;

    int num, num_sum = 0;
    for (int num_index = 0; num_index < 3; ++num_index) {
        std::cin >> num;
        num_sum = (num_sum << 1 | (num % 2));
    }

    int num_index = -1;
    switch (num_sum) {
        case 4:
        case 3:
            num_index = 1;
            break;

        case 2:
        case 5:
            num_index = 2;
            break;

        case 1:
        case 6:
            num_index = 3;
            break;
    }

    int item = 0;
    if (num_index == -1) {
        item = (num_sum == 0) ? 1 : 0;
        for (int i = 3; i < num_count; ++i) {
            std::cin >> num;
            num_index = (item == (num % 2)) ? (i + 1) : num_index;
        }
    } else {
        for (int i = 3; i < num_count; ++i) {
            std::cin >> num;
        }
    }

    std::cout << num_index << '\n';
}


void setup() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);
}


int main() {
    setup();
    solution();
}
