use std::f32::consts::LN_2;
use std::hash::{BuildHasher, Hasher};
use bit_vec::BitVec;
use xxhash_rust::xxh3::{Xxh3Builder};

pub struct BloomFilter<H1: BuildHasher, H2: BuildHasher> {
    bits: usize,
    data: BitVec,
    hash_count: u32,
    build_hasher_one: H1,
    build_hasher_two: H2,
}

pub type StandardBloomFilter = BloomFilter<Xxh3Builder, Xxh3Builder>;

impl StandardBloomFilter {
    pub fn with_entries_and_error(entries: i32, error: f32) -> StandardBloomFilter {
        let bits = StandardBloomFilter::calculate_bits(entries, error);
        let hash_count = StandardBloomFilter::calculate_hash_count(bits, entries);

        return StandardBloomFilter::with_bits_and_hash_count(bits, hash_count);
    }

    pub fn with_bits_and_hash_count(bits: usize, hash_count: u32) -> StandardBloomFilter {
        let data = BitVec::from_elem(bits as usize, false);
        let build_hasher_one = Xxh3Builder::new().with_seed(0x9747b28c);
        let build_hasher_two = Xxh3Builder::new().with_seed(0xe17a1465);

        BloomFilter {
            bits,
            data,
            hash_count,
            build_hasher_one,
            build_hasher_two,
        }
    }


    fn calculate_bits(entries: i32, error: f32) -> usize {
        let bits = (-entries as f32 * error.ln()) / (LN_2 * LN_2);

        return bits.ceil() as usize;
    }

    fn calculate_hash_count(bits: usize, entries: i32) -> u32 {
        let hashes = ((bits.clone() as f32 / entries.clone() as f32) * LN_2).ceil();

        return hashes as u32;
    }
}

impl<H1: BuildHasher, H2: BuildHasher> BloomFilter<H1, H2> {
    pub fn contains(&self, element: &str) -> bool {
        let hashes = self.find_hash_bits(element);

        for hash in hashes {
            if self.data.get(hash as usize).unwrap() == false {
                return false;
            }
        }

        return true;
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn insert(&mut self, element: &str) {
        let hashes = self.find_hash_bits(element);

        for hash in hashes {
            self.data.set(hash as usize, true);
        }
    }

    fn find_hash_bits(&self, value: &str) -> Vec<u64> {
        let mut hasher_one = self.build_hasher_one.build_hasher();
        let mut hasher_two = self.build_hasher_two.build_hasher();
        let bits = self.bits as u64;
        let mut hashes = Vec::with_capacity(self.hash_count as usize);

        if self.hash_count >= 1 {
            hasher_one.write(value.as_bytes());
            let hash = hasher_one.finish() % bits;
            hashes.insert(0, hash);
        }

        if self.hash_count >= 2 {
            hasher_two.write(value.as_bytes());
            let hash = hasher_two.finish() % bits;
            hashes.insert(1, hash);
        }
        for i in 2..self.hash_count as u64 {
            let hash = (hashes.get(0).unwrap() + i * hashes.get(1).unwrap()) % bits;
            hashes.insert(i as usize, hash)
        }

        hashes
    }
}

#[cfg(test)]
mod tests {
    use crate::bloom_filter::StandardBloomFilter;

    /*
     * Tests to verify the optimal number of bits. These tests were taken from the following sources:
     * https://github.com/alexanderbez/rust-bloom
     * https://github.com/SuperFluffy/counting-bloom-filter
     */
    #[test]
    fn test_optimal_num_bits() {
        assert_eq!(StandardBloomFilter::calculate_bits(10, 0.04), 67);
        assert_eq!(StandardBloomFilter::calculate_bits(5000, 0.01), 47926);
        assert_eq!(StandardBloomFilter::calculate_bits(100000, 0.01), 958506);
        assert_eq!(StandardBloomFilter::calculate_bits(10, 0.01), 96);
        assert_eq!(StandardBloomFilter::calculate_bits(5000, 0.01), 47926);
        assert_eq!(StandardBloomFilter::calculate_bits(100_000, 0.01), 958506);
    }

    #[test]
    fn test_optimal_num_hashes() {
        assert_eq!(StandardBloomFilter::calculate_hash_count(67, 10), 5);
        assert_eq!(StandardBloomFilter::calculate_hash_count(96, 10), 7);
        assert_eq!(StandardBloomFilter::calculate_hash_count(47926, 5000), 7);
        assert_eq!(StandardBloomFilter::calculate_hash_count(958506, 100000), 7);
    }
}
