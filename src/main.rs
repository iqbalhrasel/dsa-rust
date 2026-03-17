use crate::linear_ds::array_list::ArrayList;

mod linear_ds;

fn main() {
    let mut list = ArrayList::new(3);
    list.insert(10);
    list.insert(20);
    list.insert(30);
    list.insert(40);
    list.insert(50);

    println!("{:?}", list.index_of(40));

    println!("{:?}", list.to_string());
}
