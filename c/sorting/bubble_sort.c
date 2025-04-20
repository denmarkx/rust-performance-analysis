#include <stdio.h>

#include "random_value.c"
#include "benchmark.c"

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
    setup_benchmark(atoi(argv[2]));

    // Insertion Sort implementation:
    int array_size;
    int* array = NULL;
    ArrayData arr_data;
    arr_data.arr_size = 0;

    if (use_random_values(argv[1])) {
        // array = random_value(0, atoi(argv[1]), array_size);
        arr_data = random_value_set(0, atoi(argv[1]), atoi(argv[2]), 1);
    } else {
        arr_data = read_from_file("../test/bubble_sort.txt");
    }

    // Traverse:
    for (int i = 0; i < arr_data.arr_size; i++) {
        array_size = arr_data.count_arr[i]-1;
        array = arr_data.array[i];
        bubble_sort(array, array_size);
    }

    // Cleanup:
    cleanup_array_data(arr_data);
    complete_benchmark();
    return 0;
}
