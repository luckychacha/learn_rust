// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。

// 算法的时间复杂度应该为 O(log (m+n)) 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let min_left = (nums1.len() + nums2.len() + 1) / 2;
    let min_right = (nums1.len() + nums2.len() + 2) / 2;
    (find_k(&nums1, 0, &nums2, 0, min_left) + find_k(&nums1, 0, &nums2, 0, min_right)) as f64 / 2.0
}

fn find_k(nums1: &Vec<i32>, nums1_idx: usize, nums2: &Vec<i32>, nums2_idx: usize, k: usize) -> i32 {
    if nums1_idx >= nums1.len() {
        return nums2[nums2_idx + k - 1];
    }
    if nums2_idx >= nums2.len() {
        return nums1[nums1_idx + k - 1];
    }

    if k == 1 {
        return std::cmp::min(nums1[nums1_idx], nums2[nums2_idx]);
    }


    let mid1 = if (nums1_idx + k/2 -1) < nums1.len() {
        nums1[nums1_idx + k/2 -1]
    } else {
        i32::MAX
    };

    let mid2 = if (nums2_idx + k/2 -1) < nums2.len() {
        nums2[nums2_idx + k/2 -1]
    } else {
        i32::MAX
    };

    if mid1 > mid2 {
        return find_k(nums1, nums1_idx, nums2, nums2_idx + k/2, k - k/2);
    }
    find_k(nums1, nums1_idx + k/2, nums2, nums2_idx, k - k/2)
}