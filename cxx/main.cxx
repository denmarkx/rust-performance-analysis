#include <iostream>
#include <ctime>

// No need to rewrite benchmark and util.c,
// but we'll just end up rewriting the sorting/math funcs.
// ..though it'd be interesting to see the under-the-hood comparison.
extern "C" {
    #include "benchmark.c"
    #include "util.c"
    #include "random_value.c"
}

// Algorithm -> Function "router"
typedef struct AlgoFuncHldr {
    char* string;
    void (*func)(int*, int);
} AlgoFuncHldr;

AlgoFuncHldr AFRouter [] = {
    // {"insertion", insertion_sort},
    // {"quick", start_quick_sort},
    // {"bubble", bubble_sort},
};


// ..we can technically maybe sort of perhaps do extern main (with slight changes)
// but that may end up blowing up, so this is the cxx version of c/main.c.
int main(int argc, char* argv[]) {
    Args args = parse_args(argc, argv);
    setup_benchmark(args.n_iter);
    srand(time(NULL));

    ArrayData arr_data;
    arr_data.arr_size = 0;

    // File check:
    if (args.file != NULL) {
        // arr_data = read_from_file
    } else {
        arr_data = random_value_set(
            args.r_min,
            args.r_max,
            args.n_iter,
            args.inner_length
        );
    }

    // algorithm router:
    int f_got_algorithm = 0;
    for (
        AlgoFuncHldr* router = AFRouter;
        router != AFRouter + sizeof(AFRouter) / sizeof(AFRouter[0]);
        router++
    ) {
        if (strcmp(router->string, args.algorithm) == 0) {
            // This assumes the parameter types of our func.
            printf("Algorithm: %s\n", args.algorithm);
            for (unsigned int i = 0; i < arr_data.arr_size; i++) {
                (*router->func)(arr_data.array[i], arr_data.count_arr[i]-1);
                f_got_algorithm  = 1;
            }
        }
    }


    if (!f_got_algorithm) {
        fprintf(stderr, "Invalid algorithm specified. Acceptable are:\n");
        fprintf(stderr, "  insertion\n");
        fprintf(stderr, "  bubble\n");
        fprintf(stderr, "  quick\n");
        exit(1);
    }

    // cleanup
    cleanup_array_data(arr_data);
    // complete_benchmark(args.algorithm);

    return 0;
}