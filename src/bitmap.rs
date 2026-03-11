use std::panic;

use tracing::{instrument, trace};

//https://docs.rs/fixedbitset/latest/fixedbitset/struct.FixedBitSet.html
/*
* contains(i)
* insert(i)
* remove(i)
* clear()
* len()
*
* ones() iteration
*
* Bitset::with_capacity(bits)
*
*    bitset
- tests
- implementation
- maybe persistence experiment
- optional concurrency pass

ring buffer
- tests
- implementation
- producer/consumer concurrency pass
- perf observation

hashmap
- tests
- implementation
- maybe snapshot/load experiment
- concurrent wrapper or sharded version

tiny append-only log
- write operations
- replay operations
- rebuild state

KV store
- in-memory first
- persistent next
- concurrent request handling next
- networked next

write tests
↓
define behavior
↓
stress the structure
↓
observe failures
↓
repair invariants
*/
//use proptest
#[derive(Debug)]
struct BitSet {
    bits: Vec<u64>,
    len: usize,
    capacity: usize,
}

impl BitSet {
    #[instrument(fields(number))]
    pub fn with_capacity(number: usize) -> Self {
        trace!(requested_capacity = number, "creating bitset");

        let mut vec = Vec::with_capacity(number);
        trace!(vec_length = vec.len(), "This is the Vec's length ");

        for i in 0..number {
            vec.push(0);
        }

        trace!(vec_capacity = vec.capacity(), "allocated backing vec");

        Self {
            bits: vec,
            len: 0,
            capacity: number,
        }
    }

    #[instrument(skip(self), fields(bit, len = self.len, capacity = self.capacity))]
    pub fn set(&mut self, bit: usize) -> Vec<u64> {
        self.len = bit % 64;
        let index = bit / 64;

        if bit > ((index + 1) * 64) {
            panic!("Value is outside of range of bits");
        }

        assert!(index < self.capacity, "Index out of bounds");

        let leftover = bit - self.len;
        trace!(leftover, "computed leftover");

        trace!("computed index * 64: {}", index * 64);

        let bit_index = self.bits[index];
        trace!(bit_index, "loaded current word");

        trace!("leftover is {}", self.len as u64);
        let mask = (1_u64) << (self.len as u64);
        trace!(mask, "computed mask");

        let bit_result: u64 = (bit as u64 - leftover as u64) & mask;
        trace!(bit_result, "computed bit result");

        self.bits[index] = bit_result;
        trace!(
            index,
            stored_value = self.bits[index],
            bits_len = self.bits.len(),
            "stored result into bitset"
        );

        self.bits.clone()
    }

    #[instrument(skip(self), fields(bit, len = self.len, capacity = self.capacity))]
    pub fn contains(&mut self, bit: usize) -> bool {
        //say for example the bit is 70 -- 70 / 10 = 7
        //now the bit is 62 -- 62  / 10 is 6.2 or 6
        let index = bit / 64;
        let leftover = bit % 64;
        trace!(index, "computed contains index");

        let lhs = self.bits[index] | 1_u64 << leftover;
        let rhs = (1_u64) << (leftover);

        trace!(
            lhs,
            rhs,
            word = self.bits[index],
            "evaluating contains comparison"
        );

        if lhs == rhs {
            trace!("contains returned true");
            return true;
        }

        trace!("contains returned false");
        false
    }
}

#[cfg(test)]
mod bit_tests {
    use crate::bitmap::BitSet;
    use std::sync::Once;
    use tracing_subscriber::fmt::format::FmtSpan;

    static INIT: Once = Once::new();

    fn init_tracing() {
        INIT.call_once(|| {
            let _ = tracing_subscriber::fmt()
                .with_max_level(tracing::Level::TRACE)
                .with_test_writer()
                .with_span_events(FmtSpan::CLOSE)
                .try_init();
            better_panic::install();
        });
    }

    #[test]
    fn test_set_then_contains() {
        init_tracing();
        let mut bitset = BitSet::with_capacity(128);

        bitset.set(70);

        assert!(bitset.contains(70));
    }

    #[test]
    fn test_bit_not_set_returns_false() {
        init_tracing();
        let mut bitset = BitSet::with_capacity(128);

        bitset.set(70);

        assert!(!bitset.contains(71));
    }

    #[test]
    fn test_bits_not_words_for_init() {}

    #[test]
    fn test_multiple_bits() {
        init_tracing();
        let mut bitset = BitSet::with_capacity(128);

        bitset.set(5);
        bitset.set(70);
        bitset.set(120);

        assert!(bitset.contains(5));
        assert!(bitset.contains(70));
        assert!(bitset.contains(120));
    }

    #[test]
    fn test_bit_persistence() {
        init_tracing();
        let mut bitset = BitSet::with_capacity(128);

        bitset.set(10);

        assert!(bitset.contains(10));
        assert!(bitset.contains(10));
        assert!(bitset.contains(10));
    }

    #[test]
    fn test_out_of_bounds_panics() {
        init_tracing();
        let mut bitset = BitSet::with_capacity(10);

        let result = std::panic::catch_unwind(move || {
            bitset.set(100);
        });

        assert!(result.is_err());
    }
}
