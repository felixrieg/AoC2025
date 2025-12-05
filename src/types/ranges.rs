#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    pub fn size(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    pub fn contains(&self, n: usize) -> bool {
        n >= self.start && n <= self.end
    }

    pub fn from_string(line: &str) -> Option<Self> {
        let mut parts = line.split('-');
        let start: usize = parts.next()?.parse().ok()?;
        let end: usize = parts.next()?.parse().ok()?;
        Some(Range::new(start, end))
    }

    pub fn overlaps_with(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    pub fn merge_into(&mut self, other: &Range) {
        if self.overlaps_with(other) {
            self.end = self.end.max(other.end);
        }
    }
}

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end {
            let current = self.start;
            self.start += 1;
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_size() {
        let r = Range::new(1, 5);
        assert_eq!(r.size(), 5);
    }

    #[test]
    fn test_range_contains() {
        let r = Range::new(1, 5);
        assert!(r.contains(3));
        assert!(!r.contains(6));
        assert!(r.contains(1));
        assert!(r.contains(5));
    }

    #[test]
    fn test_range_overlaps() {
        let r1 = Range::new(1, 5);
        let r2 = Range::new(4, 8);
        let r3 = Range::new(6, 10);

        assert!(r1.overlaps_with(&r2));
        assert!(!r1.overlaps_with(&r3));
    }

    #[test]
    fn test_range_merge() {
        let mut r1 = Range::new(1, 5);
        let r2 = Range::new(4, 8);
        r1.merge_into(&r2);

        assert_eq!(r1.start, 1);
        assert_eq!(r1.end, 8);
    }

    #[test]
    fn test_range_from_string_basic() {
        let r = Range::from_string("1-5");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start, 1);
        assert_eq!(r.end, 5);
    }

    #[test]
    fn test_range_from_string_large_numbers() {
        let r = Range::from_string("100-999");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start, 100);
        assert_eq!(r.end, 999);
    }

    #[test]
    fn test_range_from_string_same_start_end() {
        let r = Range::from_string("5-5");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start, 5);
        assert_eq!(r.end, 5);
        assert_eq!(r.size(), 1);
    }

    #[test]
    fn test_range_from_string_zero() {
        let r = Range::from_string("0-10");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start, 0);
        assert_eq!(r.end, 10);
    }

    #[test]
    fn test_range_from_string_invalid_no_dash() {
        let r = Range::from_string("1 5");
        assert!(r.is_none());
    }

    #[test]
    fn test_range_from_string_invalid_non_numeric() {
        let r = Range::from_string("a-b");
        assert!(r.is_none());
    }

    #[test]
    fn test_range_from_string_invalid_empty() {
        let r = Range::from_string("");
        assert!(r.is_none());
    }

    #[test]
    fn test_range_from_string_invalid_only_dash() {
        let r = Range::from_string("-");
        assert!(r.is_none());
    }

    #[test]
    fn test_range_from_string_invalid_missing_end() {
        let r = Range::from_string("5-");
        assert!(r.is_none());
    }

    #[test]
    fn test_range_from_string_invalid_missing_start() {
        let r = Range::from_string("-10");
        assert!(r.is_none());
    }

    #[test]
    fn test_range_from_string_multiple_dashes() {
        let r = Range::from_string("1-5-10");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start, 1);
        assert_eq!(r.end, 5);
    }

    #[test]
    fn test_range_from_string_with_whitespace() {
        let r = Range::from_string(" 1 - 5 ");
        assert!(r.is_none());
    }

    #[test]
    fn test_range_from_string_reversed_range() {
        // Das sollte funktionieren - es ist ein gültiger Range
        let r = Range::from_string("10-5");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start, 10);
        assert_eq!(r.end, 5);
    }

    #[test]
    fn test_range_from_string_very_large_numbers() {
        let r = Range::from_string("1000000-9999999");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start, 1000000);
        assert_eq!(r.end, 9999999);
    }

    #[test]
    fn test_range_iterator_basic() {
        let mut r = Range::new(1, 3);
        assert_eq!(r.next(), Some(1));
        assert_eq!(r.next(), Some(2));
        assert_eq!(r.next(), Some(3));
        assert_eq!(r.next(), None);
    }

    #[test]
    fn test_range_iterator_single_element() {
        let mut r = Range::new(5, 5);
        assert_eq!(r.next(), Some(5));
        assert_eq!(r.next(), None);
    }

    #[test]
    fn test_range_iterator_large_range() {
        let mut r = Range::new(1, 10);
        let mut count = 0;
        while let Some(val) = r.next() {
            count += 1;
            assert!(val >= 1 && val <= 10);
        }
        assert_eq!(count, 10);
    }

    #[test]
    fn test_range_iterator_collect() {
        let r = Range::new(1, 5);
        let collected: Vec<usize> = r.collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_range_iterator_empty_range() {
        // Wenn start > end, sollte gleich None zurückgeben
        let mut r = Range::new(5, 3);
        assert_eq!(r.next(), None);
    }

    #[test]
    fn test_range_iterator_zero_start() {
        let r = Range::new(0, 3);
        let collected: Vec<usize> = r.collect();
        assert_eq!(collected, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_range_iterator_large_numbers() {
        let r = Range::new(1000, 1005);
        let collected: Vec<usize> = r.collect();
        assert_eq!(collected, vec![1000, 1001, 1002, 1003, 1004, 1005]);
    }

    #[test]
    fn test_range_iterator_filter() {
        let r = Range::new(1, 10);
        let even: Vec<usize> = r.filter(|x| x % 2 == 0).collect();
        assert_eq!(even, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_range_iterator_map() {
        let r = Range::new(1, 3);
        let doubled: Vec<usize> = r.map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_range_iterator_sum() {
        let r = Range::new(1, 5);
        let sum: usize = r.sum();
        assert_eq!(sum, 15); // 1+2+3+4+5 = 15
    }

    #[test]
    fn test_range_iterator_count() {
        let r = Range::new(1, 10);
        let count = r.count();
        assert_eq!(count, 10);
    }

    #[test]
    fn test_range_iterator_any() {
        let r = Range::new(1, 10);
        assert!(r.clone().any(|x| x == 5));

        let mut r = Range::new(1, 10);
        assert!(!r.any(|x| x == 15));
    }

    #[test]
    fn test_range_iterator_all() {
        let r = Range::new(1, 5);
        assert!(r.clone().all(|x| x > 0));

        let mut r = Range::new(1, 5);
        assert!(!r.all(|x| x > 3));
    }

    #[test]
    fn test_range_iterator_find() {
        let mut r = Range::new(1, 10);
        assert_eq!(r.find(|x| x > &5), Some(6));
    }

    #[test]
    fn test_range_iterator_skip() {
        let r = Range::new(1, 5);
        let skipped: Vec<usize> = r.skip(2).collect();
        assert_eq!(skipped, vec![3, 4, 5]);
    }

    #[test]
    fn test_range_iterator_take() {
        let r = Range::new(1, 10);
        let taken: Vec<usize> = r.take(3).collect();
        assert_eq!(taken, vec![1, 2, 3]);
    }

    #[test]
    fn test_range_iterator_into_iter() {
        let r = Range::new(1, 3);
        let collected: Vec<usize> = r.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_range_iterator_for_loop() {
        let mut result = Vec::new();
        for i in Range::new(1, 4) {
            result.push(i);
        }
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_range_iterator_chain() {
        let r1 = Range::new(1, 3);
        let r2 = Range::new(4, 6);
        let chained: Vec<usize> = r1.chain(r2).collect();
        assert_eq!(chained, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_range_iterator_zip() {
        let r1 = Range::new(1, 3);
        let r2 = Range::new(10, 12);
        let zipped: Vec<(usize, usize)> = r1.zip(r2).collect();
        assert_eq!(zipped, vec![(1, 10), (2, 11), (3, 12)]);
    }

    #[test]
    fn test_range_iterator_position() {
        let mut r = Range::new(1, 5);
        assert_eq!(r.position(|x| x == 3), Some(2)); // 3 ist an Position 2 (0-indexed)
    }

    #[test]
    fn test_range_iterator_enumerate() {
        let r = Range::new(10, 12);
        let enumerated: Vec<(usize, usize)> = r.enumerate().collect();
        assert_eq!(enumerated, vec![(0, 10), (1, 11), (2, 12)]);
    }
}
