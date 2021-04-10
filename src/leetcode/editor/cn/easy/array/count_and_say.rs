//「外观数列」是一个整数序列，从数字 1 开始，序列中的每一项都是对前一项的描述。前五项如下：
//
// 1.     1
//2.     11
//3.     21
//4.     1211
//5.     111221
//
//
// 1 被读作 "one 1" ("一个一") , 即 11。
//11 被读作 "two 1s" ("两个一"）, 即 21。
//21 被读作 "one 2", "one 1" （"一个二" , "一个一") , 即 1211。
//
// 给定一个正整数 n（1 ≤ n ≤ 30），输出外观数列的第 n 项。
//
// 注意：整数序列中的每一项将表示为一个字符串。
//
//
//
// 示例 1:
//
// 输入: 1
//输出: "1"
//解释：这是一个基本样例。
//
// 示例 2:
//
// 输入: 4
//输出: "1211"
//解释：当 n = 3 时，序列是 "21"，其中我们有 "2" 和 "1" 两组，"2" 可以读作 "12"，也就是出现频次 = 1 而 值 = 2；类似
//"1" 可以读作 "11"。所以答案是 "12" 和 "11" 组合在一起，也就是 "1211"。
// Related Topics 字符串

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        let n = 5;
        println!("{}", Solution::count_and_say(n));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut itar = String::from("1");
        if n == 1 {
            return itar;
        }
        for _i in 2..n + 1 {
            // 迭代计算每一轮的结果
            let mut temp = "".to_string();
            let _count = 1;
            let mut j = 0;
            while j < itar.len() {
                let num = itar.get(j..j + 1).unwrap();
                let mut count = 1;
                while j != itar.len() - 1 && itar.get(j + 1..j + 2).unwrap().eq(num) {
                    count += 1;
                    j += 1;
                }
                temp += count.to_string().as_str();
                temp += num;
                j += 1;
            }
            itar = temp.clone();
        }
        return itar;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
