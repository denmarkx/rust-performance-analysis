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
            token = strtok_s(line, " \n", &n_token);

            // First token is the array size:
            child_array = malloc(atoi(token) * sizeof(int));

            // Then that gets put in our count_arr:
            count_arr[i] = atoi(token);

            // Reset j:
            j = 0;
            while (token != NULL && j < count_arr[i]) {
                token = strtok_s(NULL, " \n", &n_token);
                
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
