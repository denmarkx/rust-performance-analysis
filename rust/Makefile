debug:
	cargo build
	cargo run

release:
	cargo build --release
	cargo run --release

# Windows uses rmdir, anything else'll use rm.
ifeq ($(OS),Windows_NT)
clean:
	rmdir /s /q target
else
clean:
	rm -r target
endif