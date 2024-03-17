/* Maximum Average Subarray - easy
* max average on a window of size k over i32 array nums
*/

// technique: use the windows implementation for slices
// slow runtime 294ms
fn funni(nums: Vec<i32>, k: i32) -> f64 {
    nums.windows(k as usize)
        .map(|w| w.iter().sum::<i32>())
        .max()
        .unwrap() as f64
        / k as f64
}

// technique: calculate max for current window, slide to the right
// avg runtime 18ms
fn max_avg_subarray(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize
    let mut s = nums[..k].iter().sum::<i32>();
    let mut max_sum = s;

    for i in k as usize..nums.len() {
        s += nums[i] - nums[i - k as usize];
        max_sum = max_sum.max(s);
    }
    max_sum as f64 / k as f64
}

fn main() {
    let arr = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let result = max_avg_subarray(arr.clone(), k);
    let r2 = funni(arr, k);
    println!(
        "Max average subarray of size {} is: {}, test_funni: {r2}",
        k, result
    );
}
