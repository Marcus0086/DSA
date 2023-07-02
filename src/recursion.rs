use std::collections::HashMap;

pub struct Solution;

impl Solution {

    pub fn print_desc(n: i32) {

        if n <= 0 {
            return;
        }

        println!("n = {}", n);
        Solution::print_desc(n-1); // the call here is after the println which causes the print to
                                   // run before calling the next print_desc
    }

    pub fn print_asc(n: i32) {
        if n <= 0 {
            return;
        }

        Solution::print_asc(n-1); // the call here is before the println which causes the print to
                                  // run after calling the next print_asc
        println!("n = {}", n);
    }

    pub fn exp(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let result = Solution::exp(n/2);
        let ans = if n & 1 == 0 {
            result * result
        } else {
            result * result * 2
        };
        ans
    }

    pub fn fact_helper(n: u128, acc: u128) -> u128 {
        if n == 0 {
            return acc;
        }

        return Solution::fact_helper(n-1, acc * n);
    }

    pub fn fact(n: u128) -> u128 {
        Solution::fact_helper(n, 1)
    }


    pub fn fib_memo(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
        if n == 0 || n == 1 {
            return n;
        }

        if n == 2 {
            return 1;
        }

        if let Some(&result) = memo.get(&n) {
            return result
        }

        let result = Solution::fib_memo(n-1, memo) + Solution::fib_memo(n-2, memo);
        memo.insert(n, result);
        return result;
    }
    pub fn fib(n: u128) -> u128 {
        let mut memo: HashMap<u128, u128> = HashMap::new();
        Solution::fib_memo(n, &mut memo)
    }

    pub fn spell_loop(mut n: i32) -> String {
        let mut spells: HashMap<i32, String> = HashMap::new();
        spells.insert(0, "zero".to_string());
        spells.insert(1, "one".to_string());
        spells.insert(2, "two".to_string());
        spells.insert(3, "three".to_string());
        spells.insert(4, "four".to_string());
        spells.insert(5, "five".to_string());
        spells.insert(6, "six".to_string());
        spells.insert(7, "seven".to_string());
        spells.insert(8, "eight".to_string());
        spells.insert(9, "nine".to_string());
        spells.insert(10, "ten".to_string());

        let mut result: String = String::new();
        while n > 0 {
            let digit = n % 10;
            let spell = spells.get(&digit).unwrap();
            result = format!("{} {}", spell, result);
            n = n / 10;
        }
        result.trim().to_string()
    }

    pub fn spell_recur_helper(n: i32, spells: &HashMap<i32, String>, result: &mut String) {
        if n == 0 {
            return;
        }

        let digit = n % 10;
        let spell = spells.get(&digit).unwrap();
        *result = format!("{} {}", spell, result);
        Solution::spell_recur_helper(n / 10, spells, result);
    }

    pub fn spell_recursion(n: i32) -> String {
        let mut spells: HashMap<i32, String> = HashMap::new();
        spells.insert(0, "zero".to_string());
        spells.insert(1, "one".to_string());
        spells.insert(2, "two".to_string());
        spells.insert(3, "three".to_string());
        spells.insert(4, "four".to_string());
        spells.insert(5, "five".to_string());
        spells.insert(6, "six".to_string());
        spells.insert(7, "seven".to_string());
        spells.insert(8, "eight".to_string());
        spells.insert(9, "nine".to_string());
        spells.insert(10, "ten".to_string());

        let mut result: String = String::new();
        Solution::spell_recur_helper(n, &spells, &mut result);
        result.trim().to_string()
    }

    pub fn sorted_unsorted_check_helper(arr: &[i32], i: &usize) -> bool {
        if *i == arr.len() - 1 {
            return true;
        }

        if arr[*i] > arr[*i+1] {
            return false;
        }

        return Solution::sorted_unsorted_check_helper(arr, &(*i+1));
    }

    pub fn sorted_unsorted_check(arr: &[i32]) -> bool {
        let i = 0;
        Solution::sorted_unsorted_check_helper(arr, &i)
    }

    pub fn subset_helper(arr: &[i32], i: &usize, result: &mut Vec<i32>, acc: &mut Vec<Vec<i32>>) {
        if i >= &arr.len() {
            acc.push(Vec::new());
            return;
        }

        Solution::subset_helper(arr, &(*i+1), result, acc);
        for j in 0..acc.len() {
            let mut temp = acc[j].clone();
            temp.push(arr[*i]);
            acc.push(temp);
        }
    }

    pub fn subset(arr: &[i32]) -> Vec<Vec<i32>> {
        let i = 0;
        let mut result: Vec<i32> = Vec::new();
        let mut acc: Vec<Vec<i32>> = Vec::new();
        Solution::subset_helper(arr, &i, &mut result, &mut acc);
        acc
    }
}
