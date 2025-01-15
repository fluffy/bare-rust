target extended-remote localhost:3333
set confirm off

load

set print asm-demangle on

break Default_Handler
break app::startup::panic
#break app::startup::Reset_Handler


# break at start of program
#stepi

break app::my_main
#break app::main

