use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: Option<i32>,
    children: RefCell<HashMap<String, Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: Some(3),
        children: RefCell::new(HashMap::new()),
        parent: RefCell::new(Weak::new()),
    });

    let mut mymap = HashMap::new();
    mymap.insert("key1".to_string(), leaf);
    let branch = Rc::new(Node {
        value: Some(5),
        children: RefCell::new(mymap),
        parent: RefCell::new(Weak::new()),
    });
    println!("{:?}", branch);

    // Leaf2をbranchのchildrenに追加する
    let rc_branch = Rc::clone(&branch);
    let leaf2 = Rc::new(Node {
        value: Some(5),
        children: RefCell::new(HashMap::new()),
        parent: RefCell::new(Rc::downgrade(&rc_branch)),
    });
    let leaf3 = Rc::new(Node {
        value: Some(6),
        children: RefCell::new(HashMap::new()),
        parent: RefCell::new(Rc::downgrade(&rc_branch)),
    });

    branch
        .children
        .borrow_mut()
        .insert("key2".to_string(), leaf2);
    branch
        .children
        .borrow_mut()
        .insert("key4".to_string(), leaf3);

    println!("{:?}", branch);
}
