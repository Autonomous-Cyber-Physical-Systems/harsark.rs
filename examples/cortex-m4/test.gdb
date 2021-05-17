target extended-remote :3333

set print asm-demangle on

break DefaultHandler
break HardFault
break rust_begin_unwind

monitor arm semihosting enable

set logging overwrite on
set logging file log.txt
set logging on

set confirm off

load

# start the process but immediately halt the processor
# stepi

c
