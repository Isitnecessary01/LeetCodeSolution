// One-Pass Hash Table Approach

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = num_map.get(&(target - num)) {
                return vec![j, i as i32];
            }
            num_map.insert(num, i as i32);
        }
        vec![]
    }
}
