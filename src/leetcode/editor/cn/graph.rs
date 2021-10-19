use std::collections::{HashSet, VecDeque};

pub struct Digraph {
    vertex: usize,
    edges: usize,
    pub adj: Vec<Vec<usize>>,
    pre: VecDeque<usize>,
    post_reverse_post: VecDeque<usize>,
    pub in_degree: Vec<usize>,
    pub out_degree: Vec<usize>,
}

impl Digraph {
    pub fn new(vertex: usize, input: &[Vec<i32>]) -> Digraph {
        let adj = vec![vec![]; vertex];
        let in_degree = vec![0; vertex];
        let out_degree = vec![0; vertex];
        let mut graph = Digraph {
            vertex,
            edges: 0,
            adj,
            pre: VecDeque::new(),
            post_reverse_post: VecDeque::new(),
            in_degree,
            out_degree,
        };
        for edge in input.iter() {
            graph.add_edges(edge[0] as usize, edge[1] as usize);
        }
        graph
    }

    pub fn add_edges(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.edges += 1;
        self.in_degree[w] += 1;
        self.out_degree[v] += 1;
    }

    pub fn has_cycle(&mut self) -> bool {
        let mut marked = vec![false; self.vertex];
        let mut on_stack = vec![false; self.vertex];
        for i in 0..self.vertex {
            if !marked[i] && self.dfs_marked(i, &mut marked, &mut on_stack) {
                return true;
            }
        }
        false
    }

    /// 获取拓扑排序
    pub fn get_to_po_list(&mut self) -> Option<Vec<usize>> {
        let mut res = Vec::with_capacity(self.vertex);
        if self.has_cycle() {
            return None;
        }
        res.extend(self.post_reverse_post.iter().rev());
        Some(res)
    }

    fn dfs_marked(&mut self, v: usize, marked: &mut [bool], on_stack: &mut [bool]) -> bool {
        marked[v] = true;
        on_stack[v] = true;
        self.pre.push_back(v);
        let adj = self.adj[v].clone();
        for &w in adj.iter() {
            if on_stack[w] {
                return true;
            }
            if !marked[w] && self.dfs_marked(w, marked, on_stack) {
                return true;
            }
        }
        on_stack[v] = false;
        self.post_reverse_post.push_back(v);
        false
    }

    pub fn get_cycle(&self) -> HashSet<usize> {
        let mut res: HashSet<usize> = HashSet::new();
        let mut path_to = (0..=self.vertex).collect::<Vec<usize>>();
        for i in 1..self.vertex {
            if !res.contains(&i) && self.find_cycle(i, &mut res, &mut path_to) {
                break;
            }
        }
        res
    }

    // 只适用与连通图
    fn find_cycle(&self, curr: usize, set: &mut HashSet<usize>, path_to: &mut Vec<usize>) -> bool {
        set.insert(curr);
        for &nt_from in self.adj[curr].iter() {
            if !set.contains(&nt_from) {
                path_to[nt_from] = curr;
                if self.find_cycle(nt_from, set, path_to) {
                    return true;
                }
            } else {
                path_to[nt_from] = nt_from;
                set.clear();
                set.insert(nt_from);
                let mut temp = curr;
                while path_to[temp] != temp {
                    set.insert(temp);
                    temp = path_to[temp];
                }
                return true;
            }
        }
        set.remove(&curr);
        false
    }
}

pub struct Graph {
    vertex: usize,
    edges: usize,
    pub adj: Vec<HashSet<usize>>,
    pre: VecDeque<usize>,
    post_reverse_post: VecDeque<usize>,
}

impl Graph {
    pub fn new(vertex: usize, input: &[Vec<i32>]) -> Graph {
        let adj = vec![HashSet::new(); vertex];
        let mut graph = Graph {
            vertex,
            edges: 0,
            adj,
            pre: VecDeque::new(),
            post_reverse_post: VecDeque::new(),
        };
        for edge in input.iter() {
            graph.add_edges(edge[1] as usize, edge[0] as usize);
        }
        graph
    }

    pub fn add_edges(&mut self, v: usize, w: usize) {
        self.adj[v].insert(w);
        self.adj[w].insert(v);
        self.edges += 1;
    }

    pub fn del_edges(&mut self, v: usize, w: usize) {
        self.adj[v].remove(&w);
        self.adj[w].remove(&v);
    }

    pub fn has_cycle(&mut self) -> bool {
        let mut marked = vec![false; self.vertex];
        let mut on_stack = vec![false; self.vertex];
        for i in 0..self.vertex {
            if !marked[i] && self.dfs_order(i, &mut marked, &mut on_stack) {
                return true;
            }
        }
        false
    }

    fn dfs_order(&mut self, v: usize, marked: &mut [bool], on_stack: &mut [bool]) -> bool {
        marked[v] = true;
        on_stack[v] = true;
        self.pre.push_back(v);
        let adj = self.adj[v].clone();
        for &w in adj.iter() {
            if on_stack[w] {
                return true;
            }
            if !marked[w] && self.dfs_order(w, marked, on_stack) {
                return true;
            }
        }
        on_stack[v] = false;
        self.post_reverse_post.push_back(v);
        false
    }

    pub fn get_cycle(&self) -> HashSet<usize> {
        let mut res: HashSet<usize> = HashSet::new();
        let mut path_to = (0..=self.vertex).collect::<Vec<usize>>();
        for i in 1..=self.vertex {
            if !res.contains(&i) && self.find_cycle(i, i, &mut res, &mut path_to) {
                break;
            }
        }
        res
    }
    // 只适用与连通图
    fn find_cycle(
        &self,
        front: usize,
        curr: usize,
        set: &mut HashSet<usize>,
        path_to: &mut Vec<usize>,
    ) -> bool {
        set.insert(curr);
        for &nt_from in self.adj[curr].iter() {
            if !set.contains(&nt_from) {
                path_to[nt_from] = curr;
                if self.find_cycle(curr, nt_from, set, path_to) {
                    return true;
                }
            } else if nt_from != front {
                path_to[nt_from] = nt_from;
                set.clear();
                set.insert(nt_from);
                let mut temp = curr;
                while path_to[temp] != temp {
                    set.insert(temp);
                    temp = path_to[temp];
                }
                return true;
            }
        }
        false
    }
}
