#!/bin/bash
cargo objdump --lib --release -- -disassemble -no-show-raw-insn -print-imm-hex | less
