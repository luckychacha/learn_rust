pub fn last_digit(lst: &[u64]) -> u64 {
    if lst.len() == 0 { return 1; }
    let mut n: u128 = 1;
    for i in (0..lst.len()).rev() {
        if n < 4 {
            n = ((lst[i] % 1000) as u128).pow(n as u32);
        }
        else {
            // a.pow(n) % 10 == a.pow(n % 4 + 4) % 10
            n = ((lst[i] % 1000) as u128).pow((n % 4 + 4) as u32);
        }
    }
    (n % 10) as u64
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