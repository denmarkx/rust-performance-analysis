# A Comparative Evaluation Between Rust, C, and C++
Source code for the Operating Systems Graduate Research Project. This projects aims to **identify if Rust's runtime checks negatively affect performance** by using novel sorting algorithms. All code, datasets,
libraries, and visuals that were used in the final research paper are present.

## Directory Structure
- `benchmark_data/` contains all [Parquet](https://parquet.apache.org/) data files and final graph notebook.
- `c/`, `cxx/`, and `rust/` contain all the sorting and benchmarking code.
- `rust_asm/` features the select Assembly code generated for the paper and research purposes.

## Execution
Each directory except for the data contains a UNIX Makefile that was used to compile the programs. It contains all the optimization flags and library includes used. The programs use flag-based argument
parsing that is nearly identical across all three languages. Once executed, the results will write itself to a CSV.
___
### Program Arguments
When running the program via `./main` or `./rust_rp_main`, these are the arguments to expect:
- **`--algorithm` (Required)**
  - <_insertion, bubble, quick_>
- **`--n-iter`**: Number of custom arrays to generate.
  - <_unsigned int_> 
- **`--r-min`**: Lower bound of the range of randomly generated array values.
  - <_unsigned int_>
- **`--r-max`**: Upper bound of the range of randomly generated array values.
  - <_unsigned int_>
- **`--seed`**: Cross-language seed for the psuedo random number generator.
  - <_unsigned int_>

There exists additional arguments: `--file`, `--inner-length` which were used solely for the presentation. These are obsolete.
___
### Rust Argument Changes

Rust will accept all the arguments above along with extras:
- **`--algorithm` (Required)**
  - <_insertion, bubble, quick, sum_>
- **`--unsafe-type**: Runs a specified unsafe version of the algorithm.
  - <_oob_, _rptr>
  - <_vectorization_>: if the algorithm is `sum`.

Rust will accept arguments in the form of `--seed 5`. C and C++ require a `=` character separating the key and value: `--seed=5`.
___

### C and C++
**Requires** [libcsv](https://github.com/rgamble/libcsv):
  It will look for this  in `VCPKG_ROOT/installed/x64-windows/<include,lib>` or `/opt/homebrew/<include,lib>`. This was geared towards a local machine,
so the use of a proper pipeine process such as CMake seemed extraneous.

**Compile:** `make all`

By default C and C++ utilize the flags `-O3`, `-march=native`, and `-flto`.
___
### Rust and Rust_ASM
The code within `rust/` utilizes Cargo and contains the sorting algorithms and benchmarking. `rust_asm` does not.

**Compile:**
- **Rust**: `make release`
- **Rust_ASM**: `make all`

By default, Rust will compile with `opt-level=3`, `lto=true`, in release mode, and `target-cpu=native`.

## Psuedo-Random Value Generation
For some data within the research paper, it is stated that across all languages, the algorithm was given the exact same values. This is done by a trivial implementation of the
**[Linear Congruential Generator](https://www.columbia.edu/~ks20/4106-18-Fall/Simulation-LCG.pdf)**--an algorithm that returns a psuedo random number. This method can only
be activated when given the `--seed` parameter. Its generated values will be consistent across Rust, C, & C++.
