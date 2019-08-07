# CORTEXM_THREADS

A simple library for context-switching on ARM Cortex-M ( 0, 0+, 3, 4, 4F ) micro-processors

Supports pre-emptive, priority based switching

This project is meant for learning and should be used only at the user's risk. For practical and mature
rust alternatives, see [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)


## Current State
Processor support:

 - [x] Cortex-M0
 - [x] Cortex-M0+
 - [x] Cortex-M3
 - [ ] Cortex-M4
 - [ ] Cortex-M4F

Features:
 - [x] Preemptive, priority-based switching
 - [x] Efficient sleep
 - [ ] Accept stack memory area as a vec (arrayvec?, smallvec?) instead of &[]
 - [ ] Non-privileged mode
 - [ ] Mutex implementation aware of thread scheduling


## Examples
The `example_crates` folder contains crates showing how to 
use cortexm-threads for different boards.

Available examples:
 - [stm32f3](./example_crates/stm32f3) - 2 threads with one 
 thread running an LED roulette, and the other periodically
 printing magnetometer readings. Currently compiles for target
 thumbv7m-none-eabi instead of thumbv7em-none-eabihf. See Roadmap#1
 - [microbit](./example_crates/microbit) - 2 threads printing
 messages with co-operative context switching
 - [qemu-m4](./example_crates/qemu-m4) - (set up to run
 on qemu) 2 threads printing messages via semi-hosting.
 Run `cargo run` from `example_crates/qemu-m4` directory
 to see it running. You must have qemu-system-arm on the system PATH.

Sample:
```rust
#![no_std]
#![no_main]
extern crate panic_semihosting;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};
use cortexm_threads::{init, create_thread, create_thread_with_config, sleep};

#[entry]
fn main() -> ! {
	let cp = cortex_m::Peripherals::take().unwrap();
	let mut syst = cp.SYST;
	syst.set_clock_source(SystClkSource::Core);
	syst.set_reload(80_000);
	syst.enable_counter();
	syst.enable_interrupt();
	let mut stack1 = [0xDEADBEEF; 512];
	let mut stack2 = [0xDEADBEEF; 512];
	let _ = create_thread(
		&mut stack1, 
		|| {
			loop {
				let _ = hprintln!("in task 1 !!");
				sleep(50); // sleep for 50 ticks
			}
		});
	let _ = create_thread_with_config(
		&mut stack2, 
		|| {
			loop {
				let _ = hprintln!("in task 2 !!");
				sleep(30); // sleep for 30 ticks
			}
		},
		0x01, // priority, higher numeric value means higher priority
		true  // privileged thread
		);
    init();
}
```

# License
See [LICENSE.md](LICENSE.md)
