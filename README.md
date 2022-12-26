# ocelotter
An experiment to implement a simple JVM in Rust

## Design - Executive Summary 

The JVM is a fundamentally multithreaded piece of software, which means that special
care must be taken to handle the dynamic, mutable nature of how the JVM handles memory.

This iteration of the design tries to use a classkeeping thread to manage the klasses
and related metadata. Specifically, interpreter threads (both user and VM threads that
need to execute bytecodes) co mmunicate using a system of channels.

The core channel is: `(Sender<OtKlassComms>, Receiver<OtKlassComms>)` where `OtKlassComms`
is a struct containing a String (the requested class name) and an `Sender<OtKlass>`. This
could presumably also be implemented using an Either type or an enum.

Startup then looks like this:

1. The Rust main thread (r_main) starts the klasskeeping thread (k_keep) passing it both a receiver (rx)
   *and* a sender (tx) of `OtKlassComms`
2. k_keep holds onto rx to receive klass requests from other threads
3. k_keep starts an interpreter thread (k_interp) to execute `clinit` methods encountered during classloading, and
   passes it tx so k_interp can send requests back to k_keep
4. 
5. r_main starts the Java main thread (main or j_main) and passes it mechanisms to communicate with k_keep 
6. r_main joins on j_main and k_keep


Object operations (e.g. putfield) are inherently unsafe.