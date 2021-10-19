//在本问题中，有根树指满足以下条件的 有向 图。该树只有一个根节点，所有其他节点都是该根节点的后继。该树除了根节点之外的每一个节点都有且只有一个父节点，而根节
//点没有父节点。
//
// 输入一个有向图，该图由一个有着 n 个节点（节点值不重复，从 1 到 n）的树及一条附加的有向边构成。附加的边包含在 1 到 n 中的两个不同顶点间，这条
//附加的边不属于树中已存在的边。
//
// 结果图是一个以边组成的二维数组 edges 。 每个元素是一对 [ui, vi]，用以表示 有向 图中连接顶点 ui 和顶点 vi 的边，其中 ui 是
//vi 的一个父节点。
//
// 返回一条能删除的边，使得剩下的图是有 n 个节点的有根树。若有多个答案，返回最后出现在给定二维数组的答案。
//
//
//
// 示例 1：
//
//
//输入：edges = [[1,2],[1,3],[2,3]]
//输出：[2,3]
//
//
// 示例 2：
//
//
//输入：edges = [[1,2],[2,3],[3,4],[4,1],[1,5]]
//输出：[4,1]
//
//
//
//
// 提示：
//
//
// n == edges.length
// 3 <= n <= 1000
// edges[i].length == 2
// 1 <= ui, vi <= n
//
// Related Topics 树 深度优先搜索 并查集 图
// 👍 237 👎 0

use std::process::id;

use crate::leetcode::editor::cn::graph::Digraph;
use crate::leetcode::editor::cn::union_find::UnionFind;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_redundant_connection_ii() {
        let edges = vec![vec![2, 1], vec![3, 1], vec![4, 2], vec![1, 4]];
        println!(
            "solution = {:?}",
            Solution::find_redundant_directed_connection(edges)
        );
    }
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let di_graph = Digraph::new(edges.len() + 1, &edges);
        let in_degree_eq_2 = di_graph
            .in_degree
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == 2)
            .collect::<Vec<(usize, &usize)>>();
        let set = di_graph.get_cycle();
        for i in (0..edges.len()).rev() {
            let from = edges[i][0] as usize;
            let to = edges[i][1] as usize;
            // 有环
            if !set.is_empty() {
                if set.contains(&from)
                    && set.contains(&to)
                    && di_graph.adj[from].contains(&to)
                    && (in_degree_eq_2.is_empty() || di_graph.in_degree[to] == 2)
                {
                    return edges[i].clone();
                }
                // 无环
            } else if di_graph.in_degree[to] == 2 {
                return edges[i].clone();
            }
        }
        vec![]
    }
    // 并查集，效率最高
    pub fn find_redundant_directed_connection2(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len());
        let mut parent = (0..=edges.len()).collect::<Vec<usize>>();
        let mut conflict = 0;
        let mut cycle = 0;
        for (idx, e) in edges.iter().enumerate() {
            let p = e[0] as usize;
            let q = e[1] as usize;
            if parent[q] != q {
                conflict = idx;
            } else {
                parent[q] = p;
                if !uf.is_conn(p, q) {
                    uf.union(p, q);
                } else {
                    cycle = idx;
                }
            }
        }
        if conflict != 0 {
            if cycle == 0 {
                return edges[conflict].clone();
            }
            let q = edges[conflict][1] as usize;
            return vec![parent[q] as i32, q as i32];
        }
        edges[cycle].clone()
    }
}

//leetcode submit region end(Prohibit modification and deletion)
