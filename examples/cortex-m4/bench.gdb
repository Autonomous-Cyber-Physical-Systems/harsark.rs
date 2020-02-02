target remote :3333

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
c
x 0xe0001004
c
x 0xe0001004
quit