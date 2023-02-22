#include <stdio.h>


void foo_none() {
	printf("foo none\n");
}


int foo_sum_ints(int i1, int i2) {

	printf("foo sum ints: %d, %d\n", i1, i2);
	return i1 + i2;
}

int foo_sum_vec(int* pv, size_t count) {

	int sum = 0;
	for (size_t i = 0; i < count; i++) {
		sum += pv[i];
	}
	return sum;
}

struct Point {
 	float x;
	float y;
};

void foo_print_points(struct Point* points, size_t count) {

	printf("count(%lu):%lu\n", sizeof(count), count);
	for (size_t i = 0; i < count; i++) {
		struct Point* point = &points[i];
		printf("%lu: x=%f y=%f\n", i, point->x, point->y);
	}
}

void foo_mult_points(struct Point* points, size_t count) {

	for (size_t i = 0; i < count; i++) {
		struct Point* point = &points[i];
		point->x *= 2;
		point->y *= 2;
	}
}


