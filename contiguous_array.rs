/*
    https://leetcode.com/problems/contiguous-array/
    Given bin array `nums`, ret maxlen of contiguous subarray with equal amount of `0` and `1`.
*/

fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut curr_sum: i32 = 0;
    let mut max_len: i32 = 0;
    let mut seen = std::collections::HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        curr_sum += if n == &0 { -1 } else { 1 };
        if let Some(val) = seen.get(&curr_sum) {
            max_len = max_len.max((i - *f) as i32);
        } else {
            seen.insert(curr_sum, i);
        }

        if curr_sum == 0 {
            max_len = max_len.max((i + 1) as i32);
        }
    }

    max_len
}
