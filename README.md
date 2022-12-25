# ocelotter
An experiment to implement a simple JVM in Rust

## Design - Executive Summary 

The JVM is a fundamentally multithreaded piece of software, which means that special
care must be taken to handle the dynamic, mutable nature of how the JVM handles memory.

This iteration of the design tries to use a classkeeping thread to manage the klasses
and related metadata.

1. The Rust main thread (r_main) starts the klasskeeping thread (k_keep)
2. r_main starts the Java main thread (main or j_main) and passes it mechanisms to communicate with k_keep 
3. r_main joins on j_main and k_keep


Object operations (e.g. putfield) are inherently unsafe.