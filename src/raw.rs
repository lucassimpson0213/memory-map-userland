#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawEntry {
    pub start: u64,
    pub len: u64,
    pub kind: u32,
    pub _pad: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_is_correct() {
        assert_eq!(core::mem::size_of::<RawEntry>(), 24);
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn create_fake_map() {
        let entries = [
            RawEntry {
                start: 0x100000,
                len: 0x400000,
                kind: 1,
                _pad: 0,
            },
            RawEntry {
                start: 0x500000,
                len: 0x100000,
                kind: 2,
                _pad: 0,
            },
        ];

        let bytes = unsafe {
            core::slice::from_raw_parts(
                entries.as_ptr() as *const u8,
                entries.len() * core::mem::size_of::<RawEntry>(),
            )
        };

        assert_eq!(bytes.len(), 48);
    }
}
