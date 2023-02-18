#include <iostream>
#include <string>
#include <vector>
#include <cstdlib>


void solution() {
    int case_count;
    std::cin >> case_count;

    std::string text;
    for (int i = 0; i < case_count; ++i) {
        std::cin >> text;

        // R23C55 -> BC23
        long pos = text.find('C');
        if ('R' == text[ 0 ] && isdigit(text[1]) && 1 < pos && pos < text.size()) {
            long row = atol(text.substr(1, pos - 1).c_str());
            long col = atol(text.substr(pos + 1).c_str());

            std::vector<char> new_text;
            while (col > 0) {
                if (col % 26 == 0) {
                    new_text.push_back('Z');
                    col = (col - 1) / 26;
                } else {
                    new_text.push_back('A' + (col % 26 - 1));
                    col /= 26;
                }
            }

            while (!new_text.empty()) {
                std::cout << *new_text.rbegin();
                new_text.pop_back();
            }

            std::cout << row << '\n';

        // BC23 -> R23C55
        } else {
            long pos = -1;
            for (pos = 0; pos < text.size(); ++pos) {
                if (isdigit(text[pos])) {
                    break;
                }
            }

            std::string row = text.substr(0, pos);
            std::string col = text.substr(pos);

            long c = 0;
            for (std::string::const_iterator p = row.begin(); p != row.end(); ++p) {
                c = c * 26 + (*p - 'A' + 1);
            }

            std::cout << 'R' << col << 'C' << c << '\n';
        }
    }
}


void setup() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);
}


int main() {
    setup();
    solution();
}
