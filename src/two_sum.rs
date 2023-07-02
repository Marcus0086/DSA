use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    result.push(i as i32);
                    result.push(j as i32);
                    break;
                }
            }
        }
        result
    }

    pub fn two_sum_two_pass_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            map.insert(*num, i as i32);
        }
        println!("{:?}", map);
        let mut result: Vec<i32> = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                if index != i as i32 {
                    result = vec![i as i32, index];
                    break;
                }
            }
        }
        result
    }

    pub fn two_sum_one_pass_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = complements.get(&complement) {
                return vec![index, i as i32];
            }
            complements.insert(*num, i as i32);
        }
        vec![]
    }
}
