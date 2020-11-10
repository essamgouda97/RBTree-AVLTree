/**
1- Insert a node to the red-black tree.
2- Delete a node from the red-black tree.
3- Count the number of leaves in a tree.
4- Return the height of a tree.
5- Print In-order traversal of the tree.
6- Check if the tree is empty.
*/


use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;

type RedBlackTree= Option<Tree>;
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    pub data: D,

    
    left: RedBlackTree,
    right: RedBlackTree,
}

impl TreeNode{

    fn new() -> () { () } //(Essam)

    //Check if the tree is empty. (Essam)
    fn empty() -> () { () }

    //Insert a node to the red-black tree. (Samantha)
    fn insert(&mut self, key: T, data: D) { 
        self.root = Self::put(self.root, None, key, data, &mut self.nodes);
        self.nodes[self.root.unwrap()].color = NodeColor::Black;
     }
    
    //Delete a node from the red-black tree. (Essam)
    fn delete() -> () { () }

    //Count the number of leaves in a tree. (Essam)
    fn count_num_of_leaves() -> () { () }

    //Return the height of a tree. (Nigel)
    fn height_of_tree() -> () { () }

    //Print In-order traversal of the tree. (Nigel)
    fn print_tree() -> () { () }

    fn recolour() -> () { () }

    fn rotation() -> () { () }
}

//Tests (Essam)
