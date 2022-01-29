pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();

    let mut res = std::collections::HashSet::new();
    for i in 0..n {
        let first = nums[i];
        for j in i+1..n {
            let second = nums[j];
            let third = 0 - first - second;
            match nums[j+1..].binary_search(&third) {
                Ok(_) => res.insert(vec![nums[i], nums[j], third]),
                Err(_) => false,
            };
        }
    }

    res.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum_1() {
        let nums = vec![-1,0,1,2,-1,-4];
        assert_eq!(three_sum(nums), vec![vec![-1,-1,2],vec![-1,0,1]])
    }
}