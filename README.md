# rust-microc
This is a WIP compiler for microc (a small C dialect). Written for learning purposes only.
## Dependencies
To run a compiled version of the compiler you'll need `gcc` and `nasm`.  
To use the project from sources you'll need the [Rust toolchain](https://rustup.rs/) and the above dependencies.

## Things to do
- Modify the code generation algorithm to use registers instead of the stack for arithimetic operations.
- int64 support.
- better FFI.
- strings, arrays and pointers.