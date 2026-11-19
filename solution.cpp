#include <bits/stdc++.h>

using namespace std;

int main() {
	ios::sync_with_stdio(false); cin.tie(nullptr);
	string s;
	bool input_ended = false;
	while (cin >> s) {
		if (s.find("THISISSEPARATOR") != -1) {
			input_ended = true;
			continue;
		}

		if (!input_ended) {
			continue;
		}

		cout << s << '\n';
	}
	return 0;
}
