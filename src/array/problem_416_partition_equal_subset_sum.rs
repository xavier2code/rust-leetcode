/// Given an integer array nums, return true if you can partition the array into two
/// subsets such that the sum of the elements in both subsets is equal or false otherwise.
///
#[allow(unused)]
pub fn partition_equal_subset_sum(nums: Vec<i32>) -> bool {
    // 如果数组的和是奇数，则不能分成两个相等的子集
    let sum: i32 = nums.iter().sum();
    if sum % 2 != 0 {
        return false;
    }
    let target = sum / 2;
    let n = nums.len();
    // 创建一个 dp 数组，dp[i] 表示是否可以用 nums 中的元素组成和为 i 的子集
    let mut dp = vec![false; (target + 1) as usize];
    dp[0] = true; // 和为 0 的子集是空集
    // 遍历 nums 中的每个元素
    for &num in &nums {
        // 从后往前遍历 dp 数组，避免重复使用同一个元素
        for j in (num..=target).rev() {
            dp[j as usize] = dp[j as usize] || dp[(j - num) as usize];
        }
    }
    // 返回 dp[target]，表示是否可以用 nums 中的元素组成和为 target 的子集
    dp[target as usize];
    // 如果 dp[target] 为 true，则表示可以分成两个相等的子集
    // 否则返回 false
    if dp[target as usize] {
        return true;
    }
    // 如果 dp[target] 为 false，则表示不能分成两个相等的子集
    // 返回 false
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_equal_subset_sum() {
        assert_eq!(partition_equal_subset_sum(vec![1, 5, 11, 5]), true);
        assert_eq!(partition_equal_subset_sum(vec![1, 2, 3, 5]), false);
        assert_eq!(partition_equal_subset_sum(vec![1, 2, 3, 4]), true);
        assert_eq!(partition_equal_subset_sum(vec![1, 2]), false);
    }
}
