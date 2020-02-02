#!/usr/bin/sh

cargo build --example bench --release && 
arm-none-eabi-gdb -x bench.gdb target/thumbv7em-none-eabi/release/examples/bench;

values=`cat log.txt | grep "0xe0001004"`;
time1=`echo $values | cut -d' ' -f2`;
time2=`echo $values | cut -d' ' -f4`;
echo $(($time2-$time1));