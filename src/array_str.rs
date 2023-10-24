use std::cmp::max;
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn profit(prices: Vec<i32>) -> i32 {
        let (mut r, mut l) = (1, 0); // two pointers
        let mut maxP = 0;
        while r < prices.len() {
            if prices[l] < prices[r] {
                let profit = prices[r] - prices[l];
                maxP = max(maxP, profit);
            } else {
                l = r;
            }
            r += 1;
        }
        maxP
    }

    pub fn is_valid_pallindrome(s: &String) -> bool {
        let (mut l, mut r) = (0, s.len() - 1);
        let n = s.len() as usize;
        for _ in 0..n {
            let (lc, rc) = (s.chars().nth(l).unwrap(), s.chars().nth(r).unwrap());
            if !lc.is_alphanumeric() {
                l += 1;
                continue;
            }
            if !rc.is_alphanumeric() {
                r -= 1;
                continue;
            }
            if lc.to_ascii_lowercase() != rc.to_ascii_lowercase() {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            if map.contains_key(&n) {
                return true;
            }
            map.insert(n, 1);
        }
        false
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in t.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;
        }

        for (_, v) in map {
            if v != 0 {
                return false;
            }
        }
        true
    }

    pub fn longest_pallindrome(s: String) -> i32 {
        if (s.len() == 0 || s.len() == 1) {
            return s.len() as i32;
        }
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut odd = 0;
        for (_, v) in map {
            if v % 2 == 1 {
                odd += 1;
            }
        }
        if odd == 0 {
            return s.len() as i32;
        }
        s.len() as i32 - odd + 1
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // let's go thorugh a bruteforce techniqe first
        // for num in &nums {
        //     let mut count = 0;
        //     for n in nums.clone() {
        //         if num == &n {
        //             count += 1;
        //         }
        //     }
        //     if count > nums.len() / 2 {
        //         return *num;
        //     }
        // }
        // 0

        // let's try a hashmap
        // let mut map: HashMap<i32, i32> = HashMap::new();
        // for num in &nums {
        //     let count = map.entry(*num).or_insert(0);
        //     *count += 1;
        // }
        // for (k, v) in map {
        //     if v > nums.len() as i32 / 2 {
        //         return k;
        //     }
        // }
        // 0

        // let's try O(1) space Boyer-Moore Voting Algorithm
        let mut count = 0;
        let mut candidate = 0;
        for num in &nums {
            if count == 0 {
                candidate = *num;
            }
            if candidate == *num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort();
        for (i, num) in nums.iter().enumerate() {
            if i > 0 && num == &nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = num + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    result.push(vec![*num, nums[left], nums[right]]);
                    left += 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }
        result
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1;
        nums.extend(nums2);
        nums.sort();
        let n = nums.len();
        if n % 2 == 0 {
            (nums[n / 2] + nums[n / 2 - 1]) as f64 / 2.0
        } else {
            nums[n / 2] as f64
        }
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        for c in ransom_note.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }
        true
    }

}
