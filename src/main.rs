mod two_sum;

use two_sum::Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = super::Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}

fn main() {
    println!("Hello, world!");
}
