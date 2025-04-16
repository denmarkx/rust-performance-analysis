#include <stdio.h>

#include "random_value.c"
#include "benchmark.c"
#include "util.c"

void insertion_sort(int* array, int array_size) {
    // Start at the 2nd item (i=0)
    benchmark();
    for (size_t i = 1; i < array_size; i++) {
        int n = array[i];
        int j = i-1;

        // Continuously check previous until first.
        while (j >= 0 && array[j] > n) {
            array[j+1] = array[j];
            j--;
        }
        // Insert ahead:
        array[j+1] = n;
    }
    end_benchmark();
}

int main(int argc, char* argv[]) {
    warn_arguments(argc, argv);
    setup_benchmark(atoi(argv[3]));

    const int array_size = atoi(argv[2]);

    // Insertion Sort implementation:
    int* array = random_value(0, atoi(argv[1]), array_size);

    for (int i = 0; i < atoi(argv[3]); i++) {
        insertion_sort(array, array_size);
    }

    // Cleanup:
    free(array);
    complete_benchmark();
    return 0;
}
