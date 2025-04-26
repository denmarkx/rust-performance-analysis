#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#if defined(_WIN32) || defined(_WIN64)
// Windows: use strtok_s as-is
#define STRTOK(str, delim, context) strtok_s((str), (delim), (context))

// also Windows: getopt from MINGW/MSYS.
#include <getopt.h>

#else
// POSIX (macOS/Linux): use strtok_r
#define STRTOK(str, delim, context) strtok_r((str), (delim), (context))

// we'll get getopt from unistd.
#include <unistd.h>
#endif


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

// Similar to POSIX getline()
ssize_t read_line(char** lineptr, size_t* n, FILE* stream) {
    int c;
    size_t len = 0;

    // allocate for lineptr
    if (*lineptr == NULL || *n == 0) {
        *n = 128;  // Initial buffer size
        *lineptr = malloc(*n);
        if (*lineptr == NULL) return -1;
    }

    // read by character
    while ((c = fgetc(stream)) != EOF) {
        // quick buffer expansion. we'll actually double the size.
        if (len + 1 >= *n) {
            *n *= 2;
            char* temp = realloc(*lineptr, *n);
            if (!temp) {
                return -1;
            }
            *lineptr = temp;
        }

        // store and inc
        (*lineptr)[len++] = (char)c;

        // line end.
        if (c == '\n') {
            break;
        }
    }

    if (len == 0 && c == EOF) {
        return -1;
    }

    (*lineptr)[len] = '\0';
    return (ssize_t)len;
}

ArrayData read_from_file(char* file_name) {
    FILE* file = fopen(file_name, "r");
    int** array, *child_array, *count_arr = NULL;
    char* n_token, *token;
    char* line = NULL;
    size_t len = 0;

    // optimization or compiler screws this if its all in one line. zzz
    int line_count=0;
    int i = 0;
    int j = 0;

    // read line by line:
    if (file != NULL) {
        while (read_line(&line, &len, file) != -1) {
            line_count++;
        }
        array = malloc(line_count * sizeof(*array));
        count_arr = malloc(line_count * sizeof(int));
        rewind(file);

        while (read_line(&line, &len, file) != -1) {
            // Delimited by a space:
            token = STRTOK(line, " \n", &n_token);

            // First token is the array size:
            child_array = malloc(atoi(token) * sizeof(int));

            // Then that gets put in our count_arr:
            count_arr[i] = atoi(token);

            // Reset j:
            j = 0;
            while (token != NULL && j < count_arr[i]) {
                token = STRTOK(NULL, " \n", &n_token);
                
                // Subsequent are just items of the child array.
                child_array[j] = atoi(token);
                j++;
            }

            // Add array:
            array[i] = child_array;
            i++;
        }
        fclose(file);
        free(line);
    }
    else {
        fprintf(stderr, "Could not open file: %s.\n", file_name);
        exit(1);
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
* Argument Struct similar to rust/src/main.rs.
*/
typedef struct Args {
    char* file;
    unsigned int r_min;
    unsigned int r_max;
    unsigned int n_iter;
    char* algorithm;
    unsigned int inner_length;
} Args;


static struct option long_options[] = {
    {"file", optional_argument, NULL, 'f'},
    {"r_min", optional_argument, NULL, 'r'},
    {"r_max", optional_argument, NULL, 'm'},
    {"n_iter", optional_argument, NULL, 'i'},
    {"algorithm", required_argument, NULL, 'a'},
    {"inner_length", optional_argument, NULL, 'l'},
};

void print_usage() {
    printf("Acceptable Arguments:\n\n");
    printf("  --file [-f] <filename>\n");
    printf("     Input File. All other arguments will be voided except for algorithm.\n\n");
    printf("  --r-min [-r] <unsigned int>\n");
    printf("     The lower bound of the rand_range for each element in array.\n\n");
    printf("  --r-max [-m] <unsigned int>\n");
    printf("     The upper bound of the rand_range for each element in array.\n\n");
    printf("  --n-iter [-i] <unsigned int>\n");
    printf("     The number of times to benchmark.\n\n");
    printf("  --algorithm [-a] <algorithm>\n");
    printf("     Algorithms: {insertion, bubble, quick, matrix}\n\n");
    printf("  --inner-length [-l] <unsigned int>\n");
    printf("     The fixed length of all inner arrays. 0 will make it random.\n     Required for Matrix Multiplication.\n");
}

Args parse_args(int argc, char* argv[]) {
    Args args;
    int opt;

    if (argc <= 1) {
        print_usage();
    }

    // We'll set some defaults:
    args.file = NULL;
    args.r_min = 0;
    args.r_max = 10;
    args.n_iter = 10;
    args.algorithm = "insertion";
    args.inner_length = 0;

    // Option Router
    while ((opt = getopt_long(argc, argv, "r:m:i:l:a:f:", long_options, NULL)) != -1) {
        switch (opt) {
            case 'f':
                args.file = optarg;
                break;
            case 'r':
                args.r_min = atoi(optarg);
                break;
            case 'm':
                args.r_max = atoi(optarg);
                break;
            case 'i':
                args.n_iter = atoi(optarg);
                break;
            case 'a':
                args.algorithm = optarg;
                break;
            case 'l':
                args.inner_length = atoi(optarg);
                break;
            default:
                print_usage();
                break;
        }
    }

    // Let's clean things up and verify a few things.

    // r_min, r_max
    if (args.r_min < 0 || args.r_max <= 0) {
        fprintf(stderr, "--r_min [-r] cannot be less than 0 and --r_max [-m] cannot be less than or equal to 0!\n");
        exit(1);
    }

    // inner_length:
    if (args.inner_length < 0) {
        fprintf(stderr, "--inner_length [-l] cannot be less than zero!\n");
        exit(1);
    }

    // n_iter:
    if (args.n_iter <= 0) {
        fprintf(stderr, "--n_iter [-i] cannot be less than or equal to zero!\n");
        exit(1);
    }


    // algorithm and file is done by main.c
    // but we will lower the algorithm:
    args.algorithm = strlwr(args.algorithm);

    return args;
}

/*
* Currently only used for sorting programs.
* If no arguments are supplied, it'll write the 1st and 2nd index of argv in-place.
* Stops and warns if arguments are illegal.
*
*/
void warn_arguments(int argc, char* argv[]) {
    // One(argc==2) argument is ONLY allowed if we're doing this via file mode.
    if (argc == 2) {
        // Otherwise, it's not allowed.
        if (use_random_values(argv[1])) {
            printf("Usages:\n./program <random_value_max> <iteration_count>.\n./program -f\n");
            exit(1);        
        }
        return;
    }

    // Only other acceptable args is 3.
    if (argc != 3) {
        printf("Usages:\n./program <random_value_max> <iteration_count>.\n./program -f\n");
        exit(1);
        return;
    }
}
