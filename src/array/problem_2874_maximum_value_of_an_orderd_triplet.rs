/// You are given a 0-indexed integer array nums.
/// Return the maximum value over all triplets of indices (i, j, k) such that i < j < k. If all such triplets have a negative value, return 0.
/// The value of a triplet of indices (i, j, k) is equal to (nums[i] - nums[j]) * nums[k].
///
#[allow(unused)]
pub fn maximum_value_of_ordered_triplet_1(_nums: Vec<i32>) -> i64 {
    let mut max_value = 0; // Fix: Set initial value to i64::MIN
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

/// i < j < k
///
///
///
#[allow(unused)]
pub fn maximum_value_of_ordered_triplet_2(nums: Vec<i32>) -> i64 {
    let mut stuff_max = nums.clone();
    for i in (0..nums.len()).rev().skip(1) {
        stuff_max[i] = stuff_max[i].max(stuff_max[i + 1]);
    }
    let mut max_value = 0;
    let mut prefix_max = nums[0];
    for i in 1..nums.len() - 1 {
        max_value = max_value.max((prefix_max - nums[i]) as i64 * stuff_max[i + 1] as i64);
        prefix_max = prefix_max.max(nums[i])
    }

    max_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_value_of_ordered_triplet_1() {
        assert_eq!(maximum_value_of_ordered_triplet_1(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(
            maximum_value_of_ordered_triplet_1(vec![1, 10, 3, 4, 19]),
            133
        );
        assert_eq!(maximum_value_of_ordered_triplet_1(vec![1, 2, 3]), 0);
    }

    #[test]
    fn test_maximum_value_of_ordered_triplet_2() {
        assert_eq!(maximum_value_of_ordered_triplet_2(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(
            maximum_value_of_ordered_triplet_2(vec![1, 10, 3, 4, 19]),
            133
        );
        assert_eq!(maximum_value_of_ordered_triplet_2(vec![1, 2, 3]), 0);
    }
}
