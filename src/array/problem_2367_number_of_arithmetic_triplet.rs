/// You are given a 0-indexed, strictly increasing integer array nums and a positive integer diff. A triplet (i, j, k) is an arithmetic triplet if the following conditions are met:
/// i < j < k,
/// nums[j] - nums[i] == diff, and
/// nums[k] - nums[j] == diff.
/// Return the number of unique arithmetic triplets.
///
#[allow(unused)]
pub fn number_of_arithmetic_triplet(_nums: Vec<i32>, _diff: i32) -> i32 {
    let nums_len = _nums.len();
    if nums_len < 3 {
        return 0;
    }
    let mut count = 0;
    for i in 0..nums_len {
        for j in (i + 1)..nums_len {
            for k in (j + 1)..nums_len {
                if _nums[j] - _nums[i] == _diff && _nums[k] - _nums[j] == _diff {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_triplets() {
        assert_eq!(number_of_arithmetic_triplet(vec![0, 1, 4, 6, 7, 10], 3), 2);
        assert_eq!(number_of_arithmetic_triplet(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
