// mb1_memmap_workbook.rs
//
// You are NOT writing a parser.
// You are writing the first piece of code in your kernel that interprets
// memory created by something that is NOT your program (the bootloader).
//
// The kernel rule:
//    Every byte you did not create yourself is hostile.
//
// This file teaches you how an OS safely reads hardware/firmware tables.
//
// Your final pipeline:
//
// &[u8]  ---> RawEntry  ---> MemRegion ---> PhysFrame ---> Frame allocator
//
// In userland tests: &[u8] comes from Vec<u8>
// In kernel: &[u8] comes from (ptr, len) from the bootloader

#![allow(dead_code)]

use core::marker::PhantomData;

// ============================================================
// RAW ENTRY (this mirrors the bootloader wire format)
// ============================================================
//
// Multiboot1 memory map entry layout in RAM:
//
//   u32 size        (payload size, DOES NOT include this field)
//   u64 base_addr
//   u64 length
//   u32 type
//   extra bytes (optional if size > 20)
//
// IMPORTANT CONCEPT:
//
// This struct does NOT describe Rust memory.
// It describes hardware memory.
//
// The bootloader is not a Rust program.
// It just dumped bytes into RAM.
//
// Therefore:
//   this struct may be unaligned in real memory.
//
// That is why packed + read_unaligned is required.

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawEntry {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub typ: u32,
}

// ------------------------------------------------------------
// UNALIGNED READS
// ------------------------------------------------------------
//
// Why this exists:
//
// Normally Rust would do:
//     load 8 bytes aligned to 8
//
// But firmware might place this struct at an odd address.
//
// If you read a packed field normally:
//     CPU can fault OR Rust causes UB.
//
// So we do:
//     copy bytes out safely.
//
// Think:
//   "I am copying bytes out of unknown memory into a safe register."

impl RawEntry {
    // Read the field WITHOUT creating a reference to packed memory.
    // addr_of! gives raw pointer, not reference.
    // read_unaligned copies value safely.
    pub fn get_size_unaligned(&self) -> u32 {
        let p = core::ptr::addr_of!(self.size);
        unsafe { p.read_unaligned() }
    }

    pub fn get_base_addr_unaligned(&self) -> u64 {
        let p = core::ptr::addr_of!(self.base_addr);
        unsafe { p.read_unaligned() }
    }

    pub fn get_length_unaligned(&self) -> u64 {
        let p = core::ptr::addr_of!(self.length);
        unsafe { p.read_unaligned() }
    }

    pub fn get_type_unaligned(&self) -> u32 {
        let p = core::ptr::addr_of!(self.typ);
        unsafe { p.read_unaligned() }
    }
}

// ============================================================
// ERRORS
// ============================================================
//
// These are not “Rust errors”.
// These are “hardware validation failures”.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MmapError {
    // You could not even read the size field.
    // (Bootloader memory is shorter than 4 bytes)
    TruncatedHeader { have: usize },

    // size must be >= 20 (base + length + type)
    SizeTooSmall { size: u32 },

    // Entry claims to exist but runs past provided memory.
    // This prevents reading random RAM.
    TruncatedEntry { needed: usize, have: usize },
}

// ============================================================
// CONSTRUCTOR
// ============================================================

pub fn raw(start: u64, len: u64, kind: u32) -> RawEntry {
    // Minimal payload is ALWAYS 20 bytes:
    // 8 (base) + 8 (length) + 4 (type)
    //
    // This function is just a convenience for tests.
    // You are pretending to be the bootloader.

    RawEntry {
        size: 20,
        base_addr: start,
        length: len,
        typ: kind,
    }
}

// ============================================================
// SERIALIZATION (YOU BECOME THE BOOTLOADER)
// ============================================================

pub fn push_entry(buf: &mut Vec<u8>, entry: RawEntry) {
    // GOAL:
    // Convert a struct into the exact byte layout GRUB would place in RAM.

    // IMPORTANT CONCEPT:
    // Vec<u8> is a byte stream.
    // You are NOT pushing numbers.
    // You are pushing BYTES.

    // Step 1:
    // Read the fields safely using unaligned getters.

    // Step 2:
    // Each integer must be converted into LITTLE ENDIAN bytes.
    //
    // Ask yourself:
    //   How does a u64 become 8 individual u8 values?

    // Step 3:
    // Append those bytes into buf in this order:
    //   size -> base -> length -> type

    // Step 4:
    // If size > 20:
    //   The entry has extra payload bytes.
    //
    // You MUST append (size - 20) extra bytes.
    //
    // Tests expect the pattern 0xEE.

    todo!()
}

// ============================================================
// PARSER (MOST IMPORTANT FUNCTION IN THE FILE)
// ============================================================
//
// This function protects your kernel from crashing the CPU.

pub fn read_one(buf: &[u8]) -> Result<(RawEntry, usize), MmapError> {
    // Think EXACTLY in this order.

    // --------------------------------------------------------
    // 1) Can I read the header?
    // --------------------------------------------------------
    //
    // Need at least 4 bytes to read size.
    //
    // If buf shorter than 4:
    //   return TruncatedHeader

    // --------------------------------------------------------
    // 2) Read size
    // --------------------------------------------------------
    //
    // The size field is little-endian.
    //
    // You are converting 4 raw bytes -> u32 number.

    // --------------------------------------------------------
    // 3) Validate size
    // --------------------------------------------------------
    //
    // MB1 guarantee:
    //   size >= 20
    //
    // If smaller:
    //   bootloader memory is invalid.

    // --------------------------------------------------------
    // 4) Ensure the whole entry exists
    // --------------------------------------------------------
    //
    // Total entry bytes = 4 + size
    //
    // If buffer shorter than this:
    //   you must NOT read further.

    // --------------------------------------------------------
    // 5) Read payload
    // --------------------------------------------------------
    //
    // Offsets:
    //   base   : bytes 4..12
    //   length : bytes 12..20
    //   type   : bytes 20..24
    //
    // Ignore extra bytes beyond 20.

    // --------------------------------------------------------
    // 6) Return entry and how many bytes were consumed
    // --------------------------------------------------------
    //
    // consumed = 4 + size

    todo!()
}

// ============================================================
// ITERATOR (POINTER WALKER)
// ============================================================
//
// This walks a contiguous blob of bootloader memory.
//
// Real kernel equivalent:
//
//   ptr = ptr + (4 + size)
//
// Must NEVER infinite loop.

pub struct Mb1MmapIter<'a> {
    // You need:
    //   original buffer
    //   current offset inside it
    _p: PhantomData<&'a [u8]>,
}

impl<'a> Mb1MmapIter<'a> {
    pub fn new(_buf: &'a [u8]) -> Self {
        // Initialize offset to 0.

        todo!()
    }
}

impl<'a> Iterator for Mb1MmapIter<'a> {
    type Item = Result<RawEntry, MmapError>;

    fn next(&mut self) -> Option<Self::Item> {
        // If offset at end:
        //   return None

        // Call read_one on remaining slice.

        // If success:
        //   advance offset by consumed bytes
        //   return entry

        // If error:
        //   return the error ONCE
        //   then move offset to end
        //   (prevents infinite loop)

        todo!()
    }
}

// ============================================================
// SANITIZATION
// ============================================================
//
// RawEntry describes firmware claims.
// MemRegion describes safe kernel knowledge.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemRegion {
    pub start: u64,
    pub len: u64,
    pub kind: u32,
}

impl MemRegion {
    // end address of region
    pub fn end(self) -> u64 {
        // Decide overflow policy.
        // Kernel must NEVER wrap addresses.
        self.start.saturating_add(self.len)
    }
}

pub fn sanitize(e: RawEntry) -> Option<MemRegion> {
    // Drop zero-length regions.
    //
    // Then check:
    //   start + length overflow
    //
    // If overflow occurs:
    //   region is invalid -> return None
    //
    // Otherwise return MemRegion.

    todo!()
}

// ============================================================
// FRAMES (THIS IS THE REAL GOAL)
// ============================================================
//
// A PhysFrame is a 4KiB physical page.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysFrame(pub u64);

// alignment helpers
fn align_up(x: u64, a: u64) -> u64 {
    (x + (a - 1)) & !(a - 1)
}
fn align_down(x: u64, a: u64) -> u64 {
    x & !(a - 1)
}

pub struct UsableFrames<'a> {
    // You need:
    //   iterator over regions
    //   current frame pointer
    //   end pointer
    _p: PhantomData<&'a [MemRegion]>,
}

impl<'a> UsableFrames<'a> {
    pub fn new(_regions: &'a [MemRegion]) -> Self {
        // Prepare to iterate regions.

        todo!()
    }
}

impl<'a> Iterator for UsableFrames<'a> {
    type Item = PhysFrame;

    fn next(&mut self) -> Option<Self::Item> {
        // Algorithm:
        //
        // Loop:
        //   If current < end:
        //       return frame and advance by 4096
        //
        //   Otherwise:
        //       load next region
        //       skip if not type 1 (usable)
        //
        //       start = align_up(region.start, 4096)
        //       end   = align_down(region.end(), 4096)
        //
        //       if start >= end:
        //           continue
        //
        //       set current=start, end=end
        //       repeat

        todo!()
    }
}
