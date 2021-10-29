fn find_prime_factors_of_n(n: i64) -> Vec<i64> {
    let mut n = n.abs();
    let mut prime_factors: Vec<i64> = Vec::new();
    for i in 2..((n as f64).sqrt() as i64 + 1) {
        while n % i == 0 {
            prime_factors.push(i);
            n /= i;
        }
    }
    if n > 1 {
        prime_factors.push(n);
    }
    prime_factors
}

pub fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    // your code
    let mut all_prime_factors: Vec<i64> = Vec::new();
    for i in &l {
        for j in find_prime_factors_of_n(*i) {
            all_prime_factors.push(j);
        }
    }
    all_prime_factors.sort();
    all_prime_factors.dedup();

    all_prime_factors.into_iter().map(|factor| {
        (factor, l.iter().filter(|&arg| arg % factor == 0).sum())
    }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_divided() {
        assert_eq!(
            sum_of_divided(vec![12, 15]), vec![ (2, 12), (3, 27), (5, 15) ]
        );
    }
}