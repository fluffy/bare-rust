target extended-remote localhost:3333
set confirm off

load

stepi

set print asm-demangle on

break rust_begin_unwind
break Default_Handler
