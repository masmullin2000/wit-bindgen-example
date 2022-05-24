.PHONY: build run clean guest guest2
build: guest guest2
	cargo build

run: build
	cargo run

clean:
	cargo clean
	rm Cargo.lock

guest:
	cd guest; cargo build --target=wasm32-wasi

guest2:
	cd guest2; cargo build --target=wasm32-wasi

.PHONY: release run-rel
release:
	cd guest; cargo build --target=wasm32-wasi --release; cd ..
	cd guest2; cargo build --target=wasm32-wasi --release; cd ..
	cargo build --release

run-rel: release
	cargo run --release

