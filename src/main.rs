use std::fmt::Debug;

#[derive(Debug)]
struct LinkedList<T: PartialEq + Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Debug> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: T) {
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

    fn pop(&mut self) -> Option<T> {
        let prev_head = self.head.take();
        match prev_head {
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.val),
            None => None,
        }
    }
}

#[derive(Debug)]
struct Node<T: PartialEq + Debug> {
    val: T,
    next: Option<Box<Node<T>>>,
}

fn main() {
    let mut list = LinkedList::<String>::new();

    list.push("Sai".to_owned());
    list.push("Krishna".to_owned());
    list.push("Rao".to_owned());
    list.push("Katterishetty".to_owned());

    println!("peek {:?}", list.peek());

    //    dbg!(&list);

    println!("{:?}", list.pop());
    println!("{:?}", list.pop());

    //   dbg!(&list);
    println!("peek2 {:?}", list.peek());
}
