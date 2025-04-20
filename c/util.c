#include <stdlib.h>
#include <string.h>
#include <stdio.h>

void print_array(int* array, size_t size) {
    for (size_t i = 0; i < size; i++) {
        printf("%d ", array[i]);
    }
    printf("\n");
}

typedef struct ArrayData {
    int** array; // Contains [[...], [...]] where inner arrays are of arbitrary size.
    int* count_arr; // The length of each inner child array of above ptr array.
    int arr_size; // size of array and count_arr ptr.
} ArrayData;

ArrayData read_from_file(char* file_name) {
    FILE* file = fopen(file_name, "r");
    int** array, *child_array, *count_arr = NULL;
    char* n_token, *token;
    char line[256];

    // optimization or compiler screws this if its all in one line. zzz
    int line_count=0;
    int i = 0;
    int j = 0;

    // read line by line:
    if (file != NULL) {
        while (fgets(line, sizeof(line), file)) {
            line_count++;
        }
        array = malloc(line_count * sizeof(*array));
        count_arr = malloc(line_count * sizeof(int));
        rewind(file);

        while (fgets(line, sizeof(line), file)) {
            // Delimited by a space:
            token = strtok_s(line, " ", &n_token);

            // First token is the array size:
            child_array = malloc(atoi(token) * sizeof(int));

            // Then that gets put in our count_arr:
            count_arr[i] = atoi(token);

            // Reset j:
            j = 0;
            while (token != NULL) {
                token = strtok_s(NULL, " ", &n_token);
                
                // Subsequent are just items of the child array.
                child_array[j] = atoi(token);
                j++;
            }

            // Add array:
            array[i] = child_array;
            i++;
        }
        fclose(file);
    }
    else {
        fprintf(stderr, "Unable to open file\n");
    }

    // Place everything in here:
    ArrayData data;
    data.array = array;
    data.count_arr = count_arr;
    data.arr_size = line_count;

    return data;
}

void cleanup_array_data(ArrayData arr_data) {
    for (int i = 0; i < arr_data.arr_size; i++) {
        free(arr_data.array[i]);
    }
    free(arr_data.array);
    free(arr_data.count_arr);
}

/*
* Returns bool representing if -f was passed in argv[1].
*/
int use_random_values(char* argv) {
    if (argv == NULL) {
        return 1;
    }
    return strcmp(argv, "-f") != 0;
}

/*
* Currently only used for sorting programs.
* If no arguments are supplied, it'll write the 1st and 2nd index of argv in-place.
* Stops and warns if arguments are illegal.
*/
void warn_arguments(int argc, char* argv[]) {
    // One argument is fine, we'll just pick something on our own.
    if (argc == 1) {
        if (use_random_values(argv[1])) {
            argv[1] = "5"; // [0-5]
            argv[2] = "5"; // 5 iterations.
            argv[3] = "1"; // Only benchmarks once.
            return;
        }
        // use supplied ./prog -f
        return;
    }

    // Default benchmark count is once for none given.
    // For the sake of keeping the setup actually dedicated to most of benchmark.c
    // we let benchmark.c::setup_benchmark make the determination for us.
    if (argc == 2) {
        argv[3] = "1";
        return;
    }

    // Though the sorting programs will require a max random value and iteration count.
    if (argc < 3) {
        printf("Incorrect usage: ./program <random_value> <iteration_count>.\n");
        exit(1);
    }

    // Number of times we want to benchmark.
    if (argc > 4) {
        printf("Incorrect usage: ./program <random_value> <iteration_count> <benchmark_count>.\n");
        exit(1);
    }
}
