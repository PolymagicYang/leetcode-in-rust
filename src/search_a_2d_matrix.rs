use crate::Solution;
use std::collections::{ VecDeque, HashSet };

fn is_outbound(x: i32, y: i32, h: i32, w: i32) -> bool {
	x < 0 || x >= h || y < 0 || y >= w
}

impl Solution {
	pub fn search_matrix(&self, matrix: Vec<Vec<i32>>, target: i32) -> bool {
		if matrix.len() == 0 || matrix[0].len() == 0 {
			return false;
		}
		let mut set = HashSet::new();
		let (x, y) = (0, 0);
		let (h, w) = (matrix.len(), matrix[0].len());
		
		let mut deque = VecDeque::new();
		deque.push_back((x, y));

		while !deque.is_empty() {
			let (x, y) = deque.pop_front().unwrap();

			if !is_outbound(x, y, h as i32, w as i32) && !set.contains(&(x, y)) {
				set.insert((x, y));
				if matrix[x as usize][y as usize] == target {
					return true;
				} else {
					if matrix[x as usize][y as usize] < target {
						deque.push_back((x + 1, y));
						deque.push_back((x, y + 1));
					}
				}
			}
		};
		false
    }
}

#[test] 
fn test() {
	let solution = Solution::new();
	let s2 = solution.search_matrix(vec![vec![1, 1]], 0);
	println!("{}", s2);
	let s1 = solution.search_matrix(vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 23);
	println!("{}", s1);
}