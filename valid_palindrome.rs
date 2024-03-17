/* Valid Palindrome - easy
* string s, do not consider non alphanumeric characters.
* runtime 0ms
*/

/* technique
* 1. remove non alphanumeric characters and convert to lowercase
* 2. compare the string with its reverse
*/

fn ispal(s: &str) -> bool {
    let s_lower: String = s.replace(|c: char| !c.is_alphanumeric(), "").to_lowercase();
    s_lower.chars().eq(s_lower.chars().rev())
}

fn main() {
    let s: &str = "A man, a plan, a canal, Panama";
    println!("{}", ispal(s));
}
