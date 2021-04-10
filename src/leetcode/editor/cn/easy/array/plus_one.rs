//给定一个由整数组成的非空数组所表示的非负整数，在该数的基础上加一。
//
// 最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。
//
// 你可以假设除了整数 0 之外，这个整数不会以零开头。
//
// 示例 1:
//
// 输入: [1,2,3]
//输出: [1,2,4]
//解释: 输入数组表示数字 123。
//
//
// 示例 2:
//
// 输入: [4,3,2,1]
//输出: [4,3,2,2]
//解释: 输入数组表示数字 4321。
//
// Related Topics 数组
// 👍 496 👎 0

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let v = vec![8, 9, 9, 9];
        eprintln!("v = {:?}", Solution::plus_one(v));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut v = digits.clone();
        let mut count = true;
        for i in (0..v.len()).rev() {
            if count {
                v[i] += 1;
                if v[i] == 10 {
                    v[i] = 0;
                } else {
                    count = false;
                }
            }
        }
        if count {
            v.insert(0, 1);
        }
        return v;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
