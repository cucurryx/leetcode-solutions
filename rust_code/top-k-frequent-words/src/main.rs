use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

struct Solution;

#[derive(PartialEq, Eq, Debug)]
struct FreqPair {
    word: String,
    freq: u32
}

impl PartialOrd for FreqPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result= other.freq.partial_cmp(&self.freq);
        match result {
            Some(Ordering::Equal) => self.word.partial_cmp(&other.word),
            _ => result
        }
    }
}

impl Ord for FreqPair {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = other.freq.cmp(&self.freq);
        if result == Ordering::Equal {
            return self.word.cmp(&other.word);
        }
        result
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_freq_map = HashMap::new();
        let mut result_vec = Vec::with_capacity(k as usize);
        let mut heap: BinaryHeap<FreqPair> = BinaryHeap::new();

        for word in &words {
            let stat = word_freq_map.entry(word).or_insert(1);
            *stat += 1;
        }

        for word in &words {
            if word_freq_map.contains_key(word) {
                let curr = FreqPair{
                    word: word.clone(),
                    freq: *word_freq_map.get(word).unwrap()
                };
                if heap.len() < k as usize {
                    heap.push(curr);
                } else {
                    if heap.peek().unwrap().gt(&curr) {
                        heap.pop();
                        heap.push(curr);
                    }
                }
                word_freq_map.remove(word);
            }
        }

        while !heap.is_empty() {
            result_vec.push(heap.peek().unwrap().word.clone());
            heap.pop();
        }
        result_vec.reverse();
        result_vec
    }
}

fn main() {
    println!("{:?}", FreqPair{word:"coding".to_string(), freq: 2}.partial_cmp(
        &FreqPair{word:"leetcode".to_string(), freq: 2}
    ));
    let test_vec1 = vec!["i", "love", "leetcode", "i", "love", "coding"].iter().map(|x| x.to_string()).collect();
    assert_eq!(Solution::top_k_frequent(test_vec1, 1), ["i", "love"]);
    let test_vec2 = vec!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"].iter().map(|x| x.to_string()).collect();
    assert_eq!(Solution::top_k_frequent(test_vec2, 4), ["the", "is", "sunny", "day"]);
    println!("ok")
}
