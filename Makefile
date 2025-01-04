
.PHONY: run test doc clean cov

run:
	cargo run --features hal/board-sim,dev/std,app/std,app/exit


test:
	cargo test --workspace --lib --tests --bin app  --features hal/board-sim,dev/std,app/std
	cargo test --workspace --doc  --features hal/board-sim,dev/std,app/std


doc:
	cargo doc --workspace --features hal/board-sim,dev/std,app/std


clean:
	cargo clean


cov:
	cargo llvm-cov --workspace --lib --tests --bin app --features hal/board-sim,dev/std,app/std
