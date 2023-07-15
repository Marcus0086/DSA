use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn without_dp_fib(n: i32) -> i32 {
        // with loops
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut prev = 0;
        let mut curr = 1;
        for _ in 2..=n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        curr
    }

    pub fn with_dp_fib_loop(n: u128) -> u128 {
        let mut dp: Vec<u128> = vec![0; (n + 1) as usize];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;
        for i in 3..=n {
            dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
        }
        dp[n as usize]
    }

    pub fn with_rec_fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        Solution::with_rec_fib(n - 1) + Solution::with_rec_fib(n - 2)
    }

    pub fn dp_rec_fib_helper(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if let Some(&val) = map.get(&n) {
            return val;
        }
        let result = Solution::dp_rec_fib_helper(n - 1, map) + Solution::dp_rec_fib_helper(n - 2, map);
        map.insert(n, result);
        result
    }

    pub fn dp_rec_fib(n: u64) -> u64 {
        let mut map: HashMap<u64, u64> = HashMap::new();
        Solution::dp_rec_fib_helper(n, &mut map)
    }

    pub fn climbing_stairs(n: i32) -> i32 {
        let (mut prev, mut curr) = (1, 1);
        for _ in 2..=n {
            let temp = curr;
            curr += prev;
            prev = temp;
        }
        curr
    }
}
