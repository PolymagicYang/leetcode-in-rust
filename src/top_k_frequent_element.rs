use crate::Solution;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct FreqPair(i32, i32);

impl PartialEq for FreqPair {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for FreqPair {}

impl PartialOrd for FreqPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for FreqPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn top_k_frequent(&self, nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut heap = BinaryHeap::new();
        let mut map: HashMap<i32, FreqPair> = HashMap::new();
        let mut result = Vec::new();
        for num in nums.clone() {
            let state = map.entry(num).or_insert(FreqPair(num, 0));
            *state = FreqPair(num, state.1 + 1);
        }

        for num in map.keys() {
            heap.push(map.get(num).unwrap());
        }

        for _ in 0..k {
            let elem = match heap.pop() {
                Some(elem) => {
                    elem
                },
                None => {
                    return result;
                }
            };
            result.push(elem.0);
        }
        result
    }
} 

#[test]
fn test() {
    let solution = Solution::new();
    println!("{:?}", solution.top_k_frequent(vec![2, 2, 3, 3, 4, 5], 2));
    println!("{:?}", solution.top_k_frequent(vec![2, 2, 3, 3, 4, 5, 3, 3, 5, 7, 7], 3));
    println!("{:?}", solution.top_k_frequent(vec![], 2));
    println!("{:?}", solution.top_k_frequent(vec![2, 3, 4, 5], 10000));
}