/// This iterator gives output of numbers from 0 to max, where each number given increases in number of bits set.
/// If max = 12, it produces 1 2 4 8 - all with one bit set, then
/// 3 5 6 9 10 12 - two bits set, then
/// 7 11 - 3 bits set
#[derive(Debug)]
pub struct GosperIterator {
    max: u32,
    bits: u32,
    v: u32,
}

impl GosperIterator {
    pub fn new(max: u32) -> Self {
        GosperIterator { max, bits: 1, v: 1 }
    }
}

impl Iterator for GosperIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. If the current value is already out of range, we stop immediately.
        if self.v > self.max || self.bits > 32 {
            return None;
        }

        let current_val = self.v;

        // 2. Calculate the next permutation for the SAME bit count
        let lowbit = self.v & self.v.wrapping_neg();
        let tmp = self.v + lowbit;
        let next_v = (((self.v ^ tmp) >> 2) / lowbit) | tmp;

        // 3. Prepare the state for the NEXT call
        if next_v > self.max {
            // Jump to the next Hamming Weight group
            self.bits += 1;

            if self.bits > 32 {
                self.v = self.max + 1; // Exhausted
            } else {
                // Smallest number with (bits + 1) bits set
                self.v = (1u32 << self.bits).wrapping_sub(1);
            }
        } else {
            self.v = next_v;
        }

        // 4. Return the value we found at the start
        Some(current_val)
    }
}
