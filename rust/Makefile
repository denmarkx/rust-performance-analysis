ALGORITHM=insertion
ITER=10
R_MIN=0
R_MAX=100
INNER_LENGTH=0
RUN_ENV=release

FLAGS=--algorithm insertion

# BUILD:
debug:
	cargo build

release:
	cargo build --release

# RUN RELEASE BUILDS ONLY:
# make safe
run:
	cargo run --$(RUN_ENV) -- --algorithm $(ALGORITHM) --n-iter $(ITER)\
		--r-min $(R_MIN) --r-max $(R_MAX) -l $(INNER_LENGTH)

# make run_unsafe
run_unsafe:
	cargo run --$(RUN_ENV) -- --algorithm $(ALGORITHM) --n-iter $(ITER)\
		--r-min $(R_MIN) --r-max $(R_MAX) -l $(INNER_LENGTH) -u

# make run_flags FLAGS="--algorithm matrix -l 5"
run_flags:
	cargo run --$(RUN_ENV) -- $(FLAGS)

# Windows uses rmdir, anything else'll use rm.
ifeq ($(OS),Windows_NT)
clean:
	rmdir /s /q target
else
clean:
	rm -r target
endif