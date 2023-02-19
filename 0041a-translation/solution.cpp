#include <iostream>
#include <string>

void solution() {
    std::string s, t;
    std::cin >> s >> t;

    bool result = true;
    if (s.size() != t.size()) {
        result = false;
    } else {
        std::string::const_iterator p = s.begin();
        std::string::const_reverse_iterator q = t.rbegin();
        while (p != s.end()) {
            if (*p != *q) {
                result = false;
                break;
            }

            ++p;
            ++q;
        }
    }

    std::cout << (result ? "YES" : "NO") << '\n';
}


void setup() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);
}


int main() {
    setup();
    solution();
}
