#include <stdio.h>

#include "random_value.c"
#include "benchmark.c"

int main() {
    const int array_size = 5;

    // Insertion Sort implementation:
    int* array = random_value(0, 10, array_size);

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

    // Cleanup:
    free(array);
    return 0;
}
