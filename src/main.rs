mod array_str;
mod dp;
mod graphs;
mod recursion;
mod test_all;
mod two_sum;
mod valid_paranthesis;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::array_str;
    use crate::dp;
    use crate::graphs;
    use crate::graphs::Graph;
    use crate::graphs::Node;
    use crate::recursion;
    use crate::test_all;
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
        let result = test_all::Solution::is_anagram(input, input2);
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

    #[test]
    fn test_max_subarray() {
        let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(dp::Solution::max_subarray(input), 6);
    }

    #[test]
    fn test_majority_element() {
        let input = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(array_str::Solution::majority_element(input), 2);
    }

    #[test]
    fn test_three_sum() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            array_str::Solution::three_sum(input),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn test_flood_fill() {
        let input = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        assert_eq!(
            graphs::Solution::flood_fill(input, 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn test_update_matrix() {
        let input = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        assert_eq!(
            graphs::Solution::update_matrix(input),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }

    #[test]
    fn test_clone_graph() {
        let node = Rc::new(RefCell::new(Node::new(1)));
        node.borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(2))));
        node.borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(3))));
        let result = graphs::Solution::clone_graph(Some(node));
        let euating_node = Rc::new(RefCell::new(Node::new(1)));
        euating_node
            .borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(2))));
        euating_node
            .borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(3))));
        assert_eq!(
            result.unwrap().borrow().get_val(),
            euating_node.borrow().get_val()
        );
    }

    #[test]
    fn test_clone_graph_bfs() {
        let node = Rc::new(RefCell::new(Node::new(1)));
        node.borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(2))));
        node.borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(3))));
        let result = graphs::Solution::clone_graph_bfs(Some(node));
        let euating_node = Rc::new(RefCell::new(Node::new(1)));
        euating_node
            .borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(2))));
        euating_node
            .borrow_mut()
            .add_neighbor(Rc::new(RefCell::new(Node::new(3))));
        assert_eq!(
            result.unwrap().borrow().get_val(),
            euating_node.borrow().get_val()
        );
    }

    #[test]
    fn test_graph_bfs() {
        let mut graph: Graph<i32, i32, String> = graphs::Graph::new();
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 0, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 1);
        graph.add_edge(4, 2, 1);
        let result = graph.dfs(2);
        assert_eq!(result, vec![2, 0, 1, 3, 4]);
    }

    #[test]
    fn test_oranges_rotting() {
        let input = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(graphs::Solution::oranges_rotting(input), 4);
    }

    #[test]
    fn test_num_islands() {
        let input = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(graphs::Solution::num_islands(input), 1);
    }

    #[test]
    fn test_exist() {
        let input = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(graphs::Solution::exist(input, "ABCCED".to_string()), true);
    }

    #[test]
    fn test_topological_sort() {
        let mut graph: Graph<i32, i32> = graphs::Graph::new();
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);

        let result = graph.topological_sort();
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

}

fn main() {
}
