// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
// 你可以按任意顺序返回答案。
// 示例 1：

// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
// 示例 2：

// 输入：nums = [3,2,4], target = 6
// 输出：[1,2]
// 示例 3：

// 输入：nums = [3,3], target = 6
// 输出：[0,1]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/two-sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut tmp = target;
    let mut start = 0;
    while start < nums.len() {
        let first = *nums.get(start).unwrap();
        res.push(start as i32);
        start += 1;
        tmp -= first;
        // if tmp <= 0 {
        //     break;
        // }
        for (idx, item) in nums.iter().skip(start).enumerate() {
            if *item == tmp {
                res.push((idx + start) as i32);
                return res;
            }
        }
        tmp = target;
        res = vec![];
    }

    return Vec::new();
}

#[cfg(test)]
mod test {
    use super::*;
    
    // #[test]
    // fn test_two_sum_1() {
    //     let nums = vec![2,7,11,15];
    //     let target = 9;
    //     assert_eq!(two_sum(nums, target), vec![0, 1]);
    // }

    #[test]
    fn test_two_sum_2() {
        let nums = vec![3,2,4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }
    
    #[test]
    fn test_two_sum_3() {
        let nums = vec![3,2,3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 2]);
    }
    
    #[test]
    fn test_two_sum_4() {
        let nums = vec![0,4,3,0];
        let target = 0;
        assert_eq!(two_sum(nums, target), vec![0, 3]);
    }
    
    
}