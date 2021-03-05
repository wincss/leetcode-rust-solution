#[allow(dead_code)]
pub struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
impl MyQueue {
    /** Initialize your data structure here. */
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            stack_in: vec![],
            stack_out: vec![],
        }
    }

    /** Push element x to the back of queue. */
    #[allow(dead_code)]
    pub fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    #[allow(dead_code)]
    pub fn pop(&mut self) -> i32 {
        if let Some(v) = self.stack_out.pop() {
            return v;
        }
        while let Some(v) = self.stack_in.pop() {
            self.stack_out.push(v);
        }
        self.stack_out.pop().unwrap()
    }

    /** Get the front element. */
    #[allow(dead_code)]
    pub fn peek(&mut self) -> i32 {
        if let Some(&v) = self.stack_out.last() {
            return v;
        }
        while let Some(v) = self.stack_in.pop() {
            self.stack_out.push(v);
        }
        *self.stack_out.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    #[allow(dead_code)]
    pub fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}
