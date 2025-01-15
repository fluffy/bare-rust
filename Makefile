
.PHONY: run test doc clean cov

run:
	cargo run --bin ui --features bsp/board-sim,hal/stm32f405,hal/std,ui/std,ui/exit


build:
	cd crates/ui ; cargo build --bin ui --features bsp/board-hactar12,hal/stm32f405 --target=thumbv7em-none-eabihf --verbose


build-mgmt:
	cd crates/mgmt; cargo build --bin mgmt --features hal/stm32f072 --target=thumbv6m-none-eabi  --verbose


test:
	cargo test --workspace --lib --tests --bin ui  --features bsp/board-sim,hal/stm32f405,hal/std,ui/std,ui/exit  -- --test-threads=1
	cargo test --workspace --doc  --features bsp/board-sim,hal/stm32f405,hal/std,ui/std,ui/exit  -- --test-threads=1


doc:
	cargo doc --workspace --features bsp/board-sim,hal/stm32f405,hal/std,ui/std,ui/exit


clean:
	cargo clean


cov:
	cargo llvm-cov --workspace --lib --tests --bin ui --features bsp/board-sim,hal/stm32f405,hal/std,ui/std,ui/exit  -- --test-threads=1
