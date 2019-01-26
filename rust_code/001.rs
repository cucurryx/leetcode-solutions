use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_index_map = HashMap::new();
        for i in 0..nums.len() {
            let need = target - nums[i];
            match num_index_map.get(&need) {
                Some(index) => {
                    return vec![*index, i as i32];
                },
                None => {}
            };
            num_index_map.insert(nums[i], i as i32);
        }
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    for x in two_sum(nums, 9) {
        print!("{}", x);
    }
    // println!("{}", two_sum(nums, 9));
}