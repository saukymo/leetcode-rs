use std::collections::HashMap;

struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut numbers: HashMap<i32, i32> = HashMap::new();

        for (idx, value) in nums.iter().enumerate() {
            match numbers.get(&(target - value)) {
                Some(pre_idx) => {
                    return vec![i32::from(*pre_idx), idx as i32];
                },
                None => {
                    numbers.insert(i32::from(*value), idx as i32);
                }
            }
        }

        return vec![];
    }
}