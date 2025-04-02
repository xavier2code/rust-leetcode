/// You are given a 0-indexed array nums of integers.
/// A triplet of indices (i, j, k) is a mountain if:
/// i < j < k
/// nums[i] < nums[j] and nums[k] < nums[j]
/// Return the minimum possible sum of a mountain triplet of nums. If no such triplet exists, return -1.
///
#[allow(unused)]
pub fn minimum_sum_of_mountain_triplets(_nums: Vec<i32>) -> i32 {
    let nums_len = _nums.len();
    if nums_len < 3 {
        return -1;
    }
    let mut min_sum = i32::MAX;
    for i in 0..nums_len {
        for j in (i + 1)..nums_len {
            for k in (j + 1)..nums_len {
                if _nums[i] < _nums[j] && _nums[k] < _nums[j] {
                    let sum = _nums[i] + _nums[j] + _nums[k];
                    if min_sum == 0 || sum < min_sum {
                        min_sum = sum;
                    }
                }
            }
        }
    }
    if min_sum == i32::MAX {
        return -1;
    }
    min_sum
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
