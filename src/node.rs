use bevy::prelude::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type ParentLink = Option<Weak<RefCell<Node>>>;
type ChildLink = Option<Rc<RefCell<Node>>>;

// Definition for MCTS node
#[derive(Clone, Debug)]
pub struct Node {
    parent: ParentLink,
    atk_low_child: ChildLink,
    atk_high_child: ChildLink,
    matk_hit_child: ChildLink,
    matk_miss_child: ChildLink,
    heal_child: ChildLink,
    value: f32,
    times_visited: u32,
    ucb_value: f32,
    /* Notes (some are directly copied from other sources, this should only be used for personal understanding/reference):
     * Option<T> allows the value type to be set to None
     * Box<T> is used to store the data on the heap, which is necessary because the node size is not known at compile time
     *  - is a pointer
     * Rc<T> is like box but enables multiple ownership (e.g. one node having multiple parents or multiple child nodes being doubly linked to one parent)
     *  - is a pointer
     *  - if using Rc, probably don't need box
     *  - only allows immutable access to contents, so RefCell is needed if want to be able to modify data
     *  - if 2 nodes are doubly linked to each other (aka circularly linked), neither Rc can drop and you can get memory leaks
     * RefCell<T> allows mutable borrows checked at runtime, so you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable
     *  - is a cell
     *  - Rc<RefCell<T>> allows contents to be more accessible than RefCell<Rc<T>>
     * Weak<T> is useful for keeping a temporary reference to the allocation managed by Rc without preventing its inner value from being dropped
     * - is a pointer 
     * - used to prevent circular references between Rc pointers, since mutual owning references would never allow either Rc to be dropped
     *      - ex: a tree could have strong Rc pointers from parent nodes to children, and Weak pointers from children back to their parents
     */
}

impl Node { 
    // create an empty node with no connections and base values of 0
    fn new() -> Self {
        Node {
            parent: None,
            atk_low_child: None,
            atk_high_child: None,
            matk_hit_child: None,
            matk_miss_child: None,
            heal_child: None,
            value: 0.,
            times_visited: 0,
            ucb_value: 0.,
        }
    }
    fn new_child(parent: ParentLink, value: f32) -> Self {
        Node {
            parent,
            atk_low_child: None,
            atk_high_child: None,
            matk_hit_child: None,
            matk_miss_child: None,
            heal_child: None,
            value,
            times_visited: 0,
            ucb_value: 0.,
        }
    }
}

pub trait NodeFunctions {
    fn add_atk_low_child(&self, value: f32) -> Self;
}

impl NodeFunctions for Rc<RefCell<Node>> {
    fn add_atk_low_child(&self, value: f32) -> Self {
        let new_atk_low_child = Node::new_child(Some(Rc::downgrade(self)), value);  // create child node

        let rc = Rc::new(RefCell::new(new_atk_low_child));                          // create reference to child node
        self.borrow_mut().atk_low_child = Some(Rc::clone(&rc));                                  // set self.atk_low_child ref to rc

        Rc::clone(&rc)  // returns this value
    }
}

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, make_test_tree);
    }
}

fn print_node(node: &Node) {
    info!("value: {}", node.value.to_string());
    info!("times_visited: {}", node.times_visited.to_string());
    info!("ucb_value: {}", node.ucb_value.to_string());
}

fn print_node_ref(node: &Rc<RefCell<Node>>) {
    info!("value: {:?}", node.clone());
}

fn update_value(node: &mut Node, value: f32) {
    node.value = value;
}

fn update_times_visited(node: &mut Node) {
    node.times_visited += 1;
}

fn update_ucb_value(node: &mut Node, ucb_value: f32) {
    node.value = ucb_value;
}

fn make_test_tree() {
    println!("Tree nodes");

    let mut root = Node::new();
    update_value(&mut root, 2.);
    print_node(&root);
    let mut n1 = Node::new();
    update_value(&mut n1, 1.);
    print_node(&n1);
    let mut n2 = Node::new();
    print_node(&n2);

    let r = Rc::new(RefCell::new(root.clone()));
    let c = Rc::new(RefCell::new(root.clone())).add_atk_low_child(5.);
    print_node_ref(&r);
    print_node_ref(&c);
}