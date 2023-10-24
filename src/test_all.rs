use std::cmp::max;
use std::collections::{HashMap, VecDeque};
// include hashSet
use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let compelment = target - num;
            if let Some(&val) = map.get(&compelment) {
                return vec![val, i as i32];
            }
            map.insert(*num, i as i32);
        }
        vec![]
    }

    pub fn isValid(s: String) -> bool {
        let mut res = "".to_string();
        for character in s.chars() {
            match character {
                '(' => res.push(')'),
                '{' => res.push('}'),
                '[' => res.push(']'),
                ')' | '}' | ']' => {
                    if let Some(last_char) = res.pop() {
                        if last_char != character {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        res.len() == 0
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_p = 0;
        let (mut l, mut r) = (0, 1);
        while r < prices.len() {
            if prices[l] < prices[r] {
                let profit = prices[r] - prices[l];
                max_p = max(max_p, profit);
            } else {
                l = r;
            }
            r += 1;
        }
        max_p
    }

    pub fn is_palindrome(s: String) -> bool {
        let (mut l, mut r) = (0, s.len() - 1);
        for _ in 0..s.len() {
            let (lc, rc) = (s.chars().nth(l).unwrap(), s.chars().nth(r).unwrap());
            if !lc.is_alphanumeric() {
                l += 1;
                continue;
            }
            if !rc.is_alphanumeric() {
                r -= 1;
                continue;
            }
            if lc.to_ascii_lowercase() != rc.to_ascii_uppercase() {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map: HashMap<char, i32> = HashMap::new();
        for character in s.chars() {
            let char_count = map.entry(character).or_insert(0);
            *char_count += 1;
        }
        for character in t.chars() {
            let char_count = map.entry(character).or_insert(0);
            *char_count -= 1;
        }
        for (_, &val) in map.iter() {
            if val != 0 {
                return false;
            }
        }
        true
    }

    pub fn bfs_flood_fill(
        image: &mut Vec<Vec<i32>>,
        sr: &i32,
        sc: &i32,
        color: &i32,
        prev_color: &i32,
    ) -> Vec<Vec<i32>> {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let (sr, sc) = (*sr as usize, *sc as usize);
        queue.push_back((sr, sc));
        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            if r >= image.len() || c >= image[0].len() || image[r][c] != *prev_color {
                continue;
            }
            image[r][c] = *color;
            queue.push_back((r - 1, c));
            queue.push_back((r + 1, c));
            queue.push_back((r, c + 1));
            queue.push_back((r, c - 1));
        }
        image.to_vec()
    }

    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let prev_color = image[sr as usize][sc as usize];
        let mut image = image.to_vec();
        if prev_color != color {
            return Solution::bfs_flood_fill(&mut image, &sr, &sc, &color, &prev_color);
        }
        image
    }

    pub fn can_constrct(ransom: String, magazine: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in magazine.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in ransom.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }
        true
    }

    pub fn longest_pallindrome(s: String) -> i32 {
        if (s.len() == 0) | (s.len() == 1) {
            return s.len() as i32;
        }

        let mut map: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut odd = 0;
        let n = s.len();
        for (_, &freq) in map.iter() {
            if freq % 2 != 0 {
                odd += 1;
            }
        }
        if odd <= 0 {
            return s.len() as i32;
        }

        n as i32 - odd + 1
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // we will use khan's algorith to solve
        let (mut in_degree, decendants) = prerequisites.into_iter().fold(
            (vec![0; num_courses as usize], vec![vec![]; num_courses as usize]),
            |mut acc, val| {
                acc.0[val[0] as usize] += 1;
                acc.1[val[1] as usize].push(val[0]);
                acc
            }
        );

        let mut queue: VecDeque<i32> = in_degree
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == 0)
            .map(|(i, _)| i as i32)
            .collect();

        let mut count = 0;
        while let Some(node) = queue.pop_front() {
            count += 1;
            for &val in decendants[node as usize].iter() {
                in_degree[val as usize] -= 1;
                if in_degree[val as usize] == 0 {
                    queue.push_back(val);
                }
            }
        }
        count == num_courses
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let (mut graph, mut email_to_name) = accounts.into_iter().fold(
            (HashMap::new(), HashMap::new()),
            |mut acc, val| {
                let name = val[0].clone();
                let emails = val[1..].to_vec();
                for email in emails.iter() {
                    acc.0.entry(email.clone())
                        .or_insert(vec![])
                        .push(emails[0].clone());
                    acc.1.insert(email.clone(), name.clone());
                }
                acc
            },
        );
        println!("{:?}", graph);

        let mut res: Vec<Vec<String>> = vec![];
        let mut seen: HashSet<String> = HashSet::new();

        for email in graph.keys() {
            if seen.contains(email) {
                continue;
            }
            seen.insert(email.clone());
            let mut stack: Vec<String> = vec![email.clone()];
            let mut component: Vec<String> = vec![];
            while let Some(node) = stack.pop() {
                component.push(node.clone());
                for neighbor in graph.get(&node).unwrap() {
                    if !seen.contains(neighbor) {
                        seen.insert(neighbor.clone());
                        stack.push(neighbor.clone());
                    }
                }
            }
            component.sort();
            component.insert(0, email_to_name.get(email).unwrap().clone());
            res.push(component);
        }
        res
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_can_construct() {
        assert_eq!(
            super::Solution::can_constrct("aa".to_string(), "aab".to_string()),
            true
        );
        assert_eq!(
            super::Solution::can_constrct("aa".to_string(), "ab".to_string()),
            false
        );
    }

    #[test]
    fn test_longest_pallindrome() {
        assert_eq!(
            super::Solution::longest_pallindrome("abccccdd".to_string()),
            7
        );
        assert_eq!(super::Solution::longest_pallindrome("a".to_string()), 1);
        assert_eq!(super::Solution::longest_pallindrome("bb".to_string()), 2);
    }

    #[test]
    fn test_can_finish() {
        assert_eq!(
            super::Solution::can_finish(2, vec![vec![1, 0]]),
            true
        );

        assert_eq!(
            super::Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]),
            false
        );
    }
}
