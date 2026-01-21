use std::{fmt::Debug, ops::Deref};

pub struct ListIter<'a, T: PartialEq + Debug> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: PartialEq + Debug> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

pub struct ListIterMut<'a, T: PartialEq + Debug> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: PartialEq + Debug> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|v| {
            self.next = v.next.as_deref_mut();
            &mut v.val
        })
    }
}

pub struct IntoListIter<T: PartialEq + Debug>(LinkedList<T>);

impl<T: PartialEq + Debug> Iterator for IntoListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub struct LinkedList<T: PartialEq + Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Debug> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = IntoListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoListIter(self)
    }
}

impl<T: PartialEq + Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, val: T) {
        let prev_head = self.head.take();
        match prev_head {
            Some(node) => {
                let new_node = Node {
                    val,
                    next: Some(node),
                };
                self.head = Some(Box::new(new_node));
            }
            None => self.head = Some(Box::new(Node { val, next: None })),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let prev_head = self.head.take();
        match prev_head {
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.val),
            None => None,
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter {
            next: self.head.as_ref().map(|node| node.deref()),
        }
    }

    pub fn iter_mut(&mut self) -> ListIterMut<'_, T> {
        ListIterMut {
            next: self.head.as_mut().map(|node| node.as_mut()),
        }
    }
}

#[derive(Debug)]
pub struct Node<T: PartialEq + Debug> {
    val: T,
    next: Option<Box<Node<T>>>,
}
