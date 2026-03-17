use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    next_node: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    count: usize,
}
impl LinkedList {
    pub fn new() -> Self {
        return Self {
            head: None,
            tail: None,
            count: 0,
        };
    }
}
impl LinkedList {
    pub fn add_last(&mut self, item: i32) {
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next_node: None,
        }));

        if self.head.is_none() {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        } else {
            if let Some(tail) = &self.tail {
                tail.borrow_mut().next_node = Some(Rc::clone(&node));
            }
            self.tail = Some(node);
        }

        self.count += 1;
    }

    pub fn add_first(&mut self, item: i32) {
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next_node: None,
        }));

        if self.head.is_none() {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        } else {
            node.borrow_mut().next_node = self.head.clone();
            self.head = Some(Rc::clone(&node));
        }

        self.count += 1;
    }
}
