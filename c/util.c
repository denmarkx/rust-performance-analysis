#include <stdlib.h>

void print_array(int* array, size_t size) {
    for (size_t i = 0; i < size; i++) {
        printf("%d ", array[i]);
    }
    printf("\n");
}

/*
* Currently only used for sorting programs.
* If no arguments are supplied, it'll write the 1st and 2nd index of argv in-place.
* Stops and warns if arguments are illegal.
*/
void warn_arguments(int argc, char* argv[]) {
    // One argument is fine, we'll just pick something on our own.
    if (argc == 1) {
        argv[1] = "5"; // [0-5]
        argv[2] = "5"; // 5 iterations.
        argv[3] = "1"; // Only benchmarks once.
        return;
    }

    // Default benchmark count is once for none given.
    // For the sake of keeping the setup actually dedicated to most of benchmark.c
    // we let benchmark.c::setup_benchmark make the determination for us.
    if (argc == 2) {
        argv[3] = "1";
        return;
    }

    // Though the sorting programs will require a max random value and iteration count.
    if (argc < 3) {
        printf("Incorrect usage: ./program <random_value> <iteration_count>.\n");
        exit(1);
    }

    // Number of times we want to benchmark.
    if (argc > 4) {
        printf("Incorrect usage: ./program <random_value> <iteration_count> <benchmark_count>.\n");
        exit(1);
    }
}
