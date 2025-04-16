#include <stdio.h>

#include "random_value.c"
#include "benchmark.c"
#include "util.c"

void bubble_sort(int* array, const int array_size) {
    benchmark();
    for (int i = 0; i < array_size; i++) {
        for (int j = 0; j < array_size-1; j++) {
            if (array[j] > array[j+1]) {
                // Swap:
                int temp = array[j];
                array[j] = array[j+1];
                array[j+1] = temp;
            }
        }
    }
    end_benchmark();
}

int main(int argc, char* argv[]) {
    warn_arguments(argc, argv);
    setup_benchmark(atoi(argv[3]));

    const int array_size = atoi(argv[2]);

    int* array = random_value(0, atoi(argv[1]), array_size);

    // Bubble Sort implementation:
    for (int i = 0; i < atoi(argv[3]); i++) {
        bubble_sort(array, array_size);
    }

    complete_benchmark();

    // Cleanup:
    free(array);
    return 0;
}
