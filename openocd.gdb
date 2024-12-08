target extended-remote localhost:3333
set confirm off

load

# break at start of program
# stepi
break main

set print asm-demangle on

#break rust_begin_unwind
break Default_Handler
