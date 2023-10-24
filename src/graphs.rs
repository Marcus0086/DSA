use std::collections::{HashMap, HashSet, VecDeque};
use std::{cell::RefCell, rc::Rc};
pub struct Solution;

pub struct Node {
    val: i32,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: vec![],
        }
    }

    pub fn add_neighbor(&mut self, neighbor: Rc<RefCell<Node>>) {
        self.neighbors.push(neighbor);
    }

    pub fn get_neighbors(&self) -> Vec<Rc<RefCell<Node>>> {
        self.neighbors.clone()
    }

    pub fn get_val(&self) -> i32 {
        self.val
    }
}

impl Solution {
    pub fn dfs_flood_fill(
        mut image: &mut Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        color: &i32,
        prev_color: &i32,
    ) -> Vec<Vec<i32>> {
        if sr < 0
            || sr >= image.len() as i32
            || sc < 0
            || sc >= image[0].len() as i32
            || image[sr as usize][sc as usize] != *prev_color
        {
            return image.to_vec();
        }
        image[sr as usize][sc as usize] = *color;
        Solution::dfs_flood_fill(&mut image, sr - 1, sc, color, prev_color);
        Solution::dfs_flood_fill(&mut image, sr + 1, sc, color, prev_color);
        Solution::dfs_flood_fill(&mut image, sr, sc - 1, color, prev_color);
        Solution::dfs_flood_fill(&mut image, sr, sc + 1, color, prev_color);
        image.to_vec()
    }

    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let prev_color = image[sr as usize][sc as usize];
        if prev_color != color {
            return Solution::dfs_flood_fill(&mut image.to_vec(), sr, sc, &color, &prev_color);
        }
        image
    }

    pub fn bfs_flood_fill(
        image: &mut Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        color: &i32,
        prev_color: &i32,
    ) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        queue.push_back((sr, sc));

        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            if r < 0
                || r >= image.len() as i32
                || c < 0
                || c >= image[0].len() as i32
                || image[r as usize][c as usize] != *prev_color
            {
                continue;
            }
            image[r as usize][c as usize] = *color;
            queue.push_back((r - 0, c));
            queue.push_back((r + 1, c));
            queue.push_back((r, c - 1));
            queue.push_back((r, c + 1));
        }
        image.to_vec()
    }

    pub fn flood_fill_bfs(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let prev_color = image[sr as usize][sc as usize];
        if prev_color != color {
            return Solution::bfs_flood_fill(&mut image.to_vec(), sr, sc, &color, &prev_color);
        }
        image
    }

    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut result = vec![vec![0; mat[0].len()]; mat.len()];

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    queue.push_back((i, j));
                } else {
                    result[i][j] = -1;
                }
            }
        }

        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            if r > 0 && result[r - 1][c] == -1 {
                result[r - 1][c] = result[r][c] + 1;
                queue.push_back((r - 1, c));
            }
            if r < mat.len() - 1 && result[r + 1][c] == -1 {
                result[r + 1][c] = result[r][c] + 1;
                queue.push_back((r + 1, c));
            }
            if c > 0 && result[r][c - 1] == -1 {
                result[r][c - 1] = result[r][c] + 1;
                queue.push_back((r, c - 1));
            }
            if c < mat[0].len() - 1 && result[r][c + 1] == -1 {
                result[r][c + 1] = result[r][c] + 1;
                queue.push_back((r, c + 1));
            }
        }
        result
    }

    pub fn nearest_one(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut result = vec![vec![0; mat[0].len()]; mat.len()];

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 {
                    queue.push_back((i, j));
                } else {
                    result[i][j] = -1;
                }
            }
        }

        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            if r > 0 && result[r - 1][c] == -1 {
                result[r - 1][c] = result[r][c] + 1;
                queue.push_back((r - 1, c));
            }
            if r < mat.len() - 1 && result[r + 1][c] == -1 {
                result[r + 1][c] = result[r][c] + 1;
                queue.push_back((r + 1, c));
            }
            if c > 0 && result[r][c - 1] == -1 {
                result[r][c - 1] = result[r][c] + 1;
                queue.push_back((r, c - 1));
            }
            if c < mat[0].len() - 1 && result[r][c + 1] == -1 {
                result[r][c + 1] = result[r][c] + 1;
                queue.push_back((r, c + 1));
            }
        }
        result
    }

    pub fn dfs_clone_graph(
        node: Option<Rc<RefCell<Node>>>,
        mut map: &mut HashMap<i32, Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        if node.is_none() {
            return None;
        }

        let node = node.unwrap();
        let new_node = Rc::new(RefCell::new(Node::new(node.borrow().val)));

        map.insert(new_node.borrow().val, new_node.clone());
        for neighbour in node.borrow().neighbors.iter() {
            if let Some(neighbour_node) = map.get(&neighbour.borrow().val) {
                new_node.borrow_mut().add_neighbor(neighbour_node.clone());
            } else {
                new_node.borrow_mut().add_neighbor(
                    Solution::dfs_clone_graph(Some(neighbour.clone()), &mut map).unwrap(),
                )
            }
        }
        Some(new_node)
    }

    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut map: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
        Solution::dfs_clone_graph(node, &mut map)
    }

    pub fn clone_graph_bfs(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut map: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
        let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();

        if node.is_none() {
            return None;
        }

        let node = node.unwrap();
        let new_node = Rc::new(RefCell::new(Node::new(node.borrow().val)));

        map.insert(new_node.borrow().val, new_node.clone());
        queue.push_back(node.clone());

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            for neighbour in node.borrow().neighbors.iter() {
                if let Some(neighbour_node) = map.get(&neighbour.borrow().val) {
                    new_node.borrow_mut().add_neighbor(neighbour_node.clone());
                } else {
                    let new_neighbour_node =
                        Rc::new(RefCell::new(Node::new(neighbour.borrow().val)));
                    map.insert(new_neighbour_node.borrow().val, new_neighbour_node.clone());
                    new_node
                        .borrow_mut()
                        .add_neighbor(new_neighbour_node.clone());
                    queue.push_back(neighbour.clone());
                }
            }
        }
        Some(new_node)
    }

    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut grid = grid;
        let mut minutes = 0;
        let mut fresh_oranges = 0;
        let mut rotten_oranges = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    queue.push_back((i, j));
                    rotten_oranges += 1;
                } else if grid[i][j] == 1 {
                    fresh_oranges += 1;
                }
            }
        }

        if fresh_oranges == 0 {
            return 0;
        }

        if rotten_oranges == 0 {
            return -1;
        }

        let mut new_rotten_oranges = 0;
        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            rotten_oranges -= 1;
            let (above, left, below, right) = (
                r > 0 && grid[r - 1][c] == 1,
                c > 0 && grid[r][c - 1] == 1,
                r < grid.len() - 1 && grid[r + 1][c] == 1,
                c < grid[0].len() - 1 && grid[r][c + 1] == 1,
            );
            if above {
                grid[r - 1][c] = 2;
                queue.push_back((r - 1, c));
                new_rotten_oranges += 1;
                fresh_oranges -= 1;
            }
            if left {
                grid[r][c - 1] = 2;
                queue.push_back((r, c - 1));
                new_rotten_oranges += 1;
                fresh_oranges -= 1;
            }
            if below {
                grid[r + 1][c] = 2;
                queue.push_back((r + 1, c));
                new_rotten_oranges += 1;
                fresh_oranges -= 1;
            }
            if right {
                grid[r][c + 1] = 2;
                queue.push_back((r, c + 1));
                new_rotten_oranges += 1;
                fresh_oranges -= 1;
            }
            if rotten_oranges == 0 {
                minutes += 1;
                rotten_oranges = new_rotten_oranges;
                new_rotten_oranges = 0;
            }
        }
        if fresh_oranges > 0 {
            return -1;
        }
        minutes - 1
    }

    pub fn num_islands_dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i < grid.len() && j < grid[0].len() && grid[i][j] == '1' {
            grid[i][j] = '0';
            Self::num_islands_dfs(grid, i + 1, j);
            Self::num_islands_dfs(grid, i - 1, j);
            Self::num_islands_dfs(grid, i, j + 1);
            Self::num_islands_dfs(grid, i, j - 1);
        }
        return;
    }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut islands = 0;
        if grid.len() == 0 {
            return 0;
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    Solution::num_islands_dfs(&mut grid, i, j);
                    islands += 1;
                }
            }
        }
        islands
    }

    pub fn exist_dfs(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        i: usize,
        j: usize,
        k: usize,
    ) -> bool {
        if i >= board.len() || j >= board[0].len() || board[i][j] != word[k] {
            return false;
        }

        if k == word.len() - 1 {
            return true;
        }

        board[i][j] = '#';
        let exist = Solution::exist_dfs(board, word, i + 1, j, k + 1)
            || Solution::exist_dfs(board, word, i - 1, j, k + 1)
            || Solution::exist_dfs(board, word, i, j + 1, k + 1)
            || Solution::exist_dfs(board, word, i, j - 1, k + 1);

        board[i][j] = word[k];
        exist
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let word = word.chars().collect::<Vec<char>>();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Solution::exist_dfs(&mut board, &word, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }

    pub fn can_finish_dfs(
        graph: &HashMap<i32, Vec<i32>>,
        visited: &mut Vec<bool>,
        memo: &mut HashMap<i32, bool>,
        course: i32,
    ) -> bool {
        if let Some(&result) = memo.get(&course) {
            return result;
        }
        if visited[course as usize] {
            memo.insert(course, false);
            return false;
        }

        if graph.get(&course).is_none() {
            return true;
        }

        visited[course as usize] = true;
        if let Some(prereq) = graph.get(&course) {
            for course_preqreq in prereq {
                if !Solution::can_finish_dfs(graph, visited, memo, *course_preqreq) {
                    memo.insert(course, false);
                    return false;
                }
            }
        }
        visited[course as usize] = false;
        memo.insert(course, true);
        true
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = HashMap::new();
        for prereq in prerequisites {
            graph.entry(prereq[0]).or_insert(vec![]).push(prereq[1]);
        }

        let mut visited = vec![false; num_courses as usize];
        let mut memo: HashMap<i32, bool> = HashMap::new();
        for course in 0..num_courses {
            if !visited[course as usize]
                && !Solution::can_finish_dfs(&graph, &mut visited, &mut memo, course)
            {
                return false;
            }
        }
        true
    }

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut in_degree, decendants) = prerequisites.into_iter().fold(
            (vec![0; num_courses as usize], vec![vec![]; num_courses as usize]),
            |(mut in_degree, mut decendants), prereq| {
                in_degree[prereq[0] as usize] += 1;
                decendants[prereq[1] as usize].push(prereq[0]);
                (in_degree, decendants)
            },
        );

        let mut queue: VecDeque<i32> = in_degree.iter().enumerate()
            .filter(|(_, &val)| val == 0)
            .map(|(i, _)| i as i32)
            .collect();
        let mut result = vec![];

        while let Some(node) = queue.pop_front() {
            result.push(node);
            for &decendant in decendants[node as usize].iter() {
                in_degree[decendant as usize] -= 1;
                if in_degree[decendant as usize] == 0 {
                    queue.push_back(decendant);
                }
            }
        }

        if result.len() == num_courses as usize {
            result
        } else {
            vec![]
        }
    }

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }

        let mut word_list = word_list;
        let mut word_list_set: HashMap<String, Vec<String>> = HashMap::new();
        word_list.insert(0, begin_word.clone());

        for word in word_list.iter() {
            for i in 0..word.len() {
                let mut chars = word.chars().collect::<Vec<char>>();
                chars[i] = '*';
                let new_word = chars.iter().collect::<String>();
                word_list_set
                    .entry(new_word)
                    .or_insert(vec![])
                    .push(word.clone());
            }
        }
        
        let mut visited: HashSet<String> = HashSet::new();

        let mut queue: VecDeque<(String, i32)> = VecDeque::new();

        queue.push_back((begin_word, 1));

        while !queue.is_empty() {
            let (word, level) = queue.pop_front().unwrap();
            for i in 0..word.len() {
                let mut chars = word.chars().collect::<Vec<char>>();
                chars[i] = '*';
                let new_word = chars.iter().collect::<String>();
                if let Some(neighbours) = word_list_set.get(&new_word) {
                    for neighbour in neighbours {
                        if visited.contains(neighbour) {
                            continue;
                        }
                        visited.insert(neighbour.clone());
                        if neighbour == &end_word {
                            return level;
                        }
                        queue.push_back((neighbour.clone(), level + 1));
                    }
                }
            }

        }
        0
    }
}

#[derive(Debug)]
pub struct Graph<VID, Edge = (), Vertices = ()> {
    vertices: HashMap<VID, Vertices>,
    edges: HashMap<VID, Vec<(VID, Edge)>>,
}
impl<
        VID: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Debug,
        Edge: std::fmt::Debug,
        Vertices: std::fmt::Debug,
    > Graph<VID, Edge, Vertices>
{
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, id: VID, vertices: Vertices) {
        self.vertices.insert(id, vertices);
    }

    pub fn add_edge(&mut self, from: VID, to: VID, edge: Edge) {
        self.edges.entry(from).or_insert(vec![]).push((to, edge));
    }

    pub fn get_edges(&self) -> &HashMap<VID, Vec<(VID, Edge)>> {
        &self.edges
    }

    pub fn bfs(&self, start: VID) -> Vec<VID> {
        let mut visited: HashSet<VID> = HashSet::new();
        let mut queue: VecDeque<VID> = VecDeque::new();
        let mut result: Vec<VID> = vec![];

        queue.push_back(start);
        visited.insert(start);

        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();
            result.push(vertex);
            if let Some(neighbours) = self.edges.get(&vertex) {
                for (neighbour, _) in neighbours {
                    if !visited.contains(neighbour) {
                        queue.push_back(*neighbour);
                        visited.insert(*neighbour);
                    }
                }
            }
        }
        result
    }

    pub fn dfs_helper(&self, vertex: VID, visited: &mut HashSet<VID>, result: &mut Vec<VID>) {
        visited.insert(vertex);
        result.push(vertex);
        if let Some(neighbours) = self.edges.get(&vertex) {
            for (neighbour, _) in neighbours {
                if !visited.contains(neighbour) {
                    self.dfs_helper(*neighbour, visited, result);
                }
            }
        }
    }

    pub fn dfs(&self, start: VID) -> Vec<VID> {
        let mut visited: HashSet<VID> = HashSet::new();
        let mut result: Vec<VID> = vec![];
        self.dfs_helper(start, &mut visited, &mut result);
        result
    }

    pub fn topological_dfs(
        graph: &Graph<VID, Edge, Vertices>,
        vertex: VID,
        visited: &mut HashSet<VID>,
        stack: &mut Vec<VID>,
    ) {
        visited.insert(vertex);
        if let Some(edges) = graph.edges.get(&vertex) {
            for (edge, _) in edges {
                if !visited.contains(edge) {
                    Self::topological_dfs(graph, *edge, visited, stack);
                }
            }
        }
        stack.push(vertex);
    }

    pub fn topological_sort(&self) -> Vec<VID> {
        let mut visited: HashSet<VID> = HashSet::new();
        let mut stack: Vec<VID> = vec![];
        println!("{:?}", self);

        for vertex in self.get_edges().keys() {
            if !visited.contains(vertex) {
                Self::topological_dfs(self, *vertex, &mut visited, &mut stack);
            }
        }
        stack.reverse();
        stack
    }
}
