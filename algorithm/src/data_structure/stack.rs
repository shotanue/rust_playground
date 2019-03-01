#[derive(Debug, Eq, PartialEq)]
pub struct Stack<T> {
    top: usize,
    item: Vec<T>,
}

pub fn new<T>(size: usize) -> Stack<T> {
    Stack {
        top: 0,
        item: Vec::with_capacity(size),
    }
}

impl<T> Stack<T> {
    pub fn push(&mut self, item: T) {
        self.top += 1;
        self.item.push(item);
    }
    pub fn pop(&mut self) -> T {
        self.top -= 1;
        self.item.pop().unwrap()
    }
}


#[cfg(test)]
mod test {
    use crate::data_structure::stack;

    #[test]
    fn new_test() {
        let stack = stack::new::<usize>(1000);
        assert_eq!(stack, stack::Stack { top: 0, item: vec![] });
    }

    #[test]
    fn push_test() {
        let mut stack = stack::new::<usize>(1000);
//        assert_eq!(stack, stack::Stack { top: 0, item: vec![] });
        stack.push(1);
        assert_eq!(stack, stack::Stack { top: 1, item: vec![1] });
        stack.push(2);
        assert_eq!(stack, stack::Stack { top: 2, item: vec![1, 2] });

        let item = stack.pop();
        assert_eq!(stack, stack::Stack { top: 1, item: vec![1] });
        assert_eq!(item, 2);
    }
}