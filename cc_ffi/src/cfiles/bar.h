#pragma once
#include <vector>

extern "C" {
	int cbar_sum_vec(const int* v, std::size_t count);
}

int bar_sum_vec(const std::vector<int>& v);

