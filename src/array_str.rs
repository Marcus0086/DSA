use std::cmp::max;
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn profit(prices: Vec<i32>) -> i32 {
        let (mut r , mut l) = (1, 0); // two pointers
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
}

