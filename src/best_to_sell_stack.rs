use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn max_profit(&self, prices: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        profit_helper(&mut map, 0, &prices)   
    }
}

fn profit_helper(map: &mut HashMap<usize, i32>, index: usize, prices: &[i32]) -> i32 {
    if map.contains_key(&index) {
        return map[&index];
    }
    if prices.len() == 0 || prices.len() == 1 {
        return 0;
    }
    let mut start = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i-1] {
            break 
        }
        start = i-1;
    }

    let new_prices = &prices[start..];

    let buy_day_one = new_prices[0];
    let mut max_profit = 0;
    for i in 1..new_prices.len() {
        let curr_profit = new_prices[i] - buy_day_one;
        let profit_without_day1 = profit_helper(map, index+1, &new_prices[1..]);
        map.insert(index+1, profit_without_day1);
        let profit_with_day1 = profit_helper(map, index + i + 1, &new_prices[i+1..]);
        map.insert(index+i+1, profit_with_day1);
        max_profit = std::cmp::max(std::cmp::max(profit_with_day1 + curr_profit, profit_without_day1), max_profit);
        map.insert(index, max_profit);
    } 
    return max_profit
}
#[test]
fn test() {
	let s = Solution::new();
	let r = s.max_profit(vec![7,1,5,3,6,4]);
	assert_eq!(r, 7);
}