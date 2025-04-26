#include <stdio.h>
#include "../benchmark.c"

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
