
.PHONY: run test doc clean cov

run:
	cargo run --features bsp/board-sim,hal/std,app/std,app/exit


build:
	cargo build --features bsp/board-hactar12 --target=thumbv7em-none-eabihf


test:
	cargo test --workspace --lib --tests --bin app  --features bsp/board-sim,hal/std,app/std,app/exit  -- --test-threads=1
	cargo test --workspace --doc  --features bsp/board-sim,hal/std,app/std,app/exit  -- --test-threads=1


doc:
	cargo doc --workspace --features bsp/board-sim,hal/std,app/std,app/exit


clean:
	cargo clean


cov:
	cargo llvm-cov --workspace --lib --tests --bin app --features bsp/board-sim,hal/std,app/std,app/exit  -- --test-threads=1
