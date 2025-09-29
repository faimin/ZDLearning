use std::cmp::max;
use std::collections::HashSet;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for item in nums.clone() {
            if item == val {
                continue;
            }
            nums[i] = item;
            i += 1;
        }
        i as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut index = 0;
        let mut pre_item = i32::MIN;
        for i in nums.clone() {
            if i == pre_item {
                continue;
            }
            nums[index] = i;
            index += 1;
            pre_item = i;
        }
        index as i32
    }

    //121
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0_i32;
        let mut min_price = prices[0];
        let mut max_price = prices[0];
        for item in prices {
            if item < min_price {
                min_price = item;
                max_price = item;
            } else if item > max_price {
                max_price = item;
            }
            max_price = max(max_price, max_price - min_price);
        }
        max_profit
    }

    //122
    pub fn max_profit_122(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            profit += max(0, prices[i] - prices[i-1]);
        }
        profit
    }
}

impl Solution {
    //575
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut cache = HashSet::new();
        let mut count = 0;
        for item in candy_type {
            cache.insert(item);
            count += 1;
        }
        (count/2).min(cache.len() as i32)
    }
}

impl Solution {
    // 206
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        while let Some(mut current) = cur {
            let next = current.next;
            current.next = pre;

            pre = Some(current);
            cur = next;
        }
        pre
    }
}

// 155
struct MinStack {
    inner_vec: Vec<(i32, i32)>,
}

impl MinStack {

    fn new() -> Self {
        MinStack {
            inner_vec: vec![]
        }
    }
    
    fn push(&mut self, val: i32) {
        let last = match self.inner_vec.last() {
            Some(a) => a.1,
            None => val
        }; 
        self.inner_vec.push((val, val.min(last)))
    }
    
    fn pop(&mut self) {
        self.inner_vec.pop();
    }
    
    fn top(&self) -> i32 {
        self.inner_vec.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.inner_vec.last().unwrap().1
    }
}