#include <stdio.h>

#include "random_value.c"
#include "benchmark.c"

/*
* Given references to x and y, swaps them in-place.
*/
void swap(int *x, int *y) {
    int temp = *x;
    *x = *y;
    *y = temp;
}

/*
* Partitions the array between a pivot (array[high]) such
* such that values higher than the pivot are on the right of it.
* and values lower are on the left.
*
* Returns index representing pivot.
*/
int partition(int* array, int low, int high) {
    int pivot = array[high];
    int i = low-1;

    for (size_t j = low; j < high; j++) {
        // Check if we're less than pivot:
        // Swap left and right.
        if (array[j] <= pivot) {
            i++;
            swap(&array[i], &array[j]);
        }
    }

    // Then position the pivot.
    swap(&array[i+1], &array[high]);
    return i+1;
}

/*
* Recursively splits the array until array becomes too small to partition.
*/
void sort(int* array, int low, int high) {
    if (low >= high) {
        return;
    }
    int pivot = partition(array, low, high);
    sort(array, low, pivot-1);
    sort(array, pivot+1, high);
}

void start_quick_sort(int* array, const size_t array_size) {
    benchmark();
    sort(array, 0, array_size-1);
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
        arr_data = read_from_file("../test/quick_sort.txt");
    }

    // Traverse:
    for (int i = 0; i < arr_data.arr_size; i++) {
        array_size = arr_data.count_arr[i]-1;
        array = arr_data.array[i];
        start_quick_sort(array, array_size);
    }

    // Cleanup:
    cleanup_array_data(arr_data);
    complete_benchmark();
    return 0;
}

