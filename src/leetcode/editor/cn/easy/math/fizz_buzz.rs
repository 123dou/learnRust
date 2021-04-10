//写一个程序，输出从 1 到 n 数字的字符串表示。
//
// 1. 如果 n 是3的倍数，输出“Fizz”；
//
// 2. 如果 n 是5的倍数，输出“Buzz”；
//
// 3.如果 n 同时是3和5的倍数，输出 “FizzBuzz”。
//
// 示例：
//
// n = 15,
//
//返回:
//[
//    "1",
//    "2",
//    "Fizz",
//    "4",
//    "Buzz",
//    "Fizz",
//    "7",
//    "8",
//    "Fizz",
//    "Buzz",
//    "11",
//    "Fizz",
//    "13",
//    "14",
//    "FizzBuzz"
//]
//
// 👍 76 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_fizz_buzz() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ve = vec![];
        for i in 1..n + 1 {
            if i % 3 == 0 && i % 5 == 0 {
                ve.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                ve.push("Fizz".to_string());
            } else if i % 5 == 0 {
                ve.push("Buzz".to_string());
            } else {
                ve.push(i.to_string());
            }
        }
        ve
    }
}
//leetcode submit region end(Prohibit modification and deletion)
