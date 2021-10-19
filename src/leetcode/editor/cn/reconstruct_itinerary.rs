//给定一个机票的字符串二维数组 [from, to]，子数组中的两个成员分别表示飞机出发和降落的机场地点，对该行程进行重新规划排序。所有这些机票都属于一个从
//JFK（肯尼迪国际机场）出发的先生，所以该行程必须从 JFK 开始。
//
//
//
// 提示：
//
//
// 如果存在多种有效的行程，请你按字符自然排序返回最小的行程组合。例如，行程 ["JFK", "LGA"] 与 ["JFK", "LGB"] 相比就更小，排序
//更靠前
// 所有的机场都用三个大写字母表示（机场代码）。
// 假定所有机票至少存在一种合理的行程。
// 所有的机票必须都用一次 且 只能用一次。
//
//
//
//
// 示例 1：
//
//
//输入：[["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"]]
//输出：["JFK", "MUC", "LHR", "SFO", "SJC"]
//
//
// 示例 2：
//
//
//输入：[["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
//输出：["JFK","ATL","JFK","SFO","ATL","SFO"]
//解释：另一种有效的行程是 ["JFK","SFO","ATL","JFK","ATL","SFO"]。但是它自然排序更大更靠后。
// Related Topics 深度优先搜索 图
// 👍 386 👎 0

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::option::Option::Some;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_itinerary() {}
}

struct Solution {}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Rev<'a>(&'a str);
impl<'a> PartialOrd for Rev<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(self.0)
    }
}
impl<'a> Ord for Rev<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(self.0)
    }
}
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<&str, BinaryHeap<Rev>> = HashMap::new();
        tickets.iter().for_each(|e| {
            graph
                .entry(&e[0])
                .or_insert_with(BinaryHeap::new)
                .push(Rev(&e[1]));
        });
        let mut stack = vec![];
        Solution::dfs(&mut graph, &mut stack, "JFK");
        stack.reverse();
        stack
    }

    fn dfs(graph: &mut HashMap<&str, BinaryHeap<Rev>>, stack: &mut Vec<String>, from: &str) {
        // let x = graph.get_mut(from).unwrap_or(&mut BinaryHeap::new());
        // while let Some(to) = x.pop() {
        //     Solution::dfs(graph, stack, to.0);
        // }
        while let Some(to) = graph.get_mut(from).unwrap_or(&mut BinaryHeap::new()).pop() {
            Solution::dfs(graph, stack, to.0);
        }
        stack.push(from.to_owned());
    }
}
//leetcode submit region end(Prohibit modification and deletion)
