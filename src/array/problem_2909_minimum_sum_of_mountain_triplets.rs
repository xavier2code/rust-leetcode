/// You are given a 0-indexed array nums of integers.
/// A triplet of indices (i, j, k) is a mountain if:
/// i < j < k
/// nums[i] < nums[j] and nums[k] < nums[j]
/// Return the minimum possible sum of a mountain triplet of nums. If no such triplet exists, return -1.
#[allow(unused)]
pub fn minimum_sum_of_mountain_triplets(nums: Vec<i32>) -> i32 {
    // 暴力三循环
    let n = nums.len();
    let mut min_value = i32::MAX;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if nums[i] < nums[j] && nums[k] < nums[j] {
                    let temp_min_value = nums[i] + nums[j] + nums[k];
                    if temp_min_value < min_value {
                        min_value = temp_min_value
                    }
                }
            }
        }
    }
    if min_value == i32::MAX {
        return -1;
    }
    min_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_sum_of_mountain_triplets() {
        assert_eq!(minimum_sum_of_mountain_triplets(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(
            minimum_sum_of_mountain_triplets(vec![5, 4, 8, 7, 10, 2]),
            13
        );
        assert_eq!(minimum_sum_of_mountain_triplets(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
