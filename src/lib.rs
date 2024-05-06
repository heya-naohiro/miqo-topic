use core::fmt::Debug;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T: Debug + Clone> {
    value: Option<T>,
    children: RefCell<HashMap<String, Vec<Rc<Node<T>>>>>,
    parent: RefCell<Weak<Node<T>>>,
}

impl<T: Debug + Clone> Node<T> {
    pub fn new_node(value: Option<T>) -> Node<T> {
        return Node {
            value,
            children: RefCell::new(HashMap::new()),
            parent: RefCell::new(Weak::new()),
        };
    }
}

struct Tree<T: Debug + Clone> {
    rootnode: Node<T>,
}

impl<T: Debug + Clone> Tree<T> {
    fn add_child_node(parent: &Rc<Node<T>>, child: Rc<Node<T>>, key: String) {
        let rc_branch = Rc::clone(&parent);
        *child.parent.borrow_mut() = Rc::downgrade(&rc_branch);
        //rc_branch.children.borrow_mut().insert(key, child);
        let mut binding = rc_branch.children.borrow_mut();
        let v = binding.entry(key).or_insert(vec![]);
        v.push(child)
    }

    fn add_subscriber(root: &Rc<Node<T>>, topic_filter: String, value: Option<T>) {
        let mut tmp_node = Rc::clone(&root);
        for tfe in topic_filter.split('/') {
            let child = Rc::new(Node::new_node(value.clone()));
            let tmp_child = Rc::clone(&child);
            Tree::add_child_node(&tmp_node, tmp_child, tfe.to_string());
            tmp_node = Rc::clone(&child)
        }
    }

    fn search_topic(node: &Rc<Node<T>>, topic: String) -> Vec<Option<T>> {
        println!("search : {:?}", node);
        let mut result: Vec<Option<T>> = vec![];
        // hello/world
        // hello/+
        let mut tmp_node_vector: Vec<Rc<Node<T>>> = vec![Rc::clone(node)];
        let mut new_tmp_node_vector: Vec<Rc<Node<T>>> = vec![];
        let mut iter = topic.split('/');
        while let Some(e) = iter.next() {
            println!("topic element: {:?}", e);
            // hello
            new_tmp_node_vector = vec![];

            for node in &tmp_node_vector {
                //let node_ref = Rc::clone(node);
                println!("Hello!!!! {:?}", e);
                if let Some(node_vector) = node.children.borrow().get(&e.to_string()) {
                    for n in node_vector {
                        new_tmp_node_vector.push(Rc::clone(n));
                    }
                }
                if let Some(node_vector) = node.children.borrow().get(&'#'.to_string()) {
                    //new_tmp_node_vector.extend(*node_vector);
                    for n in node_vector {
                        result.push(n.value.clone());
                    }
                }

                if let Some(node_vector) = node.children.borrow().get(&'+'.to_string()) {
                    for n in node_vector {
                        new_tmp_node_vector.push(Rc::clone(n));
                    }
                }
            }
            // /a/b/cのcまで到達したら結果を格納する
            if iter.next().is_none() {
                for node in &tmp_node_vector {
                    result.push(node.value.clone());
                }
            }
            tmp_node_vector = new_tmp_node_vector;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Node, Tree};
    use std::{
        //borrow::{Borrow, BorrowMut},
        rc::Rc,
    };

    #[test]
    fn small_tree() {
        let root: Node<i32> = Node::new_node(None);
        let rc_root: Rc<Node<i32>> = Rc::new(root);
        // ---
        let leaf = Node::new_node(Some(1));
        let rc_leaf = Rc::new(leaf);
        Tree::add_child_node(&rc_root, rc_leaf, "hogehoge".to_string());

        let leaf2 = Node::new_node(Some(2));
        let rc_leaf2 = Rc::new(leaf2);
        Tree::add_child_node(&rc_root, rc_leaf2, "hogehoge2".to_string());
        //let r: Node<i32> = rc_root.borrow();
        println!("{:?}", rc_root);
    }

    #[test]
    fn search_tree() {
        let root: Node<i32> = Node::new_node(None);
        let rc_root: Rc<Node<i32>> = Rc::new(root);
        Tree::add_subscriber(&rc_root, "hoge/piyo/fuga".to_string(), Some(10));
        //let leaf = Node::new_node(Some(1));
        //let rc_leaf = Rc::new(leaf);
        //Tree::add_child_node(&rc_root, rc_leaf, "hoge/fuga/piyo".to_string());

        //let leaf2 = Node::new_node(Some(2));
        //let rc_leaf2 = Rc::new(leaf2);
        //Tree::add_child_node(&rc_root, rc_leaf2, "hoge/piyo/fuga".to_string());

        println!("{:?}", rc_root);

        let result = Tree::search_topic(&rc_root, "hoge/fuga/piyo".to_string());
        println!("result = {:?}", result);
    }
}
/*
Node { value: None,
    children: RefCell
        { value:
            {"hogehoge2": Node
                { value: Some(2),
                    children: RefCell { value: {} },
                    parent: RefCell { value: (Weak) }
                },
            "hogehoge": Node
                { value: Some(1), children: RefCell { value: {} }, parent: RefCell { value: (Weak) }
                }
            }
        }, parent: RefCell
        { value: (Weak) }
    }

*/
