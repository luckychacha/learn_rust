
fn calculate_sum(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for item in nums.iter() {
        if sum.overflowing_add(*item).1 {
            return None;
        }
        sum = sum.overflowing_add(*item).0;
    }

    Some(sum)
}

fn main() {
    // let nums= [1,4,3,u32::MAX];
    let nums= [1,4,3,2];
    match calculate_sum(&nums[..]) {
        None => {
            println!("overflow!");
        }
        Some(sum) => {
            println!("sum is {}", sum);
        }
    };

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid() {
        let nums= [1,4,3,2];
        assert_eq!(calculate_sum(&nums[..]), Some(10));
    }

    #[test]
    fn overflow() {
        let nums= [1,4,3,u32::MAX];
        assert_eq!(calculate_sum(&nums[..]), None);
    }

}
