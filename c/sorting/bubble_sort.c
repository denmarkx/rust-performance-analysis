#include <stdio.h>
#include "../benchmark.c"

void bubble_sort(int* array, const int array_size) {
    benchmark();
    for (size_t i = 0; i < array_size; i++) {
        for (size_t j = 0; j < array_size-1; j++) {
            if (array[j] > array[j+1]) {
                // Swap:
                swap(&array[j], &array[j+1]);
            }
        }
    }
    end_benchmark();
}
