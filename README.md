# memmap

A standalone Rust library for parsing and iterating physical memory maps, designed to be tested in userland and later reused inside an OS kernel.

---

## Why this exists

When developing a kernel, one of the first hard problems is **interpreting the bootloader’s memory map** and turning it into a usable physical frame allocator.

Unfortunately, debugging that logic inside the kernel is extremely slow:

* no debugger
* limited logging
* crashes often result in a reboot (triple fault)
* long build/boot cycles

This crate solves that by separating:

**hardware environment**
from
**memory management logic**

The goal is to write and fully test the algorithms in a normal Rust environment first, then plug them into the kernel with minimal changes.

The kernel becomes integration code, not the development environment.

---

## Design Philosophy

The library is written so it can run in two environments:

| Environment             | Features                                |
| ----------------------- | --------------------------------------- |
| Userland (`cargo test`) | Uses `std` for testing and debugging    |
| Kernel (`no_std`)       | Uses only `core` and runs inside the OS |

This is achieved with:

```rust
#![cfg_attr(not(test), no_std)]
```

So the same code:

* can be unit tested normally
* and later compiled into the kernel

---

## What the crate will do

The pipeline looks like this:

```
raw bootloader bytes
        ↓
RawEntry iterator
        ↓
validated memory regions
        ↓
usable memory regions
        ↓
page-aligned physical frames
        ↓
frame allocator
```

Eventually the crate will provide:

* Raw memory map parsing
* Region sanitization
* Alignment handling
* Reserved memory exclusion
* Frame iteration
* Frame allocation

---

## Project Structure

```
src/
 ├── lib.rs      - crate configuration
 ├── raw.rs      - raw entry parsing from byte buffers
 ├── region.rs   - validated memory region abstraction
 └── frames.rs   - page frame iterator and allocator
```

---

## Development Workflow

1. Simulate bootloader memory using byte buffers
2. Write iterators to parse entries
3. Validate with unit tests
4. Convert to `no_std`
5. Integrate into kernel

All logic is verified before the kernel ever boots.

Example test:

```bash
cargo test
```

This allows debugging with:

* assertions
* panic messages
* stack traces

instead of kernel reboots.

---

## Kernel Integration

Add this crate as a dependency in your kernel:

```toml
[dependencies]
memmap = { path = "../memmap" }
```

Then use it:

```rust
use memmap::raw::RawEntry;
```

At boot, the kernel will replace the simulated buffer with the real bootloader pointer:

```rust
let bytes = unsafe {
    core::slice::from_raw_parts(boot_ptr as *const u8, boot_len)
};
```

No other code changes should be necessary.

---

## Long-Term Goal

Provide a safe and testable foundation for:

* physical memory management
* page table setup
* heap allocator backing
* future virtual memory system

This crate is intended to eliminate logic bugs *before* they reach the kernel.

---

## Status

Early development — initial raw entry structures and test harness in progress.

---

## License

MIT or Apache-2.0 (to be decided)

