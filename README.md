# bare-rust

Bare rust project for STM32 

# Running on Board

Connect up with stlink to board. 

From in the main directory, start
```aiignore
openocd
```
and leave running. Then in another window do 
```aiignore
cargo run
```
this will bring you to gdb prompt where you can type "c" to continue.

# Running on Emulator

Build with:
```aiignore
cargo build --features board-qemu  --target=thumbv7em-none-eabihf 
```

Run with:

```aiignore
qemu-system-arm  -S -no-reboot -cpu cortex-m4  -machine netduinoplus2  -gdb tcp::3333  -nographic  -semihosting-config enable=on,target=native -kernel target/thumbv7em-none-eabihf/debug/app  --trace "memory_region_ops_*" 
```
The -S cause it to wait for the debugger to connect.  

Connect debugger with:

```aiignore
arm-none-eabi-gdb -q  target/thumbv7em-none-eabihf/debug/app --init-eval-command="target extended-remote localhost:3333"
```

# Running on the Simulator

```aiignore
cargo test --features board-sim
```

# Notes
