use crate::linear_ds::{array_list::ArrayList, linked_list::LinkedList};

mod linear_ds;

fn main() {
    let mut list = LinkedList::new();
    list.add_last(10);
    list.add_last(20);
    list.add_last(30);
    list.add_last(40);
    list.add_first(5);

    list.remove_first();
    list.remove_first();
    list.remove_first();
    list.remove_first();
    list.remove_first();

    println!("{:?}", list);
}
