#include <iostream>
#include <string>
#include <map>


int solution() {
	int name_count;
	std::cin >> name_count;

	std::map<std::string, int> name_map;

	std::string name;
	for (int name_index = 0; name_index < name_count; ++name_index) {
		std::cin >> name;

		std::map<std::string, int>::iterator name_iterator = name_map.find(name);
		if (name_iterator == name_map.end()) {
			name_map.insert(make_pair(name, 1));
			std::cout << "OK\n";
		} else {
			std::cout << name << (name_iterator->second)++ << '\n';
		}
	}

	return 0;
}


void setup() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);
}


int main() {
    setup();
    solution();
}
