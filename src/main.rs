mod array_str;
mod dp;
mod recursion;
mod two_sum;
mod valid_paranthesis;

#[cfg(test)]
mod tests {
    use crate::array_str;
    use crate::dp;
    use crate::recursion;
    use crate::two_sum;
    use crate::valid_paranthesis;

    #[test]
    fn test_two_sum() {
        let cases = 3;
        let mut nums: Vec<Vec<i32>> = Vec::new();
        let mut targets: Vec<i32> = Vec::new();
        let mut results: Vec<Vec<i32>> = Vec::new();
        nums.push(vec![2, 7, 11, 15]);
        targets.push(9);
        nums.push(vec![3, 2, 4]);
        targets.push(6);
        nums.push(vec![3, 3]);
        targets.push(6);
        results.push(vec![0, 1]);
        results.push(vec![1, 2]);
        results.push(vec![0, 1]);
        for i in 0..cases {
            assert_eq!(
                two_sum::Solution::two_sum(nums[i].clone(), targets[i]),
                results[i]
            );
        }
    }

    fn test_two_sum_two_pass_hash() {
        let cases = 3;
        let mut nums: Vec<Vec<i32>> = Vec::new();
        let mut targets: Vec<i32> = Vec::new();
        let mut results: Vec<Vec<i32>> = Vec::new();
        nums.push(vec![2, 7, 11, 15]);
        targets.push(9);
        nums.push(vec![3, 2, 4]);
        targets.push(6);
        nums.push(vec![3, 3]);
        targets.push(6);
        results.push(vec![0, 1]);
        results.push(vec![1, 2]);
        results.push(vec![0, 1]);
        for i in 0..cases {
            assert_eq!(
                two_sum::Solution::two_sum_two_pass_hash(nums[i].clone(), targets[i]),
                results[i]
            );
        }
    }

    #[test]
    fn test_two_sum_one_pass_hash() {
        let cases = 3;
        let mut nums: Vec<Vec<i32>> = Vec::new();
        let mut targets: Vec<i32> = Vec::new();
        let mut results: Vec<Vec<i32>> = Vec::new();
        nums.push(vec![2, 7, 11, 15]);
        targets.push(9);
        nums.push(vec![3, 2, 4]);
        targets.push(6);
        nums.push(vec![3, 3]);
        targets.push(6);
        results.push(vec![0, 1]);
        results.push(vec![1, 2]);
        results.push(vec![0, 1]);
        for i in 0..cases {
            assert_eq!(
                two_sum::Solution::two_sum_one_pass_hash(nums[i].clone(), targets[i]),
                results[i]
            );
        }
    }

    #[test]
    fn test_valid_paranthesis() {
        let input = "()";
        assert_eq!(
            valid_paranthesis::Solution::is_valid(input.to_string()),
            true
        );
    }

    #[test]
    fn test_valid_paranthesis_2() {
        let input = "()[]{}";
        assert_eq!(
            valid_paranthesis::Solution::is_valid(input.to_string()),
            true
        );
    }

    #[test]
    fn test_valid_paranthesis_3() {
        let input = "(]";
        assert_eq!(
            valid_paranthesis::Solution::is_valid(input.to_string()),
            false
        );
    }

    #[test]
    fn test_valid_paranthesis_4() {
        let input = "([)]";
        assert_eq!(
            valid_paranthesis::Solution::is_valid(input.to_string()),
            false
        );
    }

    #[test]
    fn test_factorial() {
        let input = 30;
        assert_eq!(
            recursion::Solution::fact(input),
            265252859812191058636308480000000
        );
    }

    #[test]
    fn test_fibonacci() {
        let input = 10;
        assert_eq!(recursion::Solution::fib(input), 55);
    }

    #[test]
    fn test_spell_recur() {
        let input = 430198610;
        assert_eq!(
            recursion::Solution::spell_recursion(input),
            "four three zero one nine eight six one zero"
        );
    }

    #[test]
    fn test_exp() {
        let input = 5;
        let result = recursion::Solution::exp(input);
        assert_eq!(result, 32);
    }

    #[test]
    fn test_sorted_unsorted_arr_true() {
        let input: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = recursion::Solution::sorted_unsorted_check(&input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_sorted_unsorted_arr_false() {
        let input: [i32; 10] = [1, 2, 3, 6, 4, 6, 7, 8, 10, 9];
        let result = recursion::Solution::sorted_unsorted_check(&input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_subsets() {
        let input: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = recursion::Solution::subset(&input);
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_profit() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let result = array_str::Solution::profit(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_profit_2() {
        let input = vec![7, 6, 4, 3, 1];
        let result = array_str::Solution::profit(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_pallindrom() {
        let input = "A man, a plan, a canal: Panama";
        let result = array_str::Solution::is_valid_pallindrome(&input.to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn test_contain_duplicate() {
        let input = vec![1, 2, 3, 1];
        let result = array_str::Solution::contains_duplicate(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_contain_duplicate_2() {
        let input = vec![1, 2, 3, 4];
        let result = array_str::Solution::contains_duplicate(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_angram() {
        let input = "anagram".to_string();
        let input2 = "nagaram".to_string();
        let result = array_str::Solution::is_anagram(input, input2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_fib() {
        let input = 100;
        assert_eq!(dp::Solution::with_dp_fib_loop(input), 354224848179261915075);
    }

    #[test]
    fn test_climbing_stairs() {
        let input = 10;
        assert_eq!(dp::Solution::climbing_stairs(input), 89);
    }

    #[test]
    fn test_longest_pallindrome() {
        let input = "abccccdd".to_string();
        assert_eq!(array_str::Solution::longest_pallindrome(input), 7);
    }
}

fn main() {}
