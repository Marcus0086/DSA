pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = String::new();
        for characters in s.chars() {
            match characters {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    if let Some(last_char) = stack.pop() {
                        if last_char != characters {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.len() == 0
    }
}
