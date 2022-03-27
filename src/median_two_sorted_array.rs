use crate::Solution;

impl Solution {
	pub fn find_min(&self, nums: Vec<i32>) -> i32 {
		if nums.len() == 1 {
			return nums[0];
		}
		let (mut left, mut right) = (0, nums.len() - 1);

		while left < right {
			let mid = (left + right) / 2;

			if nums[left] < nums[right] {
				return nums[left];
			}	

			if nums[mid] < nums[left] {
				right = mid;
			} else {
				left = mid + 1;
			}
		};
		nums[left]
    } 
}

#[test]
fn test() {
	let s = Solution::new();
	println!("{}", s.find_min(vec![3, 4, 6]));
}