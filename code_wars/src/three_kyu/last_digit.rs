use core::slice::SlicePattern;
use std::slice;

pub fn last_digit(lst: &[u64]) -> u64 {
    if lst.len() < 2 {
        return 1;
    }
    let mut res: u128 = 1;
    for i in (0..lst.len()).rev() {
        if res < 4 {
            res = ((lst[i] % 1000) as u128).pow(res as u32);
        } else {
            res = ((lst[i] % 1000) as u128).pow((res % 4 + 4) as u32);
        }
    }

    (res % 10) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_last_digit() {
        let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6)
        ];

        for test in tests {
            assert_eq!(last_digit(&test.0), test.1);
        }
    }
}