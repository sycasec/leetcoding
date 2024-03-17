use std::collections::HashMap;
/*  Contiguous Array
    https://leetcode.com/problems/contiguous-array/
    Given bin array `nums`, ret maxlen of contiguous subarray with equal amount of `0` and `1`.
    runtime 14ms
*/

/* Technique
* init count to 0 with corresponding value in map
* loop through nums, everytime we see 0: count += -1
*                    everytime we see 1: count += 1
* check map if we have current value of count
*              we do  ? -> update max_len
*              we dont? -> insert count, index into map
* finish loop through nums, return max_len
*/

fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;
    let mut max_len = 0;
    map.insert(0, -1);

    for (i, &num) in nums.iter().enumerate() {
        count += if num == 1 { 1 } else { -1 };

        if let Some(prev_index) = map.get(&count) {
            max_len = max_len.max(i as i32 - prev_index);
        } else {
            map.insert(count, i as i32);
        }
    }
    max_len
}

fn main() {
    let tc = vec![0, 0, 1, 0, 0, 0, 1, 1];
    let ans = find_max_length(tc);
    println!("{ans}");
}
