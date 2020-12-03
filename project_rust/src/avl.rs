#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::cmp::Ord;
use std::cmp::Ordering;
use std::ptr;
use std::io::{Error};
use std::fmt::{self, Debug};
use colored::*;
use std::cmp::max;

//error handling
#[derive(Debug)]
pub enum AVLBaseErr {
    DuplicateErr,
    UndefError(Error),
}

impl From<Error> for AVLBaseErr {
    fn from(err: Error) -> AVLBaseErr {
        AVLBaseErr::UndefError(err)
    }
}

pub fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {:?}", inner_err);
    } else {
        println!("No inner error");
    }
}

struct TreeNode<K: Ord, V> { //each tree node has
    key: K, //key of node
    value: V, //value of node
    parent: node_ptr<K, V>, // reference to parent
    left: node_ptr<K, V>, //reference to left child
    right: node_ptr<K, V>, //reference to right child
    level: usize, //height of node
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
            left: node_ptr(ptr::null_mut()),
            right: node_ptr(ptr::null_mut()),
            parent: node_ptr(ptr::null_mut()),
            key: k,
            value: v,
            level: 0, //TODO implement height in insertion
        };
        node_ptr(Box::into_raw(Box::new(node)))
    }

    //###############################################
    #[inline]
    fn get_node_level(&mut self) -> usize{
        unsafe{(*self.0).level}
    }

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

    fn set_level(&self, lvl:usize){
        unsafe{(*self.0).level = lvl}
    }

    //###############################################
    
    #[inline]
    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

#[derive(Debug)]
pub struct AVLTree<K: Ord, V> {
    root: node_ptr<K, V>,
    len: usize,
}

//######################## PRINTING TREE ################################
//TODO ADD A WAY TO PRINT STARTING FROM A CERTAIN NODE

const PRINT_SPACE_GLOBAL: u32 = 5;

//TreeNode display
impl<K, V> Debug for TreeNode<K, V>
where
    K: Ord + Debug + fmt::Display,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {       
    write!(f, "{}", self.key.to_string().black().on_white())  
    }
}


//AVLTree display
impl<K: Ord + Debug + fmt::Display, V: Debug> AVLTree<K, V> {
    fn print_rec(&self, node: node_ptr<K, V>, mut space: u32) {
        if node.is_null() { //exit condition
            return;
        }

        space = space + PRINT_SPACE_GLOBAL;
        self.print_rec(node.get_right(), space);
        print!("\n");
        for _ in PRINT_SPACE_GLOBAL..space{
            print!(" ")
        }
        unsafe {
            print!("{:?}\n", *node.0);
        }
        self.print_rec(node.get_left(), space);

    }

    pub fn print_tree(&self, space: u32) {
        if self.root.is_null() {
            println!("[NOTE] Tree is Empty");
            return;
        }
        
        println!("[INFO] Tree size = {:?}", self.len());
        self.print_rec(self.root, space);
    }

    pub fn inorder_trav_print(&self){
        if self.root.is_null() {
            println!("[NOTE] Tree is Empty");
            return;
        }
        println!("[INFO] Tree size = {:?}", self.len());
        self.trav_print_rec(self.root.get_left());
        unsafe{
            print!("{:?} ", *self.root.0);
        }
        self.trav_print_rec(self.root.get_right());
    }

    fn trav_print_rec(&self, node: node_ptr<K, V>){
        if node.is_null(){
            return;
        }
        self.trav_print_rec(node.get_left());
        unsafe{
            print!("{:?} ", *node.0);
        }
        self.trav_print_rec(node.get_right());
        
    }

    pub fn preorder_trav_print(&self){
        if self.root.is_null() {
            println!("[NOTE] Tree is Empty");
            return;
        }
        println!("[INFO] Tree size = {:?}", self.len());
        unsafe{
            print!("{:?} ", *self.root.0);
        }
        self.preorder_print_rec(self.root.get_left());
        self.preorder_print_rec(self.root.get_right());
    }

    fn preorder_print_rec(&self, node: node_ptr<K, V>){
        if node.is_null(){
            return;
        }
        unsafe{
            print!("{:?} ", *node.0);
        }
        self.preorder_print_rec(node.get_left());
        self.preorder_print_rec(node.get_right());
    }

    pub fn postorder_trav_print(&self){
        if self.root.is_null() {
            println!("[NOTE] Tree is Empty");
            return;
        }
        println!("[INFO] Tree size = {:?}", self.len());
        
        self.preorder_print_rec(self.root.get_left());
        self.preorder_print_rec(self.root.get_right());
        unsafe{
            print!("{:?} ", *self.root.0);
        }
    }

    fn postorder_print_rec(&self, node: node_ptr<K, V>){
        if node.is_null(){
            return;
        }
        
        self.preorder_print_rec(node.get_left());
        self.preorder_print_rec(node.get_right());
        unsafe{
            print!("{:?} ", *node.0);
        }
    }

}
//######################################################################

impl<K: Ord + Debug + fmt::Display, V: Debug> AVLTree<K, V> {
    //returns an empty AVLTree
    pub fn new() -> AVLTree<K, V> {
        AVLTree {
            root: node_ptr(ptr::null_mut()),
            len: 0, //total number of nodes
        }
    }

    // returns len of AVLTree
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    //returns true if tree is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_null()
    }

    //similar to BST insertion, then rebalancing is unique to AVLTree
    pub fn insert(&mut self, k: K, v: V) -> Result<(), AVLBaseErr>{
        self.len+=1;
        let mut new_node = node_ptr::new(k, v);
        let mut temp_root = self.root;
        let mut temp_node: node_ptr<K, V> = node_ptr(ptr::null_mut());

        if self.is_empty(){ //if the tree is empty set new node to as root
            self.root = new_node;
            return Ok(());
        }

        while !temp_root.is_null(){ //temp root isn't null (exits when reaching a null pointer at the end of the tree)
            temp_node = temp_root;
            match new_node.cmp(&&mut temp_root){
                Ordering::Less => {
                    temp_root = temp_root.get_left();
                },
                Ordering::Greater => {
                    temp_root = temp_root.get_right();
                },
                _ => {
                    return Err(AVLBaseErr::DuplicateErr);
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

        self.insert_balance(new_node); //Rotate Tree
        Ok(())
    }
    // Function required to determine the level of new nodes placed inside of the tree
    fn level_of_child_nodes(&mut self, node: node_ptr<K, V>) -> i32 {
        let mut left = 0; // Assume that the level of the left node is of level 0 -> required if there if the node doesn't exist
        let mut right = 0; // Assume that the level of the right node is of level 0 -> required if there if the node doesn't exist
        if !node.get_parent().get_left().is_null(){ //if the left node exists (not null)
            left = node.get_parent().get_left().get_node_level(); // get the level of the left node
        }
        if !node.get_parent().get_right().is_null(){ //if the right node exists (not null)
            right = node.get_parent().get_right().get_node_level(); // get the level of the right node
        }
        left as i32 - right as i32 // get the difference between left and right nodes
    }

    // Check the level of the nodes. Apply all necessary rotations on root. 
    fn insert_balance(&mut self, node: node_ptr<K, V>) {
        let level_differance = self.level_of_child_nodes(node); // get the level differances of the nodes
        if -1 <= level_differance && level_differance <= 1 {return} // if the level differance is 1, no need to do anything.
        if level_differance == 2 { // left node level is greater than right node level
            self.left_right_rotation(node) // perform appropriate roatation
        } else if level_differance == -2 {  // right node level is greater than left node level
            self.right_left_rotation(node) // perform appropriate roatation
        } else { // can never have level difference greater than (+/- 2)
            unreachable!() 
        }
    }

    fn left_left_rotation(&mut self, node: node_ptr<K, V>){
        let mut node_parent = node.get_parent();
        let mut right_parent = node_parent.get_right();
        let mut node_gparent = node_parent.get_parent();

        node_gparent.set_left(right_parent);
        node_parent.set_right(node_gparent);

        let mut ggparent = node_gparent.get_parent();

        if node_gparent == ggparent.get_right(){
            ggparent.set_right(node_parent);
        }else{
            ggparent.set_left(node_parent);
        }

        if node_gparent == self.root{
            self.set_root(node_parent);
        }
        right_parent.set_parent(node_gparent);
        node_gparent.set_parent(node_parent);
        node_parent.set_parent(ggparent);

    }

    fn left_right_rotation(&mut self, mut node: node_ptr<K, V>){
        let mut right_node = node.get_right();
        let left_node = node.get_left();
        let mut node_parent = node.get_parent();
        let mut node_gparent = node_parent.get_parent();
        let mut left_parent = node_parent.get_left();

        node_parent.set_right(right_node);
        node_parent.set_left(left_node);
        node.set_left(node_parent);
        node.set_right(left_parent);
        node_gparent.set_left(node);

        right_node.set_parent(node_parent);
        right_node.set_parent(node_parent);
        node_parent.set_parent(node);
        left_parent.set_parent(node);
        node.set_parent(node_gparent);

        self.left_left_rotation(node_parent);
    }

    fn right_right_rotation(&mut self, node: node_ptr<K, V>){
        let mut node_parent = node.get_parent();
        let mut left_parent = node_parent.get_left();
        let mut node_gparent = node_parent.get_parent();

        
        node_gparent.set_right(left_parent);
        node_parent.set_left(node_gparent);
        let mut ggparent = node_gparent.get_parent();
        if node_gparent == ggparent.get_right(){
            ggparent.set_right(node_parent);
        }else{
            ggparent.set_left(node_parent);
        }
        
        if node_gparent == self.root{
            self.set_root(node_parent);
        }

        left_parent.set_parent(node_gparent);
        node_gparent.set_parent(node_parent);
        node_parent.set_parent(ggparent);

    }

    fn right_left_rotation(&mut self, mut node: node_ptr<K, V>){
        let mut right_node = node.get_right();
        let mut node_parent = node.get_parent();
        let mut node_gparent = node_parent.get_parent();

        node_parent.set_left(right_node);
        node.set_right(node_parent);
        node_gparent.set_right(node);

        right_node.set_parent(node_parent);
        node_parent.set_parent(node);
        node.set_parent(node_gparent);

        self.right_right_rotation(node_parent);
    }


    pub fn delete(&mut self, k: K, v: V) -> Result<(K,V), AVLBaseErr>{

        Ok((k, v))
    }

    fn delete_balance(&mut self, mut node: node_ptr<K, V>, mut parent: node_ptr<K, V>) -> Result<(), AVLBaseErr>{


        Ok(())
    }

    fn set_root(&mut self, node: node_ptr<K, V>){
        
        (*self).root = node;
        
    }

    pub fn get_height(&self) -> Option<usize>{
        let mut height = 0;
        if self.is_empty(){
            return Some(height);
        }else{
            
            height = self.get_height_rec(self.root,1);

        }

        return Some(height);
    }

    fn get_height_rec(&self, node: node_ptr<K, V>, mut h: usize) -> usize{
        if node.is_null(){
            return h-1;
        }

        h += 1;

        let height_r = self.get_height_rec(node.get_right(), h);
        let height_l = self.get_height_rec(node.get_left(), h);

        max(height_r, height_l)
    }

}


//Tests (Essam)
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn insert_test(){
        let mut tree: AVLTree<usize, usize> = AVLTree::new();
        
        tree.insert(4, 4).unwrap();
        tree.insert(5, 5).unwrap();
        tree.insert(2, 2).unwrap();
        tree.insert(1, 1).unwrap();
        tree.insert(3, 3).unwrap();
        tree.insert(6, 6).unwrap();
        tree.insert(0, 0).unwrap();
        tree.insert(9, 9).unwrap();
        tree.insert(11, 11).unwrap();
        tree.insert(13, 13).unwrap();
        tree.insert(67, 67).unwrap();
        tree.insert(68, 68).unwrap();
        tree.insert(69, 69).unwrap();
        tree.insert(77, 77).unwrap();
        tree.insert(88, 88).unwrap();
        tree.insert(99, 99).unwrap();
        tree.insert(608, 608).unwrap();
        tree.insert(111, 111).unwrap();
        tree.insert(222, 222).unwrap();
        tree.insert(300,300).unwrap();


        tree.print_tree(1);
    }

    #[test]
    fn height_test(){
        let mut tree: AVLTree<usize, usize> = AVLTree::new();

        tree.insert(4, 4).unwrap();
        tree.insert(5, 5).unwrap();
        tree.insert(2, 2).unwrap();
        tree.insert(1, 1).unwrap();
        tree.insert(3, 3).unwrap();
        tree.insert(6, 6).unwrap();
        tree.insert(0, 0).unwrap();
        tree.insert(9, 9).unwrap();

        let h = tree.get_height().unwrap();
        println!("{}", h);
    }

    #[test]
    fn print_inorder_test(){
        let mut tree: AVLTree<usize, usize> = AVLTree::new();

        tree.insert(4, 4).unwrap();
        tree.insert(5, 5).unwrap();
        tree.insert(2, 2).unwrap();
        tree.insert(1, 1).unwrap();
        tree.insert(3, 3).unwrap();
        tree.insert(6, 6).unwrap();
        tree.insert(0, 0).unwrap();
        tree.insert(9, 9).unwrap();
        tree.insert(11, 11).unwrap();
        tree.insert(13, 13).unwrap();
        tree.insert(67, 67).unwrap();
        tree.insert(68, 68).unwrap();
        tree.insert(69, 69).unwrap();
        tree.insert(77, 77).unwrap();
        tree.insert(88, 88).unwrap();
        tree.insert(99, 99).unwrap();
        tree.insert(608, 608).unwrap();
        tree.insert(111, 111).unwrap();
        tree.insert(222, 222).unwrap();
        tree.insert(300,300).unwrap();

        tree.print_tree(1);

        tree.inorder_trav_print();
    }

    #[test]
    fn print_preorder_test(){
        let mut tree: AVLTree<usize, usize> = AVLTree::new();

        tree.insert(4, 4).unwrap();
        tree.insert(5, 5).unwrap();
        tree.insert(2, 2).unwrap();
        tree.insert(1, 1).unwrap();
        tree.insert(3, 3).unwrap();
        tree.insert(6, 6).unwrap();
        tree.insert(0, 0).unwrap();
        tree.insert(9, 9).unwrap();
        tree.insert(11, 11).unwrap();
        tree.insert(13, 13).unwrap();
        tree.insert(67, 67).unwrap();
        tree.insert(68, 68).unwrap();
        tree.insert(69, 69).unwrap();
        tree.insert(77, 77).unwrap();
        tree.insert(88, 88).unwrap();
        tree.insert(99, 99).unwrap();
        tree.insert(608, 608).unwrap();
        tree.insert(111, 111).unwrap();
        tree.insert(222, 222).unwrap();
        tree.insert(300,300).unwrap();

        tree.print_tree(1);

        tree.preorder_trav_print();
    }

    #[test]
    fn print_postorder_test(){
        let mut tree: AVLTree<usize, usize> = AVLTree::new();

        tree.insert(4, 4).unwrap();
        tree.insert(5, 5).unwrap();
        tree.insert(2, 2).unwrap();
        tree.insert(1, 1).unwrap();
        tree.insert(3, 3).unwrap();
        tree.insert(6, 6).unwrap();
        tree.insert(0, 0).unwrap();
        tree.insert(9, 9).unwrap();
        tree.insert(11, 11).unwrap();
        tree.insert(13, 13).unwrap();
        tree.insert(67, 67).unwrap();
        tree.insert(68, 68).unwrap();
        tree.insert(69, 69).unwrap();
        tree.insert(77, 77).unwrap();
        tree.insert(88, 88).unwrap();
        tree.insert(99, 99).unwrap();
        tree.insert(608, 608).unwrap();
        tree.insert(111, 111).unwrap();
        tree.insert(222, 222).unwrap();
        tree.insert(300,300).unwrap();

        tree.print_tree(1);

        tree.postorder_trav_print();
    }

}
