# Hartex-Rust

A safe and lightweight real-time Kernel written in Rust. The Kernel is developed for cortex-m3/m4 based microcontrollers. The goal of the project was to develop a memory efficient, safe and lightweight real-time Kernel. Rust-lang was choosen due to its powerful compile-time checks. All the subsystems have been developed and manually tested. Language features like Traits, Generics helped define powerful and safe types. The Kernel uses statically allocated data structures to keep itself simple and fast. But using feature flags, the developer can enable dynamic memory allocation (for end-application and not Kernel itself). Cargo feature flags are used to configure constants such as maximum tasks, resources, etc. 

This Project is the Implementation of my Bachelor's Thesis. For details regarding design and architecuture take a look at the thesis. The Kernel subsystem design has been inspired from the Hartex design specification.

## Features

* Due to the usage of boolean vectors, the kernel does not use and intensive data- structure like queue or list. 
* Scheduling, Software bus, and resource management is implemented by boolean vectors, which reduce the memory and performance overhead of the kernel. 
* Non-blocking Synchronisation and communication between tasks are achieved through boolean vector semaphores. 
* Event manager with lightweight event handlers: This helps keep the execution time of interrupts very low. 
* Resource management through Stack-based priority ceiling protocol: This guarantees not only mutually exclusive allocation of resources but also deadlock-free execution of tasks.

For examples, take a look at `/examples`.

[User Documentation](https://docs.rs/hartex-rust/)

[API Reference](http://autonomous-cyber-physical-systems.github.io/hartex-rust)

## References

Gourinath Banda. “Scalable Real-Time Kernel for Small Embedded Systems”. English. MSEngg Dissertation. Denmark: University of Southern Denmark, June 2003. URL: http://citeseerx.ist.psu.edu/viewdoc/download;jsessionid=84D11348847CDC13691DFAED09883FCB?doi=10.1.1.118.1909&rep=rep1&type=pdf.

## Future Work

Many language features like constant functions, constant generics, etc. are under heavy development. These features, once on reaching a stable state, can be used for further performance improvement of various kernel routines. These features could be used to evaluate most of the kernel configuration primitives at compile-time, which would boost the performance and reduce binary size. Rust supports conditional compilation. This can be used to support other machine architectures with minimal code duplication. Other features like implementation of networking stack would enable usage of hartex-rust in IoT projects. The Kernel has been designed to operate with very low interrupt latency, these claims can be benchmarked.

The Kernel has been designed and developed for single-core/processor systems. The future work on this project could include modifying the internals to work efficiently on multiprocessor systems. Another security feature that could be added is the protection of task stacks from the other tasks. This can be done with the help of Memory Protection Units (MPUs) provided by most Microcontrollers.

## License

This project is under the MIT license.
