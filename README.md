# itisascript

it is what it is. 

a compiler built with **Rust (SWC)** that turns TypeScript (with strict types like `i32`, `u64`) into **Clang**-ready C code.

### why?
- TS `number` is a lie (it's always f64).
- C headers are a pain.
- I like braces, but I hate `long long int`.

### usage
```bash
cargo run -- input.ts -o output.c