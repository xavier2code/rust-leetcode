/// You are given a 0-indexed integer array nums.
/// Return the maximum value over all triplets of indices (i, j, k) such that i < j < k. If all such triplets have a negative value, return 0.
/// The value of a triplet of indices (i, j, k) is equal to (nums[i] - nums[j]) * nums[k].
/// 
#[allow(dead_code)]
pub fn maximum_value_of_ordered_triplet(_nums: Vec<i32>) -> i64 {

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_value_of_ordered_triplet() {
        assert_eq!(maximum_value_of_ordered_triplet(vec![1, 2, 3]), 0);
        assert_eq!(maximum_value_of_ordered_triplet(vec![12,6,1,2,7]), 77);
        assert_eq!(maximum_value_of_ordered_triplet(vec![1,10,3,4,19]), 133);
    }
}