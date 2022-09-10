pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for first in 0..nums.len() {
            let left = nums.get(first).unwrap().to_owned();
            for second in (first + 1)..nums.len() {
                let right = nums.get(second).unwrap().to_owned();
                if left + right == target {
                    return vec![first as i32, second as i32]
                }
            }
        }
        vec![0, 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tests = vec![
            (vec![2,7,11,15], 9, vec![0,1]),
            (vec![3,2,4], 6, vec![1,2]),
            (vec![3,3], 6, vec![0,1]),
        ];
        for (nums, target, expected) in tests {
            let actual = Solution::two_sum(nums, target);
            assert_eq!(actual, expected);
        }
    }
}
