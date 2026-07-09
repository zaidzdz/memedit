# osxmem

[![crates.io](https://img.shields.io/crates/v/osxmem.svg)](https://crates.io/crates/osxmem)
[![docs.rs](https://docs.rs/osxmem/badge.svg)](https://docs.rs/osxmem)
[![license](https://img.shields.io/crates/l/osxmem.svg)](LICENSE)

Read and write process memory on macOS using the Mach kernel APIs.

## Requirements

- macOS only
- Must be ran with root

## Usage

```rust
use osxmem::process::Process;
use osxmem::memory::{read_mem, write_mem};

// attach by name or PID
let process = Process::open_by_name("Game").unwrap();
// or: Process::open(1234).unwrap();

let address: usize = 0x7060E5800;

let health: f32 = read_mem::<f32>(&process, address).unwrap();
println!("health: {}", health);

write_mem::<f32>(&process, address, 200.0).unwrap();
```

`read_mem` and `write_mem` are generic over any `Copy` type, so `u8`, `i32`, `f64`, structs, etc. all work the same way.

## License

MIT