/*
1- Insert a node to the red-black tree.
2- Delete a node from the red-black tree.
3- Count the number of leaves in a tree.
4- Return the height of a tree.
5- Print In-order traversal of the tree.
6- Check if the tree is empty.

*/

use std::cmp::Ord;
use std::cmp::Ordering;
use std::ptr;
use std::io::{Error, ErrorKind};

//error handling
#[derive(Debug)]
pub enum RBBaseErr {
    UndefError(Error),
}

impl From<Error> for RBBaseErr {
    fn from(err: Error) -> RBBaseErr {
        RBBaseErr::UndefError(err)
    }
}

pub fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {:?}", inner_err);
    } else {
        println!("No inner error");
    }
}


#[derive(Clone, Debug, PartialEq, Copy, Eq)]
enum NodeColor { //color of node either red or black
    Red,
    Black,
}

type RedBlackTree= Option<usize>;

struct TreeNode<K: Ord, V> { //each tree node has
    color: NodeColor,  //node color either red or black (red for new nodes, black for root)
    key: K, //key of node
    value: V, //value of node
    parent: node_ptr<K, V>, // reference to parent
    left: node_ptr<K, V>, //reference to left child
    right: node_ptr<K, V>, //reference to right child
    height: usize, //height of node
}


#[derive(Debug)]
struct node_ptr<K: Ord, V>(*mut TreeNode<K, V>); //pointer to mutable TreeNode

impl<K: Ord, V> Clone for node_ptr<K, V>{
    fn clone(&self) -> node_ptr<K, V>{ //clones the ptr
        node_ptr(self.0) //returns the pointer
    }
}

impl<K: Ord, V> Copy for node_ptr<K, V> {}

impl<K: Ord, V> Ord for node_ptr<K, V> { //compare node key with another node key
    fn cmp(&self, other: &node_ptr<K, V>) -> Ordering {
        unsafe{
            (*self.0).key.cmp(&(*other.0).key)
        }
    }
}

impl<K: Ord, V> Eq for node_ptr<K, V> {} //requried for Ord

impl<K: Ord, V> PartialOrd for node_ptr<K, V> { //required for Ord
    fn partial_cmp(&self, other: &node_ptr<K, V>) -> Option<Ordering> {
        unsafe { Some((*self.0).key.cmp(&(*other.0).key)) }
    }
}

impl<K: Ord, V> PartialEq for node_ptr<K, V> { //required for Eq and PartialOrd
    fn eq(&self, other: &node_ptr<K, V>) -> bool {
        self.0 == other.0
    }
}

impl<K: Ord, V> node_ptr<K, V>{
    fn new(k: K, v: V) -> node_ptr<K, V>{ //create new node
        let node = TreeNode {
            color: NodeColor::Black,
            left: node_ptr(ptr::null_mut()),
            right: node_ptr(ptr::null_mut()),
            parent: node_ptr(ptr::null_mut()),
            key: k,
            value: v,
            height: 0,
        };
        node_ptr(Box::into_raw(Box::new(node)))
    }

    //############ HANDLING COLORS ##################
    #[inline]
    fn set_color(&mut self, color: NodeColor){
        if self.is_null(){
            return;
        }
        unsafe{
            (*self.0).color = color;
        }
        
    }
    #[inline]
    fn set_red(&mut self){
        self.set_color(NodeColor::Red);
    }
    #[inline]
    fn set_black(&mut self){
        self.set_color(NodeColor::Black);
    }

    #[inline]
    fn get_color(&mut self) -> NodeColor{
        if self.is_null(){
            return NodeColor::Black;
        }
        unsafe{
            (*self.0).color
        }
        
    }
    #[inline]
    fn is_red(&mut self) -> bool{
        self.get_color() == NodeColor::Red
    }
    #[inline]
    fn is_black(&mut self) -> bool{
        self.get_color() == NodeColor::Black
    }
    //###############################################

    //############ HANDLING RELATIONS ###############
    //setting parent for current node
    #[inline]
    fn set_parent(&mut self, parent: node_ptr<K, V>){
        if self.is_null(){
            return;
        }
        unsafe{
            (*self.0).parent = parent
        }
    }

    //setting right child of current node
    #[inline]
    fn set_right(&mut self, right: node_ptr<K, V>){
        if self.is_null(){
            return;
        }
        unsafe{
            (*self.0).right = right
        }
    }

    //setting left child for current node
    #[inline]
    fn set_left(&mut self, left: node_ptr<K, V>){
        if self.is_null(){
            return;
        }
        unsafe{
            (*self.0).left = left
        }
    }

    //getting clone of current node parent
    #[inline]
    fn get_parent(&self) -> node_ptr<K, V>{
        if self.is_null(){
            return node_ptr(ptr::null_mut());
        }
        unsafe{
            (*self.0).parent.clone()
        }
    }

    //getting clone of current node right child
    #[inline]
    fn get_right(&self) -> node_ptr<K, V>{
        if self.is_null(){
            return node_ptr(ptr::null_mut());
        }
        unsafe{
            (*self.0).right.clone()
        }
    }

    //getting clone of current node left child
    fn get_left(&self) -> node_ptr<K, V>{
        if self.is_null(){
            return node_ptr(ptr::null_mut());
        }
        unsafe{
            (*self.0).left.clone()
        }
    }

    //check if current node is left child
    fn is_left(&self) -> bool{
        self.get_parent().get_left() == *self
    }

    //check if current node is right child
    fn is_right(&self) -> bool{
        self.get_parent().get_right() == *self
    }




    //###############################################
    
    #[inline]
    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}


pub struct RBTree<K: Ord, V> {
    root: node_ptr<K, V>,
    len: usize,
}


impl<K: Ord, V> RBTree<K, V> {
    //returns an empty RBTree
    pub fn new() -> RBTree<K, V> {
        RBTree {
            root: node_ptr(ptr::null_mut()),
            len: 0, //total number of nodes
        }
    }

    // returns len of RBTree
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    //returns true if tree is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_null()
    }

    //similar to BST insertion, then rebalancing is unique to RBTree
    pub fn insert(&mut self, k: K, v: V) -> Result<(), RBBaseErr>{
        self.len+=1;
        let mut new_node = node_ptr::new(k, v);
        let mut temp_root = self.root;
        let mut temp_node: node_ptr<K, V> = node_ptr(ptr::null_mut());

        if self.is_empty(){ //if the tree is empty set new node to as root
            new_node.set_black(); //set root to black
            self.root = new_node;
            return Ok(());
        }

        while !temp_root.is_null(){ //temp root isn't null (exits when reaching a null pointer at the end of the tree)
            temp_node = temp_root;
            match new_node.cmp(&&mut temp_root){
                Ordering::Less => {
                    temp_root = temp_root.get_left();
                },
                _ => {
                    temp_root = temp_root.get_right();
                }

            };
        }

        new_node.set_parent(temp_node); //sets parent for new_node

        match new_node.cmp(&&mut temp_node){ //adds new node to tree
            Ordering::Less => {
                temp_node.set_left(new_node);
            },
            _ => {
                temp_node.set_right(new_node);
            }
        };

        new_node.set_red(); //set color of newly inserted node to red

        self.insert_balance(new_node).unwrap(); //recolour and rotate

        Ok(())
    }

    //Recolour and rotation for RBTree after insertion
    fn insert_balance(&mut self, mut node: node_ptr<K, V>) -> Result<(), RBBaseErr>{


        Ok(())
    }

    pub fn delete(&mut self, k: K, v: V) -> Result<(K,V), RBBaseErr>{

        Ok((k, v))
    }

    fn delete_balance(&mut self, mut node: node_ptr<K, V>, mut parent: node_ptr<K, V>) -> Result<(), RBBaseErr>{


        Ok(())
    }


}

//Tests (Essam)
mod tests {
    use super::*;

    #[test]
    fn insert_test(){
        let mut tree: RBTree<usize, usize> = RBTree::new();
        
        tree.insert(4, 4).unwrap();
        tree.insert(5, 5).unwrap();
        tree.insert(2, 2).unwrap();
        tree.insert(1, 1).unwrap();
        tree.insert(3, 3).unwrap();

        assert_eq!(tree.len(), 5);
    }

    #[test]
    fn delete_test(){

    }

    #[test]
    fn count_test(){

    }

    #[test]
    fn height_test(){

    }



}