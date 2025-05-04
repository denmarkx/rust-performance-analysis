"""
Combines several CSVs generated from rust/src/benchmark.rs and c/benchmark.c
to a single one to make the import process into Polars easier.
"""
from pathlib import Path
import csv, sys, polars, argparse

parser = argparse.ArgumentParser()
parser.add_argument("-a", "--algorithm", type=str, required=True)
parser.add_argument("-u", "--unsafe", action="store_true")
args = parser.parse_args()

DATA_TYPE = args.algorithm.lower()
WANT_UNSAFE_RUST = args.unsafe

SORTING_FILES = [
    "bubble",
    "insertion",
    "quick",
]

MATH_FILES = [
    "matrix",
]

if DATA_TYPE not in SORTING_FILES + MATH_FILES:
    print ("Invalid algoirthm. [bubble, insertion, quick, matrix]")
    sys.exit(1)

csv_file = open(f"{DATA_TYPE}-{sys.platform}.csv", "w+")
writer = csv.writer(csv_file, lineterminator="\n")

headers = []
if DATA_TYPE in SORTING_FILES:
    headers = ["CXX", "C", "RUST"]
    if WANT_UNSAFE_RUST:
        headers += ["RUST_RPTR", "RUST_OOB"]
    writer.writerow(headers)

CSV_DATA = {header: [] for header in headers}
MAX_LEN_ROW = 0

for file in Path("../").rglob("*.csv"):
    ext = file.parent.name
    if ext not in ["c", "cxx", "rust"]:
        continue

    # Unsafe Rust's CSV names are outputted as: <...>_<RPTR|OOB>_UNSAFE.csv.
    if file.name.startswith(DATA_TYPE):
        # TODO: unsafe rust files
        if file.name.endswith("UNSAFE.csv") and not WANT_UNSAFE_RUST:
            continue

        # associate the header based on the filename.
        if ext == "c":
            # no need to do this for C or CXX though.
            header = "C"
        elif ext in ("cxx", "cpp"):
            header = "CXX"
        else:
            # for rust, we are suffixed by RPTR and OOB if we end with unsafe.
            header = "RUST"
            if file.name.endswith("UNSAFE.csv"):
                header += "_" + file.name.split("_")[1]

        # Each file has the headers of TIME_NS, TIME_MS, TIME_S.
        with open(file, "r") as f:
            reader = csv.reader(f)

            # though we only really care about seconds.
            seconds = [float(row[-1]) for row in list(reader)[1:]]
            CSV_DATA[header] = seconds

            if len(seconds) > MAX_LEN_ROW:
                MAX_LEN_ROW = len(seconds)

# fix up data:
for header, values in CSV_DATA.items():
    offset = MAX_LEN_ROW - len(values)
    # polars will complain if the DF size is not all the same upon read_csv (??)
    if offset > 0:
        values += [0] * offset

# write by row thanks to the lovely csvwriter
csv_values = list(CSV_DATA.values())
for row_index in range(MAX_LEN_ROW):
    row = [col[row_index] for col in csv_values]
    writer.writerow(row)
csv_file.close()

polars.read_csv(f"{DATA_TYPE}-{sys.platform}.csv").write_parquet(f"{DATA_TYPE}-{sys.platform}.parquet")
