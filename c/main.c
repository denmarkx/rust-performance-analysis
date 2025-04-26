#include <stdio.h>
#include <stdlib.h>

#include "util.c"
#include "random_value.c"

// Algorithms
#include "sorting/insertion_sort.c"
#include "sorting/quick_sort.c"
#include "sorting/bubble_sort.c"

// Algorithm -> Function "router"
typedef struct AlgoFuncHldr {
    char* string;
    void (*func)(int*, int);
} AlgoFuncHldr;

AlgoFuncHldr AFRouter [] = {
    {"insertion", insertion_sort},
    {"quick", start_quick_sort},
    {"bubble", bubble_sort},
};

int main(int argc, char** argv) {
    Args args = parse_args(argc, argv);
    setup_benchmark(args.n_iter);
    srand(time(NULL));

    // We'll always have our data kept in here.
    ArrayData arr_data;
    arr_data.arr_size = 0;

    // Check if we're given a file.
    if (args.file != NULL) {
        arr_data = read_from_file(args.file);
    } else {
        // Otherwise, we're using random values.
        arr_data = random_value_set(
            args.r_min,
            args.r_max,
            args.n_iter,
            args.inner_length
        );
    }

    // Now, a router for our algorithm.
    for (
        AlgoFuncHldr* router = AFRouter;
        router != AFRouter + sizeof(AFRouter) / sizeof(AFRouter[0]);
        router++
    ) {
        if (strcmp(router->string, args.algorithm) == 0) {
            // This assumes the parameter types of our func.
            for (unsigned int i = 0; i < arr_data.arr_size; i++) {
                (*router->func)(arr_data.array[i], arr_data.count_arr[i]-1);
            }
        }
    }

    // and some cleanup
    cleanup_array_data(arr_data);
    complete_benchmark();
    return 0;
}
