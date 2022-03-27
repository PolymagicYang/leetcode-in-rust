use crate::Solution;

impl Solution {
    pub fn max_sub_array(&self, nums: Vec<i32>) -> i32 {
		// [-1, 2, 3, 5, -2] => contiguous subarray. 
		if nums.len() == 0 {
			return 0;
		}
		if nums.len() == 1 {
			return nums[1];
		}
		let mut curr = nums[0];
		let mut maxinum = curr;
		let (mut start, mut end) = (0 as usize, 1 as usize);

		loop {
			curr += nums[end];
			if curr > maxinum {
				maxinum = curr;
				end += 1;
				continue;
			}
			if nums[end] < 0 {
				start += 1;
				curr -= nums[end];
			}
			end += 1;
		}
    }
}