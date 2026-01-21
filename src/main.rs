use crate::boxed::LinkedList;

mod boxed;
mod refed;

fn main() {
    let mut list = LinkedList::<String>::new();

    list.push("Sai".to_owned());
    list.push("Krishna".to_owned());
    list.push("Rao".to_owned());
    list.push("Katterishetty".to_owned());

    for x in list.iter_mut() {
        x.push_str("Test");
    }

    dbg!(list);
}
