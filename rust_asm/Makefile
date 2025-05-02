RUST_FLAGS = --C opt-level=3 --emit asm -C "llvm-args=-x86-asm-syntax=intel" -O -C debuginfo=0
CFG = null

out_of_bounds:
	rustc out_of_bounds.rs $(RUST_FLAGS) --cfg $(CFG)

all: out_of_bounds
