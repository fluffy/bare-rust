# bare-rust

Bare rust project for STM32

Read the [Architecture.md](Architecture.md) document for more
information.

# Running on Board

Connect up the stlink to board.

From in the main directory, start

```sh
openocd --file openocd.cfg 
```

and leave running. Then in another window do

```sh
cargo run --features bsp/board-hactar12 --target=thumbv7em-none-eabihf
or 
cargo run --features bsp/board-blinkA --target=thumbv7em-none-eabihf
```

this will bring you to gdb prompt where you can type "c" to continue.

In a separate window, on a mac, can monitor USB console with

```sh
screen /dev/tty.usbserial<SOOMETHING> 115200
```

To exit screen, type ^A^\

Build doc with:

```sh
cargo doc --workspace --features bsp/board-hactar12 --target=thumbv7em-none-eabihf --open
```

# Running on Emulator

Does not work yet.

```sh
TBD broken
cargo build --features bsp/board-qemu,exit --target=thumbv7em-none-eabihf 
qemu-system-arm  -S -no-reboot -cpu cortex-m4  -machine netduinoplus2  -gdb tcp::3333  -nographic  -semihosting-config enable=on,target=native -kernel target/thumbv7em-none-eabihf/debug/app  --trace "memory_region_ops_*" 
arm-none-eabi-gdb -q  target/thumbv7em-none-eabihf/debug/app --init-eval-command="target extended-remote localhost:3333"
```

The -S cause the above to wait for the debugger to connect.

# Running on the Simulator

```aiignore
cargo run --features bsp/board-sim,hal/std,app/std,app/exit 
cargo test --workspace --features bsp/board-sim,hal/std,app/std,app/exit  -- --test-threads=1 
cargo doc  --workspace --features bsp/board-sim,hal/std,app/std,
app/exit  
```

# Notes

Useful things to look at size of binary:

```aiignore
cargo build  --features bsp/board-blinkA  --target=thumbv7em-none-eabihf  --release
arm-none-eabi-size target/thumbv7em-none-eabihf/release/app
cargo nm --no-default-features --features bsp/board-blinkA  --target=thumbv7em-none-eabihf  --release -- -a -n
```
