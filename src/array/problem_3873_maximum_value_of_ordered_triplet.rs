/// You are given a 0-indexed integer array nums.
/// Return the maximum value over all triplets of indices (i, j, k) such that i < j < k. If all such triplets have a negative value, return 0.
/// The value of a triplet of indices (i, j, k) is equal to (nums[i] - nums[j]) * nums[k].
///
#[allow(unused)]
pub fn maximum_value_of_ordered_triplet(_nums: Vec<i32>) -> i64 {
    let mut max_value = i64::MIN; // 修复：将初始值设置为 i64::MIN
    let n = _nums.len();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let value = (_nums[i] - _nums[j]) as i64 * _nums[k] as i64;
                if value > max_value {
                    max_value = value;
                }
            }
        }
    }
    if max_value < 0 {
        return 0;
    }
    max_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_value_of_ordered_triplet() {
        assert_eq!(maximum_value_of_ordered_triplet(vec![1, 2, 3]), 0);
        assert_eq!(maximum_value_of_ordered_triplet(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(maximum_value_of_ordered_triplet(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(
            maximum_value_of_ordered_triplet(vec![1000000, 1, 1000000]),
            999999000000
        );
    }
}
