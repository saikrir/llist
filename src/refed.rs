use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T: PartialEq + Debug> {
    val: T,
    next: Option<Rc<Node<T>>>,
}

pub struct LinkedList<T: PartialEq + Debug> {
    head: Option<Rc<Node<T>>>,
}

impl<T: PartialEq + Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&self, val: T) -> Self {
        let node = Node {
            val,
            next: self.head.clone(),
        };

        Self {
            head: Some(Rc::new(node)),
        }
    }

    pub fn tail(&self) -> Self {
        let tail_node = self.head.as_ref().and_then(|v| v.next.clone());
        Self { head: tail_node }
    }
}
