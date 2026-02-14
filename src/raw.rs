// mb1_memmap.rs
#![allow(dead_code)]

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawEntry {
    pub size: u32,      // payload size (MB1: >= 20, excludes this size field)
    pub base_addr: u64, // start
    pub length: u64,    // len
    pub typ: u32,       // kind
}
//unaligned getters, rust does not allow references to unaligned memory
impl RawEntry {
    pub fn get_size_unaligned(self) -> u32 {
        let pointer = core::ptr::addr_of!(self.size);
        let val = unsafe { pointer.read_unaligned() };
        return val;
    }

    pub fn get_base_addr_unaligned(self) -> u64 {
        let pointer = core::ptr::addr_of!(self.base_addr);
        let val = unsafe { pointer.read_unaligned() };
        return val;
    }
    pub fn get_length_unaligned(self) -> u64 {
        let pointer = core::ptr::addr_of!(self.length);
        let val = unsafe { pointer.read_unaligned() };
        return val;
    }
    pub fn get_type_unaligned(self) -> u32 {
        let pointer = core::ptr::addr_of!(self.typ);
        let val = unsafe { pointer.read_unaligned() };
        return val;
    }
}

// -------------------------
// Public API you implement
// -------------------------

// @doc: memmap
// use <leader>od to open the doc

/// Create a minimal MB1 entry (payload size = 20).
pub fn raw(start: u64, len: u64, kind: u32) -> RawEntry {
    // TODO: return a RawEntry with size=20 and fields set
    return RawEntry {
        size: 20,
        base_addr: start,
        length: len,
        typ: kind,
    };
}

/// Append an entry in MB1 mmap wire format (little-endian).
///
/// Wire format:
/// - u32 size (payload size, excludes this u32)
/// - u64 base_addr
/// - u64 length
/// - u32 typ
/// - (optional extra payload bytes if size > 20)
pub fn push_entry(buf: &mut Vec<u8>, entry: RawEntry) {
    // TODO:
    // - append entry.size (LE)
    // - append base_addr (LE)
    // - append length (LE)
    // - append typ (LE)
    //
    // IMPORTANT:
    // - This function should append bytes into `buf` (not print a pointer).
    // - Tests will also use size>20 and expect you to append (size-20) extra bytes.
    //   Pick a fill pattern for those extra bytes (e.g., 0xEE) and keep consistent.
    let tipo = entry.get_type_unaligned();
    let base_addr = entry.get_base_addr_unaligned();
    let length = entry.get_length_unaligned();
    let size = entry.get_size_unaligned();

    buf.push(tipo);
    buf.push(base_addr);
    buf.push(length);
    buf.push(size);
}

/// Parse ONE entry from a byte slice.
/// Returns Ok((entry, bytes_consumed)) or Err.
///
/// bytes_consumed must be: 4 + entry.size
pub fn read_one(buf: &[u8]) -> Result<(RawEntry, usize), MmapError> {
    // TODO:
    // - if buf < 4 => Err(TruncatedHeader)
    // - read size LE
    // - validate size >= 20 (else Err(SizeTooSmall{size}))
    // - needed = 4 + size as usize; if buf < needed => Err(TruncatedEntry{needed, have})
    // - read base_addr, length, typ from first 20 bytes of payload
    // - ignore extra payload bytes (size-20)
    // - return entry with that size field preserved (even if >20)
    todo!()
}

/// Iterator over a full MB1 mmap blob.
/// Stops at end, or yields Err for invalid entries.
/// Must not infinite-loop (especially size==0).
pub struct Mb1MmapIter<'a> {
    // TODO: store buf and current offset
    _p: core::marker::PhantomData<&'a [u8]>,
}

impl<'a> Mb1MmapIter<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        // TODO
        todo!()
    }
}

impl<'a> Iterator for Mb1MmapIter<'a> {
    type Item = Result<RawEntry, MmapError>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO:
        // - if at end => None
        // - call read_one on remaining slice
        // - on Ok((e, consumed)):
        //     advance offset by consumed (4 + e.size)
        //     return Some(Ok(e))
        // - on Err(e):
        //     advance offset in a way that guarantees progress OR end iteration
        //     (common policy: return Some(Err(e)) and then set offset = buf.len())
        //     so you don't yield the same error forever.
        todo!()
    }
}

/// Optional: your “sanitize” stage for later phases.
/// For now, leaving it TODO; tests for sanitize can be ignored until you implement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemRegion {
    pub start: u64,
    pub len: u64,
    pub kind: u32,
}

impl MemRegion {
    pub fn end(self) -> u64 {
        // TODO: return start + len (choose overflow policy in sanitize)
        todo!()
    }
}

pub fn sanitize(_e: RawEntry) -> Option<MemRegion> {
    // TODO (phase 2):
    // - drop len==0
    // - handle overflow start+len (reject or clamp)
    // - decide what to do with kinds (maybe only typ==1 is usable)
    todo!()
}

// -------------------------
// Errors you implement
// -------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MmapError {
    TruncatedHeader { have: usize },
    SizeTooSmall { size: u32 },
    TruncatedEntry { needed: usize, have: usize },
}

// -------------------------
// Tests
// -------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    const MIN_PAYLOAD: u32 = 20;

    /// Helper for building arbitrary-size entries directly into a buffer.
    /// This is used by tests to craft tricky cases even before push_entry is done.
    fn push_mb1_entry(buf: &mut Vec<u8>, payload_size: u32, start: u64, len: u64, kind: u32) {
        buf.extend_from_slice(&payload_size.to_le_bytes());
        buf.extend_from_slice(&start.to_le_bytes());
        buf.extend_from_slice(&len.to_le_bytes());
        buf.extend_from_slice(&kind.to_le_bytes());

        let extra = payload_size.saturating_sub(MIN_PAYLOAD) as usize;
        if extra > 0 {
            buf.extend_from_slice(&vec![0xEE; extra]);
        }
    }

    fn truncate_end(buf: &mut Vec<u8>, n: usize) {
        let new_len = buf.len().saturating_sub(n);
        buf.truncate(new_len);
    }

    // -------------------------
    // Layout / builder
    // -------------------------

    #[test]
    fn rawentry_layout_is_expected() {
        assert_eq!(mem::size_of::<RawEntry>(), 24);
        assert_eq!(mem::align_of::<RawEntry>(), 1);
    }

    #[test]
    fn raw_builder_minimal() {
        let e = raw(0x1000, 0x9000, 1);
        assert_eq!(e.size, 20);
        assert_eq!(e.base_addr, 0x1000);
        assert_eq!(e.length, 0x9000);
        assert_eq!(e.typ, 1);
    }

    // -------------------------
    // push_entry behavior
    // -------------------------

    #[test]
    fn push_entry_appends_24_for_minimal() {
        let mut buf = Vec::new();
        push_entry(&mut buf, raw(0x1000, 0x9000, 1));
        assert_eq!(buf.len(), 24);
    }

    #[test]
    fn push_entry_writes_expected_wire_format_for_minimal() {
        let mut buf = Vec::new();
        let e = RawEntry {
            size: 20,
            base_addr: 0x1122334455667788,
            length: 0x0102030405060708,
            typ: 0xAABBCCDD,
        };
        push_entry(&mut buf, e);

        assert_eq!(buf[0..4], 20u32.to_le_bytes());
        assert_eq!(buf[4..12], 0x1122334455667788u64.to_le_bytes());
        assert_eq!(buf[12..20], 0x0102030405060708u64.to_le_bytes());
        assert_eq!(buf[20..24], 0xAABBCCDDu32.to_le_bytes());
    }

    #[test]
    fn push_entry_with_extra_payload_appends_extra_bytes() {
        let mut buf = Vec::new();
        let e = RawEntry {
            size: 28, // payload includes 8 extra bytes beyond the required 20
            base_addr: 0x1000,
            length: 0x2000,
            typ: 1,
        };
        push_entry(&mut buf, e);

        // total bytes = 4 + size
        assert_eq!(buf.len(), (4 + 28) as usize);
        // extra bytes should exist (whatever pattern you chose; tests assume 0xEE)
        assert_eq!(&buf[24..32], &[0xEE; 8]);
    }

    // -------------------------
    // read_one behavior
    // -------------------------

    #[test]
    fn read_one_rejects_truncated_header() {
        let buf = vec![0xAA, 0xBB, 0xCC]; // < 4
        let err = read_one(&buf).unwrap_err();
        assert_eq!(err, MmapError::TruncatedHeader { have: 3 });
    }

    #[test]
    fn read_one_rejects_size_less_than_20() {
        let mut buf = Vec::new();
        push_mb1_entry(&mut buf, 19, 0x1000, 0x1000, 1);
        let err = read_one(&buf).unwrap_err();
        assert_eq!(err, MmapError::SizeTooSmall { size: 19 });
    }

    #[test]
    fn read_one_rejects_truncated_entry() {
        let mut buf = Vec::new();
        push_mb1_entry(&mut buf, 20, 0x1000, 0x1000, 1);
        truncate_end(&mut buf, 1);

        let err = read_one(&buf).unwrap_err();
        // needed is 4+20=24, have is 23
        assert_eq!(
            err,
            MmapError::TruncatedEntry {
                needed: 24,
                have: 23
            }
        );
    }

    #[test]
    fn read_one_parses_minimal_ok() {
        let mut buf = Vec::new();
        push_mb1_entry(&mut buf, 20, 0x1000, 0x9000, 1);

        let (e, consumed) = read_one(&buf).unwrap();
        assert_eq!(consumed, 24);
        assert_eq!(e.size, 20);
        assert_eq!(e.base_addr, 0x1000);
        assert_eq!(e.length, 0x9000);
        assert_eq!(e.typ, 1);
    }

    #[test]
    fn read_one_parses_and_skips_extra_payload() {
        let mut buf = Vec::new();
        push_mb1_entry(&mut buf, 28, 0x1000, 0x1111, 2);

        let (e, consumed) = read_one(&buf).unwrap();
        assert_eq!(consumed, (4 + 28) as usize);
        assert_eq!(e.size, 28);
        assert_eq!(e.base_addr, 0x1000);
        assert_eq!(e.length, 0x1111);
        assert_eq!(e.typ, 2);
    }

    // -------------------------
    // Iterator behavior
    // -------------------------

    #[test]
    fn iter_parses_single_entry_and_ends() {
        let mut buf = Vec::new();
        push_mb1_entry(&mut buf, 20, 0x1000, 0x9000, 1);

        let mut it = Mb1MmapIter::new(&buf);
        let e = it.next().expect("one item").expect("ok");

        assert_eq!(e.base_addr, 0x1000);
        assert!(it.next().is_none());
    }

    #[test]
    fn iter_parses_multiple_entries_in_order() {
        let mut buf = Vec::new();
        push_mb1_entry(&mut buf, 20, 0x1000, 0x1000, 1);
        push_mb1_entry(&mut buf, 28, 0x3000, 0x2000, 2);
        push_mb1_entry(&mut buf, 20, 0x9000, 0x1000, 1);

        let starts: Vec<u64> = Mb1MmapIter::new(&buf)
            .map(|r| r.unwrap().base_addr)
            .collect();

        assert_eq!(starts, vec![0x1000, 0x3000, 0x9000]);
    }

    #[test]
    fn iter_size_zero_does_not_infinite_loop() {
        // size=0 is invalid (size < 20). Iterator must not get stuck.
        let mut buf = Vec::new();
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&[0xAA; 64]);

        let mut it = Mb1MmapIter::new(&buf);
        let first = it.next().expect("must return something");
        assert!(first.is_err(), "size=0 should error");
        // after error, iterator should stop (policy enforced in next())
        assert!(it.next().is_none());
    }

    #[test]
    fn iter_truncated_entry_yields_error_once_then_stops() {
        let mut buf = Vec::new();
        push_mb1_entry(&mut buf, 20, 0x1000, 0x1000, 1);
        truncate_end(&mut buf, 5);

        let mut it = Mb1MmapIter::new(&buf);
        assert!(it.next().unwrap().is_err());
        assert!(it.next().is_none(), "must not repeat same error forever");
    }

    // -------------------------
    // sanitize tests (phase 2)
    // -------------------------
    // Uncomment when you implement sanitize.

    /*
    #[test]
    fn sanitize_drops_zero_length() {
        let e = raw(0x2000, 0, 1);
        assert!(sanitize(e).is_none());
    }

    #[test]
    fn sanitize_handles_overflow_start_plus_len() {
        let e = raw(u64::MAX - 0xF, 0x200, 1);
        let region = sanitize(e);

        // Choose one policy:
        // assert!(region.is_none()); // reject overflow
        if let Some(r) = region {
            assert!(r.end() >= r.start, "end must not wrap");
            assert_eq!(r.end(), u64::MAX, "if clamping, end saturates");
        }
    }
    */
}
