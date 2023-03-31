mod bloom_filter;

#[cfg(test)]
mod tests {
    use crate::bloom_filter::{StandardBloomFilter, BloomFilter};

    #[test]
    fn contains_find() {
        let mut filter = StandardBloomFilter::with_entries_and_error(100, 0.01);
        filter.insert("Pineapple");
        let result = filter.contains("Pineapple");
        assert_eq!(result, true);
    }

    #[test]
    fn contains_cannot_find() {
        let mut filter = StandardBloomFilter::with_entries_and_error(100, 0.01);
        filter.insert("Pineapple");
        let result = filter.contains("OrangePineapple");
        assert_eq!(result, false);
    }

    #[test]
    fn clear_works() {
        let mut filter = StandardBloomFilter::with_entries_and_error(100, 0.01);
        filter.insert("Pineapple");
        filter.insert("OrangePineapple");
        let result1 = filter.contains("Pineapple");
        let result2 = filter.contains("OrangePineapple");
        assert_eq!(result1, true);
        assert_eq!(result2, true);

        filter.clear();

        let result3 = filter.contains("Pineapple");
        let result4 = filter.contains("OrangePineapple");
        assert_eq!(result3, false);
        assert_eq!(result4, false);
    }
}
