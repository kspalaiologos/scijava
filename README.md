# scijava - Fast scientific computing for Java

SciJava is a library that wraps the following libraries and provides convenient Java interfaces:
- GMP 6.2.1
- MPFR 4.1.1
- MPC 1.2.1
- FLINT 2.8.5
- Arb 2.23.0

The following platforms are supported:
- x86_64 Linux
- aarch64 Linux
- s390x Linux
- i686 Linux
- armhf Linux
- MIPS64 Linux
- RISC-V64 Linux
- PPC64 Linux
- PPC64le Linux
- x86_64 Darwin
- x86_64 FreeBSD
- x86_64 NetBSD
- x86_64 Windows
- i686 Windows

## FAQ
- Your library does not support $MY_ARCHITECTURE.

If the GCC crosscompiler for the architecture you run is available in Ubuntu/Debian repositories and supported by Rust, feel free to open a ticket.

- I use musl.

My condolences.
