#ifndef BENCHMARK
#define BENCHMARK

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#include <winnt.h>

static LARGE_INTEGER f;
double get_time() {
    static int FREQ_INITIALIZED = 0;

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
double get_time() {
    static mach_timebase_info_data_t timebase;
    static int TIMEBASE_INITIALIZED = 0;

    if (!TIMEBASE_INITIALIZED) {
        mach_timebase_info(&timebase);
        TIMEBASE_INITIALIZED = 1;
    }

    uint64_t time = mach_absolute_time();
    // Convert to seconds
    return (double)time * timebase.numer / timebase.denom / 1e9;
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
        benchmark_totals = malloc(sizeof(double) * count);
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


/*
* Averages the benchmark totals and frees the array.
* Does nothing if benchmark_count is 0 or 1.
*/
static inline void complete_benchmark() {
    // This just means that we didnt provide additional argument when running the program.
    if (benchmark_count <= 1) {
        return;
    }

    double average = 0;
    for (int i = 0; i < benchmark_count; i++) {
        average += benchmark_totals[i];
    }
    average /= benchmark_count;
    printf("Average Time: %f\n", average);

    if (benchmark_totals != NULL) {
        free(benchmark_totals);
    }
}

#endif