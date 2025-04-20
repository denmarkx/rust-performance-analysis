#include <stdio.h>

#include "random_value.c"
#include "benchmark.c"

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
    setup_benchmark(atoi(argv[2]));

    // Insertion Sort implementation:
    int array_size;
    int* array = NULL;
    ArrayData arr_data;
    arr_data.arr_size = 0;

    if (use_random_values(argv[1])) {
        // array = random_value(0, atoi(argv[1]), array_size);
        arr_data = random_value_set(0, atoi(argv[1]), atoi(argv[2]));
    } else {
        arr_data = read_from_file("../test/insertion_sort.txt");
    }

    // Traverse:
    for (int i = 0; i < arr_data.arr_size; i++) {
        array_size = arr_data.count_arr[i]-1;
        array = arr_data.array[i];
        insertion_sort(array, array_size);
    }

    // Cleanup:
    cleanup_array_data(arr_data);
    complete_benchmark();
    return 0;
}
