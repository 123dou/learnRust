//设计一个支持在平均 时间复杂度 O(1) 下，执行以下操作的数据结构。
//
//
// insert(val)：当元素 val 不存在时，向集合中插入该项。
// remove(val)：元素 val 存在时，从集合中移除该项。
// getRandom：随机返回现有集合中的一项。每个元素应该有相同的概率被返回。
//
//
// 示例 :
//
//
//// 初始化一个空的集合。
//RandomizedSet randomSet = new RandomizedSet();
//
//// 向集合中插入 1 。返回 true 表示 1 被成功地插入。
//randomSet.insert(1);
//
//// 返回 false ，表示集合中不存在 2 。
//randomSet.remove(2);
//
//// 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
//randomSet.insert(2);
//
//// getRandom 应随机返回 1 或 2 。
//randomSet.getRandom();
//
//// 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
//randomSet.remove(1);
//
//// 2 已在集合中，所以返回 false 。
//randomSet.insert(2);
//
//// 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
//randomSet.getRandom();
//
// Related Topics 设计 数组 哈希表
// 👍 268 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_insert_delete_getrandom_o1() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
struct RandomizedSet {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedSet {}
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&self, _val: i32) -> bool {
        true
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&self, _val: i32) -> bool {
        true
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        0
    }
}

//leetcode submit region end(Prohibit modification and deletion)
