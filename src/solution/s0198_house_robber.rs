use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return cmp::max(nums[0], nums[1]);
        }
        // 1 2 3
        // 1 2 6
        // 不是贪心
        // 尝试枚举
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use super::*;

    #[test]
    fn test_26() {
        // 1 2 3
        // 0 1 2
        // 0   2
        //   1

        // 1 2 3 4
        // 0   2
        // 0     3
        //   1   3
        // 100 
    }
}
