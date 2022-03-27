use core::num;

use crate::Solution;

impl Solution {
	pub fn three_sum_closest(&self, nums: Vec<i32>, target: i32) -> i32 {
		// three pointers.
		let mut nums = nums;
		nums.sort();

		let (mut curr, mut next, mut end) = (0, 1, nums.len() - 1);
		let mut sum_of_three = 0;
		while next <= end {
			sum_of_three = nums[curr] + nums[next] + nums[end];
			if sum_of_three > target {
				end -= 1;
			} else if sum_of_three < target {
				next += 1;
			} else {
				return target;
			}
			sum_of_three = nums[curr] + nums[next] + nums[end];
		};

		sum_of_three
    }
}

#[test]
fn test() {
	let s = Solution::new();
	let r = s.three_sum_closest(vec![2, 3, 2, 2, 10], 8);
	println!("{}", r);
}