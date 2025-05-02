#include <iostream>
#include <ctime>
#include <vector>

// No need to rewrite benchmark and util.c,
// but we'll just end up rewriting the sorting/math funcs.
// ..though it'd be interesting to see the under-the-hood comparison.
extern "C" {
    #include "benchmark.c"
    #include "util.c"
    #include "random_value.c"
}

#include "sorting/insertion.cxx"

// Algorithm -> Function "router"
typedef struct AlgoFuncHldr {
    const char* string;
    void (*func)(std::vector<int>);
} AlgoFuncHldr;

AlgoFuncHldr AFRouter [] = {
    {"insertion", insertion_sort},
    // {"quick", start_quick_sort},
    // {"bubble", bubble_sort},
};

// Minimized version of c/util::ArrayData
// ..I suppose this doesn't really need to be in a struct atp.
typedef struct VectorData {
    std::vector<std::vector<int>> array;
} VectorData;

/*
* Converts dynamic C-style arrays to vector.
* See the comment in the main function as to
* the reason why I'm doing this.
*/
VectorData convert_to_vector(ArrayData arr_data) {
    // int** array;
    // int* count_arr;
    // int arr_size.
    VectorData vec_data;
    std::vector<std::vector<int>> prim_vec;
    for (int i = 0; i < arr_data.arr_size; i++) {
        std::vector<int> vec;
        for (int j = 0; j < arr_data.count_arr[i]; j++) {
            vec.push_back(arr_data.array[i][j]);
        }
        prim_vec.push_back(vec);
    }
    vec_data.array = prim_vec;
    return vec_data;
}


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

    // While you can (and for embedded systems, probably should) use malloc
    // std::vector is still a highlight of c++ so we're using it here.
    // obviously there's overhead in that, but it's mainly for a comparison
    // of c++ features vs. rust.
    VectorData vec_data = convert_to_vector(arr_data);
    cleanup_array_data(arr_data);

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
            for (unsigned int i = 0; i < vec_data.array.size(); i++) {
                (*router->func)(vec_data.array[i]);
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
    complete_benchmark(args.algorithm);

    return 0;
}