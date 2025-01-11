
.PHONY: run test doc clean cov

run:
	cargo run --bin ui --features bsp/board-sim,hal/std,ui/std,ui/exit


build:
	cargo build --bin ui --features bsp/board-hactar12 --target=thumbv7em-none-eabihf


build-mgmt:
	cargo build --bin mgmt --features bsp/board-hactar12 --target=thumbv6m-none-eabi


test:
	cargo test --workspace --lib --tests --bin ui  --features bsp/board-sim,hal/std,ui/std,ui/exit  -- --test-threads=1
	cargo test --workspace --doc  --features bsp/board-sim,hal/std,ui/std,ui/exit  -- --test-threads=1


doc:
	cargo doc --workspace --features bsp/board-sim,hal/std,ui/std,ui/exit


clean:
	cargo clean


cov:
	cargo llvm-cov --workspace --lib --tests --bin app --features bsp/board-sim,hal/std,app/std,app/exit  -- --test-threads=1
