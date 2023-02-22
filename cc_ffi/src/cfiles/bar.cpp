#include "bar.h"


int cbar_sum_vec(const int* v, std::size_t count) {

	std::vector<int> vec;
	vec.assign(v, v + count);
	auto res = bar_sum_vec(vec);
	return res;
}


int bar_sum_vec(const std::vector<int>& v) {

	auto sum = 0;
	for (auto el : v) {
		sum += el;
	}
	return sum;
}

