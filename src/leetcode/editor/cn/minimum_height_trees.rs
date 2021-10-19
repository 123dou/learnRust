//树是一个无向图，其中任何两个顶点只通过一条路径连接。 换句话说，一个任何没有简单环路的连通图都是一棵树。
//
// 给你一棵包含 n 个节点的树，标记为 0 到 n - 1 。给定数字 n 和一个有 n - 1 条无向边的 edges 列表（每一个边都是一对标签），其中
// edges[i] = [ai, bi] 表示树中节点 ai 和 bi 之间存在一条无向边。
//
// 可选择树中任何一个节点作为根。当选择节点 x 作为根节点时，设结果树的高度为 h 。在所有可能的树中，具有最小高度的树（即，min(h)）被称为 最小高度
//树 。
//
// 请你找到所有的 最小高度树 并按 任意顺序 返回它们的根节点标签列表。
//树的 高度 是指根节点和叶子节点之间最长向下路径上边的数量。
//
//
//
// 示例 1：
//
//
//输入：n = 4, edges = [[1,0],[1,2],[1,3]]
//输出：[1]
//解释：如图所示，当根是标签为 1 的节点时，树的高度是 1 ，这是唯一的最小高度树。
//
// 示例 2：
//
//
//输入：n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
//输出：[3,4]
//
//
// 示例 3：
//
//
//输入：n = 1, edges = []
//输出：[0]
//
//
// 示例 4：
//
//
//输入：n = 2, edges = [[0,1]]
//输出：[0,1]
//
//
//
//
//
//
//
// 提示：
//
//
// 1 <= n <= 2 * 104
// edges.length == n - 1
// 0 <= ai, bi < n
// ai != bi
// 所有 (ai, bi) 互不相同
// 给定的输入 保证 是一棵树，并且 不会有重复的边
//
// Related Topics 广度优先搜索 图
// 👍 321 👎 0

use std::collections::VecDeque;

use crate::leetcode::editor::cn::graph::Graph;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_minimum_height_trees() {
        let num = 6;
        let arr = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        println!("solution = {:?}", Solution::find_min_height_trees(num, arr));
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    /**
    证明思路：
    对当前的图(初始的图或者删掉了前几层叶子节点的图)，
    我们要找的满足题设的根节点所在位置只有两种可能，
    一种在当前图的叶子节点(即度为1的节点)，一种为内部节点，
    若选择某叶子节点1，则该叶子节点1与任意其他叶子节点2的距离必定为:叶子1-内部节点x-叶子2，深度为这三段边之和，
    必大于等于max(内部x-叶子1，内部x-叶子2)，所以相比于叶子节点，解空间只能出现在内部节点，
    每层情况都是这样，所以我们要剥开叶子节点直到再无可分的内部节点为止。
    */
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut graph = Graph::new(n as usize, &edges);
        let mut que = VecDeque::new();
        for i in 0..graph.adj.len() {
            if graph.adj[i].len() == 1 {
                que.push_back(i);
            }
        }
        let mut res: Vec<usize> = vec![];
        while !que.is_empty() {
            res = Vec::with_capacity(que.len());
            res.extend(que.iter());
            println!("res = {:?}", res);
            que.clear();
            for &v in res.iter() {
                if !graph.adj[v].is_empty() {
                    let w = *graph.adj[v].iter().next().unwrap();
                    println!("w = {:?}", w);
                    graph.del_edges(v, w);
                    if graph.adj[w].len() == 1 {
                        que.push_back(w);
                    }
                }
            }
        }
        res.iter().map(|&val| val as i32).collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
