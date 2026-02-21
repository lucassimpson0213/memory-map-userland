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
#![allow(unused_variables)]

