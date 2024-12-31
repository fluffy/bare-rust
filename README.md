# bare-rust

Bare rust project for STM32 

# Running on Board

Connect up stlink to board. 

From in the main directory, start
```aiignore
openocd --file openocd.cfg 
```
and leave running. Then in another window do 
```aiignore
cargo run --features app/board-blinkA,hal/board-blinkA,dev/board-blinkA  --target=thumbv7em-none-eabihf
```
this will bring you to gdb prompt where you can type "c" to continue.

In a separate window, can monitor USB console with
```aiignore
screen /dev/tty.usbserial<SOOMETHING> 115200
```
To exit screen, type ^A^\ 


# Running on Emulator

Build with:
```aiignore
TBD broken
cargo build --no-default-features  --features board-qemu,exit  --target=thumbv7em-none-eabihf 
```

Run with:

```aiignore
TBD boken 
qemu-system-arm  -S -no-reboot -cpu cortex-m4  -machine netduinoplus2  -gdb tcp::3333  -nographic  -semihosting-config enable=on,target=native -kernel target/thumbv7em-none-eabihf/debug/app  --trace "memory_region_ops_*" 
```
The -S cause it to wait for the debugger to connect.  

Connect debugger with:

```aiignore
arm-none-eabi-gdb -q  target/thumbv7em-none-eabihf/debug/app --init-eval-command="target extended-remote localhost:3333"
```

# Running on the Simulator

```aiignore
cargo run  --features dev/board-sim,hal/board-sim,app/std,app/exit
```

# Notes

Useful things to look at size of binary:
```aiignore
arm-none-eabi-size target/thumbv7em-none-eabihf/debug/app
cargo size --release -- -A
cargo nm --no-default-features --features board-blinkA  --target=thumbv7em-none-eabihf  --release -- -a -n
```
