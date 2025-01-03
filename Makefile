
.PHONY: run-sim test-sim doc clean run-cov 

run-sim:
	cargo run --features hal/board-sim,dev/std,app/std,app/exit


test-sim:
	cargo test --workspace --features hal/board-sim,dev/std,app/std


doc:
	cargo doc --workspace --features hal/board-sim,dev/std,app/std


clean:
	cargo clean


cov:
	cargo llvm-cov --workspace --lib --tests --bin app --features hal/board-sim,dev/std,app/std
