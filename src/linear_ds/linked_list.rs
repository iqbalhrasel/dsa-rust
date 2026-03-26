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

    pub fn index_of(&self, item: i32) -> Option<usize> {
        let mut current = self.head.clone();
        let mut index = 0;

        while let Some(node_rc) = current {
            let node = node_rc.borrow();

            if node.value == item {
                return Some(index);
            }

            current = node.next_node.clone();
            index += 1;
        }

        return None;
    }

    pub fn contains(&self, item: i32) -> bool {
        return self.index_of(item) != None;
    }

    pub fn remove_first(&mut self) -> Result<(), String> {
        match self.head.take() {
            Some(node_rc) => {
                let mut node = node_rc.borrow_mut();
                let temp = node.next_node.take();
                self.head = temp;

                if self.head.is_none() {
                    self.tail = None;
                }

                self.count -= 1;
                return Ok(());
            }
            None => return Err("list is empty".to_string()),
        };
    }

    pub fn remove_last(&mut self) -> Result<(), String> {
        if self.head.is_none() {
            return Err("list is empty".to_string());
        }

        if self.head.as_ref().unwrap().borrow().next_node.is_none() {
            self.head = None;
            self.tail = None;
            self.count -= 1;
            return Ok(());
        }

        let mut current = self.head.clone();

        while let Some(node_rc) = current.clone() {
            let next = node_rc.borrow_mut().next_node.clone();

            if let Some(next_node) = &next {
                if next_node.borrow().next_node.is_none() {
                    node_rc.borrow_mut().next_node = None;
                    self.tail = Some(node_rc.clone());
                    self.count -= 1;
                    return Ok(());
                }
            }
            current = next;
        }

        return Ok(());
    }

    pub fn size(&self) -> usize {
        return self.count;
    }
}
