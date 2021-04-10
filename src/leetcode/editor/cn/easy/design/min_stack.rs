//设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。
//
//
// push(x) —— 将元素 x 推入栈中。
// pop() —— 删除栈顶的元素。
// top() —— 获取栈顶元素。
// getMin() —— 检索栈中的最小元素。
//
//
//
//
// 示例:
//
// 输入：
//["MinStack","push","push","push","getMin","pop","top","getMin"]
//[[],[-2],[0],[-3],[],[],[],[]]
//
//输出：
//[null,null,null,null,-3,null,0,-2]
//
//解释：
//MinStack minStack = new MinStack();
//minStack.push(-2);
//minStack.push(0);
//minStack.push(-3);
//minStack.getMin();   --> 返回 -3.
//minStack.pop();
//minStack.top();      --> 返回 0.
//minStack.getMin();   --> 返回 -2.
//
//
//
//
// 提示：
//
//
// pop、top 和 getMin 操作总是在 非空栈 上调用。
//
// Related Topics 栈 设计
// 👍 723 👎 0

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_min_stack() {}
}

struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if x <= self.get_min() {
            self.min_stack.push(x);
        }
    }

    fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            if val == self.get_min() {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap_or_else(|| panic!())
    }

    fn get_min(&self) -> i32 {
        if let Some(val) = self.min_stack.last() {
            *val
        } else {
            i32::MAX
        }
    }
}

//leetcode submit region end(Prohibit modification and deletion)
