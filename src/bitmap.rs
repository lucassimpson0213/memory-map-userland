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
struct BitSet {
    bits: Vec<u64>,
    len: usize,
}

impl BitSet {
    pub fn with_capacity(number: usize) -> Self {
        let vec = Vec::with_capacity(number);
        Self {
            bits: vec,
            len: number,
        }
    }

    pub fn set(&mut self, bit: usize) {}
    pub fn contains(&mut self, bit: usize) {
        //say for example the bit is 70 -- 70 / 10 = 7
        //now the bit is 62 -- 62  / 10 is 6.2 or 6
        let index = bit / self.len;
    }
}

#[cfg(test)]
mod bit_tests {
    use crate::bitmap::BitSet;

    #[test]
    pub fn test_contains() {
        let mut bitset = BitSet::with_capacity(10);
        bitset.set(70);
        bitset.contains(70);
    }
}
