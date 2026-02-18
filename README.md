

# Rust Userland Systems Sandbox

**Purpose:**
Learn ownership, memory, and invariants in userland *before* writing a kernel.

Rule:

> If I can’t safely implement and test it here, I don’t understand it enough for kernel space.

This repo teaches **how the computer actually works**, not just Rust.

# Wiki
Where most of the rust thoughts go
![Wiki](https://github.com/lucassimpson0213/sys-userland-kernel-rust/wiki)


## What I Do

Follow Brown Rust Book:
[https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html)

For each concept:

**read → implement → test → fix → repeat**

Also play *Turing Complete* for hardware intuition.

---

## Projects (Order Matters)

1. Memory map iterator
2. ELF header parser
3. Page table entry (bitflags)
4. Bump allocator
5. Slab allocator
6. Packet parser (Ethernet/IP)
7. Virtual file table (inode style)

If I can write these confidently → I’m ready for kernel subsystems.

---

## Testing Rule (Most Important Part)

Testing is the real skill I’m learning.

Kernel developers debug with **invariants**, not print statements.

Always keep a watcher running:

```
cargo install cargo-watch
cargo watch -x test
```

Workflow:

**save file → tests run → fix immediately**

Never continue while tests are red.

Run a single test:

```
cargo watch -x "test <name>"
```

---

## Commands I’ll Forget

Run all tests:

```
just test
```

Run one:

```
just t read_one
```

Auto-test on save:

```
just watch
```

Fast checks:

```
just check
just clippy
```

Deep debugging:

```
just miri
```

---

## What This Actually Teaches

* memory layout
* aliasing
* lifetimes
* binary parsing
* allocators
* resource tracking
