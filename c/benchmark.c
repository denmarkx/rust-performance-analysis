#ifndef BENCHMARK
#define BENCHMARK

#ifdef USE_CSV
    #include <csv.h>
    #include <string.h>
    #include <stdlib.h>
    #include <stdio.h>
#endif

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#include <winnt.h>

static int FREQ_INITIALIZED = 0;
static LARGE_INTEGER f;

static inline double get_time() {
    // cache the frequency
    if (!FREQ_INITIALIZED) {
        QueryPerformanceFrequency(&f);
        FREQ_INITIALIZED = 1;
    }

    LARGE_INTEGER t;
    QueryPerformanceCounter(&t);
    return (double)t.QuadPart / f.QuadPart;
}
#elif defined(__APPLE__)
#include <mach/mach_time.h>
static inline double get_time() {
    static mach_timebase_info_data_t timebase;
    static int TIMEBASE_INITIALIZED = 0;

    if (!TIMEBASE_INITIALIZED) {
        mach_timebase_info(&timebase);
        TIMEBASE_INITIALIZED = 1;
    }

    uint64_t time = mach_absolute_time();
    // Convert to seconds
    return (double)time * timebase.numer / timebase.denom;
}
#endif

static double start_time;
static double end_time;
static int benchmark_count = 0;
static double *benchmark_totals = NULL;

/*
* Sets up a dynamic array for benchmark totals.
* Since everything is individual programs by themselves,
* we are perfectly fine by making it static just so
* we don't have to carry it around everywhere.
*
* Does not setup an array if count is 0.
*/
static inline void setup_benchmark(int count) {
    if (count > 0) {
        // no implicit convesion in cxx
        benchmark_totals = (double*)malloc(sizeof(double) * count);
    }
}

/*
* Starts benchmarking timer.
*/
static inline void benchmark() {
    start_time = get_time();
}

/*
* Stops benchmark and spits out result.
*/
static inline void end_benchmark() {
    end_time = get_time();
    double elapsed = (end_time - start_time);
    benchmark_count++;
    printf("[%d] Time Elapsed: %f\n", benchmark_count, elapsed);

    // Add the elapsed time to the benchmark totals only if its initialized.
    if (benchmark_totals != NULL) {
        benchmark_totals[benchmark_count-1] = elapsed;
    }
}

#ifdef USE_CSV
    /*
    * Writes header row to CSV.
    */
    void write_csv_headers(FILE* f) {
        const char* headers[3] = {"TIME_NS", "TIME_MS", "TIME_S"};
        for (int i = 0; i < 3; i++) {
            const char* header = headers[i];
            csv_fwrite(f, header, strlen(header));
            fputc(',', f);
        }
        fputc('\n', f);
    }


    /*
    * Writes non-header row to CSV.
    */
    void write_csv(FILE* f, double value) {
        char str[15];

        // NS:
        sprintf(str, "%f", value * 1e+9);
        csv_fwrite(f, str, strlen(str));
        fputc(',', f);

        // MS:
        sprintf(str, "%f", value * 1000);
        csv_fwrite(f, str, strlen(str));
        fputc(',', f);

        // S:
        sprintf(str, "%f", value);
        csv_fwrite(f, str, strlen(str));
        fputc('\n', f);
    }
#endif


/*
* Averages the benchmark totals and frees the array.
* Does nothing if benchmark_count is 0 or 1.
*/
static inline void complete_benchmark(const char* algorithm) {
    // This just means that we didnt provide additional argument when running the program.
    if (benchmark_count <= 1) {
        return;
    }

    #ifdef USE_CSV
        struct csv_parser p;
        char file_name[25];
        strcpy(file_name, algorithm);
        strcat(file_name, ".csv");
        FILE* fp = fopen(file_name, "wb+");
        csv_init(&p, 0);
        write_csv_headers(fp);
    #endif

    double average = 0;
    for (int i = 0; i < benchmark_count; i++) {
        average += benchmark_totals[i];

        // Write to CSV:
        #ifdef USE_CSV
            write_csv(fp, benchmark_totals[i]);
        #endif
    }
    average /= benchmark_count;
    printf("Average Time: %f\n", average);

    if (benchmark_totals != NULL) {
        free(benchmark_totals);

        // Cleanup file:
        #ifdef USE_CSV
            fclose(fp);
            csv_free(&p);
        #endif
    }
}

#endif
