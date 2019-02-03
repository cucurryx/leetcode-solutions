use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse_codes = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        let mut all_results: HashSet<String> = HashSet::new();

        for word in words {
            let mut curr = String::new();
            word.chars().for_each(|c| curr += morse_codes[c as usize - 'a' as usize]);
//            for c in word.chars() {
//                curr += morse_codes[c as usize - 'a' as usize];
//            }
            all_results.insert(curr);
        }

        all_results.len() as i32
    }
}

fn main() {
    let words = vec!["gin", "zen", "gig", "msg"].iter().map(|x| x.to_string()).collect();
    assert_eq!(2, Solution::unique_morse_representations(words));
}
