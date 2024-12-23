# bare-rust

Bare rust project for STM32 

# Running

Connect up with stlink to board. 

From in the main directory, start
```aiignore
openocd
```
and leave running. Then in another window do 
```aiignore
cargo run
```
this will bring you to gd prompt where you can type "c" to continue.


# Notes

```aiignore
openocd &
```

```aiignore
cargo run --features board-blinkA --target=thumbv7em-none-eabihf --release
```

```aiignore
cargo test --features board-sim
```

