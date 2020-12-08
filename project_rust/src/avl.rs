#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::cmp::Ord;
use std::cmp::Ordering;
use std::ptr;
use std::io::{Error};
use std::fmt::{self, Debug};
use colored::*;
use std::cmp::max;

///////////////////////////////////////////////////////////////////////////////
//Error Handling
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
// Define errors. Main one is Duplicate Error -> if the number already exists in the tree
pub enum AVLBaseErr {
    DuplicateErr,
    UndefError(Error),
}

impl From<Error> for AVLBaseErr {
    fn from(err: Error) -> AVLBaseErr {
        AVLBaseErr::UndefError(err)
    }
}

// Handling error printing to user
pub fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {:?}", inner_err);
    } else {
        println!("No inner error");
    }
}

///////////////////////////////////////////////////////////////////////////////
//Structure for nodes in tree. Use node_ptr to maintain relations between nodes
///////////////////////////////////////////////////////////////////////////////
struct TreeNode<K: Ord, V> { //each tree node has
    key: K, //key of node
    value: V, //value of node
    parent: node_ptr<K, V>, // reference to parent
    left: node_ptr<K, V>, //reference to left child
    right: node_ptr<K, V>, //reference to right child
    level: usize, //height of node
}

// Return the key, value pair of a node
impl<K: Ord, V> TreeNode<K, V> {
    #[inline]
    fn pair(self) -> (K, V){ //get key (K) and value (V) of the node
        (self.key, self.value)
    }
}

#[derive(Debug)]
struct node_ptr<K: Ord, V>(*mut TreeNode<K, V>); //pointer to mutable TreeNode

impl<K: Ord, V> Clone for node_ptr<K, V>{ // initiate clone trait for node
    fn clone(&self) -> node_ptr<K, V>{ //clones the ptr
        node_ptr(self.0) //returns the pointer
    }
}

impl<K: Ord, V> Copy for node_ptr<K, V> {} // initiate copy trait for node

impl<K: Ord, V> Ord for node_ptr<K, V> { //compare node key with another node key
    fn cmp(&self, other: &node_ptr<K, V>) -> Ordering {
        unsafe{
            (*self.0).key.cmp(&(*other.0).key) // return the realtion between two nodes (which key is greater, less than)
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

///////////////////////////////////////////////////////////////////////////////
//Structure for the tree. Use node_ptr to maintain relations between nodes in tree
///////////////////////////////////////////////////////////////////////////////

impl<K: Ord, V> node_ptr<K, V>{
    fn new(k: K, v: V) -> node_ptr<K, V>{ //Create a new node in the tree
        let node = TreeNode {
            left: node_ptr(ptr::null_mut()), //The left child of the node is null (DNE)
            right: node_ptr(ptr::null_mut()), //The right child of the node is null (DNE)
            parent: node_ptr(ptr::null_mut()), //The root has no parent (DNE). New nodes get the parent when intserted
            key: k, // Key from user
            value: v, // Value from user
            level: 0, //Default height / level of node in tree is 0. This gets reevaluated upon insertion
        };
        node_ptr(Box::into_raw(Box::new(node))) //Operate on the heap
    }
    ///////////////////////////////////////////////////////////////////////////////
    //Handling Node Levels
    ///////////////////////////////////////////////////////////////////////////////

    #[inline]
    // Set the level of the node in the tree. Used to rebalance AVL Tree
    fn set_level(&self, lvl:usize){
        unsafe{(*self.0).level = lvl}
    }

    #[inline]
    // Get the level of the node in the tree. Used to rebalance AVL Tree
    fn get_level(self) -> usize{
        unsafe{(*self.0).level}
    }

    ///////////////////////////////////////////////////////////////////////////////
    //Handling Node Relations
    ///////////////////////////////////////////////////////////////////////////////

    #[inline]
    // Set the parent of the node
    fn set_parent(&mut self, parent: node_ptr<K, V>){
        if self.is_null(){ // If there is no parent avalaible, return
            return;
        }
        unsafe{ // Set the parent of the node
            (*self.0).parent = parent
        }
    }

    #[inline]
    // Get clone of the parent of the node
    fn get_parent(&self) -> node_ptr<K, V>{
        if self.is_null(){
            return node_ptr(ptr::null_mut());
        }
        unsafe{ // Get the parent of the node
            (*self.0).parent.clone()
        }
    }

    #[inline]
    // Set the right child of the node
    fn set_right(&mut self, right: node_ptr<K, V>){
        if self.is_null(){ // If there is no right child avalaible, return 
            return;
        }
        unsafe{ // Set the right child of the node
            (*self.0).right = right
        }
    }

    #[inline]
    // Set the left child of the node
    fn set_left(&mut self, left: node_ptr<K, V>){
        if self.is_null(){ // If there is no left child avalaible, return 
            return;
        }
        unsafe{ // Set the left child of the node
            (*self.0).left = left
        }
    }

    #[inline]
    // Get clone of the right child of the node
    fn get_right(&self) -> node_ptr<K, V>{
        if self.is_null(){ // If there is no right child avalaible, return 
            return node_ptr(ptr::null_mut());
        }
        unsafe{ // Get clone of the right child of the node
            (*self.0).right.clone()
        }
    }

    #[inline]
    // Get clone of the right child of the node
    fn get_left(&self) -> node_ptr<K, V>{
        if self.is_null(){ // If there is no left child avalaible, return 
            return node_ptr(ptr::null_mut());
        }
        unsafe{ // Get clone of the right child of the node
            (*self.0).left.clone()
        }
    }
    #[inline]
    //Check if current node is left child
    fn is_left(&self) -> bool{
        self.get_parent().get_left() == *self
    }

    #[inline]
    //Check if current node is right child
    fn is_right(&self) -> bool{
        self.get_parent().get_right() == *self
    }
    
    #[inline]
    //Check if the node is empty / null
    fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[inline]
    // Get the smallest value in the tree
    fn node_min(self) -> node_ptr<K, V>{
        let mut temp_node = self.clone();
        while !temp_node.get_left().is_null(){
            temp_node = temp_node.get_left(); // recursively keep getting the left child. When no left child exists, that is the smallest node
        }
        return temp_node;
    }

    #[inline]
    // Get the largest value in the tree
    fn node_max(self) -> node_ptr<K, V>{
        let mut temp_node = self.clone();
        while !temp_node.get_right().is_null(){
            temp_node = temp_node.get_right(); // recursively keep getting the right child. When no right child exists, that is the largest node
        }
        return temp_node;
    }

    #[inline]
    // Get the root of the tree
    fn get_root(self) -> node_ptr<K, V>{
        let mut temp_node = self.clone();
        while !temp_node.get_parent().is_null(){
            temp_node = temp_node.get_parent(); // recursively keep getting the parent. When no right child exists, that is the root
        }
        return temp_node;
    }


    #[inline]
    // If the nodes parent left or right child is itself -> Ergo is a child and not root
    fn is_child(self, node: node_ptr<K, V>) -> bool{
        node.get_parent().get_left() == self || node.get_parent().get_right() == self
    }

    #[inline]
    // Change key / value pair
    fn replace_val(self, node: node_ptr<K, V>){
        unsafe{
            let ans = Box::from_raw(node.0);
            let (k,v) = ans.pair(); // Get key/ value pair
            (*self.0).key = k; // Change key with k
            (*self.0).value = v; // Change value with v
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Structure of AVLTree.
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
// Made of Tree nodes in the form of poitners, and has a length (number of nodes)
pub struct AVLTree<K: Ord, V> {
    root: node_ptr<K, V>,
    len: usize,
}

///////////////////////////////////////////////////////////////////////////////
//Print Tree
///////////////////////////////////////////////////////////////////////////////

// general spacing
const PRINT_SPACE_GLOBAL: u32 = 5;

//TreeNode display
impl<K, V> Debug for TreeNode<K, V>
where
    K: Ord + Debug + fmt::Display,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {       
    write!(f, "{}", self.key.to_string().black().on_white())   // color format. Black number of white background
    }
}


//AVLTree display
impl<K: Ord + Debug + fmt::Display + Copy, V: Debug> AVLTree<K, V> {
    fn print_rec(&self, node: node_ptr<K, V>, mut space: u32) {
        if node.is_null() { //exit condition
            return;
        }

        // Print adequate spaces between nodes in display
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

    // Main tree printing function
    pub fn print_tree(&self, space: u32) {
        if self.root.is_null() {
            println!("[NOTE] Tree is Empty");
            return;
        }
        
        println!("[INFO] Tree size = {:?}", self.len());
        self.print_rec(self.root, space);
    }

    // Print portion of Tree starting from k node
    pub fn print_subtree(&self, space:u32, k: &K){
        if self.root.is_null() {
            println!("[NOTE] Tree is Empty");
            return;
        }
        let node = self.find_node(k);
        self.print_rec(node, space);
    }

    // Print the nodes is ascending order of key value
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
    
    // Other printing methods influence the node order and position for the main printing methods
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
///////////////////////////////////////////////////////////////////////////////
// AVLTree properties
///////////////////////////////////////////////////////////////////////////////

impl<K: Ord + Debug + fmt::Display, V: Debug> AVLTree<K, V> {
    // Create an empty AVLTree
    pub fn new() -> AVLTree<K, V> {
        // Tree contains null values and a length of 0
        AVLTree {
            root: node_ptr(ptr::null_mut()),
            len: 0, //total number of nodes
        }
    }

    // Returns length of AVLTree
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    //Returns true if tree is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_null()
    }

    #[inline]
    // Set a new root for the tree
    fn set_root(&mut self, mut node: node_ptr<K, V>){
        node.set_parent(node_ptr(ptr::null_mut()));
        self.root = node; // set root
    }

    #[inline]
    // Get height of the tree
    pub fn get_height(&self) -> Option<usize>{
        let mut height = 0; // initialize at 0
        if self.is_empty(){ //if empty tree
            return Some(height); //return 0
        }else{
            height = self.get_height_rec(self.root,1); // recursively increase height by 1 per node
        }
        return Some(height);
    }
    #[inline]
    fn get_height_rec(&self, node: node_ptr<K, V>, mut h: usize) -> usize{
        if node.is_null(){ // if node does not exist
            return h-1; // -1 to adjust for loop
        }
        h += 1; // increase height by 1
        let height_r = self.get_height_rec(node.get_right(), h); // recursively get hegith of right subtree
        let height_l = self.get_height_rec(node.get_left(), h); // recursively get hegith of left subtree
        max(height_r, height_l) //get the max height of subtrees
    }

    
    //Similar to BST insertion, then rebalancing is unique to AVLTree
    pub fn insert(&mut self, k: K, v: V) -> Result<(), AVLBaseErr>{
        self.len+=1; // Increase length of tree
        let mut new_node = node_ptr::new(k, v); // Creation of new node
        let mut temp_root = self.root; // Empty node to act as placeholder
        let mut temp_node: node_ptr<K, V> = node_ptr(ptr::null_mut()); // Empty node to act as placeholder

        if self.is_empty(){ //if the tree is empty set new node as the root
            self.root = new_node; //Set root
            self.update_level(new_node); // Update level of node -> 1
            return Ok(());
        }

        while !temp_root.is_null(){ //temp root isn't null (exits when reaching a null pointer at the end of the tree)
            temp_node = temp_root;
            match new_node.cmp(&&mut temp_root){ // Organizing numbers
                Ordering::Less => {temp_root = temp_root.get_left();}, // If new node is less than current node, than move the new node to the left
                Ordering::Greater => {temp_root = temp_root.get_right();}, // If new node is greater than current node, than move the new node to the right
                _ => {return Err(AVLBaseErr::DuplicateErr);} // Node already exists
            };
        }
        new_node.set_parent(temp_node); //sets parent for new_node

        match new_node.cmp(&&mut temp_node){ //adds new node to tree
            Ordering::Less => {temp_node.set_left(new_node);}, // Adding the null node to the tree (as child in empty space)
            _ => {temp_node.set_right(new_node);}
        };
        self.update_level(new_node); // Update level of the nodes -> recursively updates higher nodes.
        self.balance(new_node); // Tests for impalance -> rotates tree if there is unbalance
        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////
    // AVLTree Algorithm -> balance heights of nodes
    ///////////////////////////////////////////////////////////////////////////////
    #[inline]
    // Update the heights of nodes after an insertion, rotation, or deletion
    fn update_level(&mut self, node: node_ptr<K,V>){
        let mut left = 0; // Assume that the level of the left node is of level 0 -> required if there if the node doesn't exist
        let mut right = 0; // Assume that the level of the right node is of level 0 -> required if there if the node doesn't exist
        if !node.get_left().is_null(){ //if the left node exists (not null)
            left = node.get_left().get_level(); // get the level of the left node
        }
        if !node.get_right().is_null(){ //if the right node exists (not null)
            right = node.get_right().get_level(); // get the level of the right node
        }  
        node.set_level(max(&left, &right)+1); // Increase the level of the node by +1 of its children's height
        if !node.get_parent().is_null(){ // Not root
            self.update_level(node.get_parent()) // Recursively update heights of nodes in the subtree, up to root
        }
    }
    #[inline]
    // Get the heights difference of subtrees of the grandparent
    fn balance_factor(&self, node: node_ptr<K, V>) -> i32 {
        let mut left = 0; // Assume that the level of the left node is of level 0 -> required if there if the node doesn't exist
        let mut right = 0; // Assume that the level of the right node is of level 0 -> required if there if the node doesn't exist
        let node_gparent = node.get_parent().get_parent(); // grandparent of node

        if !node_gparent.get_left().is_null(){ //if the left node exists (not null), else is 0
            left = node_gparent.get_left().get_level(); // get the level of the left node
        }
        if !node_gparent.get_right().is_null(){ //if the right node exists (not null), else is 0
            right = node_gparent.get_right().get_level(); // get the level of the right node
        }
        left as i32 - right as i32 // get the difference between left and right nodes
    }



    // Check the level of the nodes. Apply all necessary rotations on root. 
    fn balance(&mut self, node: node_ptr<K, V>){
        let level_differance = self.balance_factor(node); // get the level differances of the nodes
        match level_differance{
            -1..=1 => { // if the level differance is -1, 0, or 1, no need to do anything -> balanced.
                if !node.get_parent().is_null(){ // Not root
                    self.balance(node.get_parent()) // Recursively check balance of tree up to root.
                }},
            2 => { // Unbalanced, Left heavy
                if node.cmp(&node.get_parent()) == Ordering::Greater {   
                    self.left_right_rotation(node)} // If key of node is greater than parent, do left_right rotation to balance
                else {
                    self.left_left_rotation(node) // left rotation
                }
            },
            -2 => { // Unbalanced, Right heavy
                if node.cmp(&node.get_parent()) == Ordering::Less {
                    self.right_left_rotation(node)} // If key of node is less than parent, do right_left rotation to balance
                else {
                    self.right_right_rotation(node) // right rotation
                }
            },
            _ => {unreachable!()} // Height differance of subtrees cannot be greater/less than +2/-2
        }
    }

    fn left_left_rotation(&mut self, node: node_ptr<K, V>){ // Perform Left rotation
        // The following lines do a three way swap between parameters of nodes
        let mut node_parent = node.get_parent();
        let mut right_parent = node_parent.get_right();
        let mut node_gparent = node_parent.get_parent();

        node_gparent.set_level(node_gparent.get_level()-2); // adjust height level

        node_gparent.set_left(right_parent);
        node_parent.set_right(node_gparent);

        let mut ggparent = node_gparent.get_parent();

        if node_gparent == ggparent.get_right(){
            ggparent.set_right(node_parent);
        }else{
            ggparent.set_left(node_parent);
        }

        if node_gparent == self.root{ // special case if node is root
            self.set_root(node_parent);
        }
        //finish swapping parameters
        right_parent.set_parent(node_gparent);
        node_gparent.set_parent(node_parent);
        node_parent.set_parent(ggparent);
        
        // updates levels and balance
        self.update_level(node);
        self.balance(node);
    }

    fn left_right_rotation(&mut self, mut node: node_ptr<K, V>){ // Perform Left rotation, The use right rotation
        // The following lines do a three way swap between parameters of nodes
        let mut right_node = node.get_right();
        let left_node = node.get_left();
        let mut node_parent = node.get_parent();
        let mut node_gparent = node_parent.get_parent();
        let mut left_parent = node_parent.get_left();

        node_parent.set_level(node_parent.get_level()-1);// adjust height level
        node.set_level(node.get_level()+1);// adjust height level

        //finish swapping parameters
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

        // activate left rotation
        self.left_left_rotation(node_parent);
    }

    fn right_right_rotation(&mut self, node: node_ptr<K, V>){ // Perform Right rotation
        // The following lines do a three way swap between parameters of nodes
        let mut node_parent = node.get_parent();
        let mut left_parent = node_parent.get_left();
        let mut node_gparent = node_parent.get_parent();

        node_gparent.set_level(node_gparent.get_level()-2); // adjust height level

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
        //finish swapping parameters
        left_parent.set_parent(node_gparent);
        node_gparent.set_parent(node_parent);
        node_parent.set_parent(ggparent);

        self.update_level(node);
        self.balance(node);
    }

    fn right_left_rotation(&mut self, mut node: node_ptr<K, V>){ // Perform Right rotation, The use left rotation
        // The following lines do a three way swap between parameters of nodes
        let mut right_node = node.get_right();
        let mut node_parent = node.get_parent();
        let mut node_gparent = node_parent.get_parent();

        node_parent.set_level(node_parent.get_level()-1); // adjust height level
        node.set_level(node.get_level()+1); // adjust height level

        //finish swapping parameters
        node_parent.set_left(right_node);
        node.set_right(node_parent);
        node_gparent.set_right(node);

        right_node.set_parent(node_parent);
        node_parent.set_parent(node);
        node.set_parent(node_gparent);

        // activate right rotation
        self.right_right_rotation(node_parent);
    }
    #[inline]
    // Remove node from tree
    fn delete(&mut self, node: node_ptr<K, V>) -> (K,V){
        // Node has previously been found
        self.len -= 1; //reduce length of deletion

        // Get empty nodes
        let mut temp_c = node_ptr(ptr::null_mut());//child
        let mut temp_p;//temp parent

        // Obtain parameters of nodes
        let mut node_r = node.get_right();//right
        let mut node_l = node.get_left();//left
        let mut node_p = node.get_parent();//parent


        //CASE: 1 child
        if !node_l.is_null() && node_r.is_null() { //has left child
            temp_c = node_l;
        } else if node_l.is_null() && !node_r.is_null() {//has right child
            temp_c = node_r;
        }
        
        //CASE: two children
        if !node_r.is_null() && !node_l.is_null() {
            
            let mut temp_node = node_r.node_min();

            if node == self.root { // special case for the root, need to assign new root
                self.root = temp_node;
            } else {
                if node_p.get_left() == node {
                    node_p.set_left(temp_node); // set left child of parent
                } else {
                    node_p.set_right(temp_node); //set right child of parent
                }
            }

            // getting parameters from nodes for replacement
            temp_c = temp_node.get_right();
            temp_p = temp_node.get_parent();

            if temp_p == node { // special case for the root, need to assign new root
                temp_p = temp_node;
            } else {
                if !temp_c.is_null() {
                    temp_c.set_parent(temp_p); //set child's parent
                }
                //set other parementers
                temp_p.set_left(temp_c);
                node_r = node.get_right();
                temp_node.set_right(node_r);
                node_r.set_parent(temp_node);
            }
            //finish setting parameters
            node_p = node.get_parent();
            node_l = node.get_left();

            temp_node.set_parent(node_p);
            temp_node.set_left(node_l);
            node_l.set_parent(temp_node);

            // updates levels and balance
            // update at current node
            self.update_level(temp_p);
            self.balance(temp_p);

            // update at largest node on small side of change
            if !temp_p.get_parent().is_null(){
                self.update_level(temp_p.get_parent().node_max()); 
                self.balance(temp_p.get_parent().node_max());
            }
    
            // update at smallest node on large side of change
            if !temp_p.get_parent().is_null(){
                self.update_level(temp_p.get_parent().node_min());
                self.balance(temp_p.get_parent().node_min());
            }

            // update at largest node overall
            self.update_level(temp_p.get_root().node_max()); 
            self.balance(temp_p.get_root().node_max());

            // update at smallest node overall
            self.update_level(temp_p.get_root().node_min());
            self.balance(temp_p.get_root().node_min());


                // return key value pair
                unsafe{
                    let ans = Box::from_raw(node.0);
                    return ans.pair();
                }
            }

        // finish setting values for single child case
        temp_p = node.get_parent();
        if !temp_c.is_null() {
            temp_c.set_parent(temp_p);
        }

        if self.root == node {
            self.root = temp_c
        } else {
            if temp_p.get_left() == node {
                temp_p.set_left(temp_c);
            } else {
                temp_p.set_right(temp_c);
            }
        }
        
        // updates levels and balance
        // update at current node
        self.update_level(temp_p);
        self.balance(temp_c);

        // update at largest node on small side of change
        if !temp_p.get_parent().is_null(){
            self.update_level(temp_p.get_parent().node_max()); 
            self.balance(temp_p.get_parent().node_max());
        }

        // update at smallest node on large side of change
        if !temp_p.get_parent().is_null(){
            self.update_level(temp_p.get_parent().node_min());
            self.balance(temp_p.get_parent().node_min());
        }

        // update at largest node overall
        self.update_level(temp_p.get_root().node_max()); 
        self.balance(temp_p.get_root().node_max());

        // update at smallest node overall
        self.update_level(temp_p.get_root().node_min());
        self.balance(temp_p.get_root().node_min());

        // self.balance(temp_p);
        
        // return key value pair
        unsafe{
            let ans = Box::from_raw(node.0);
            return ans.pair();
        }
    }

    #[inline]
    pub fn remove_node(&mut self, k: &K) -> Option<(K,V)> {
        let node = self.find_node(k); // find node with key
        if node.is_null(){ //node not found in tree
            return None;
        }
        Some(self.delete(node)) // delete that node
    }

    #[inline]
    fn find_node(&self, k: &K) -> node_ptr<K, V>{
        if self.is_empty(){ //tree is empty
            return node_ptr(ptr::null_mut());
        }

        let mut temp_node =  &self.root;
        unsafe{
            loop{ // recursively search through tree to find key 
                let next_node = match k.cmp(&(*temp_node.0).key){
                    Ordering::Less => &mut (*temp_node.0).left, // key is less, go left
                    Ordering::Greater => &mut (*temp_node.0).right, // key is greater, go right
                    Ordering::Equal => return *temp_node, // Found node
                };
                if next_node.is_null(){ // end of sub tree -> not found
                    break;
                }
                temp_node = next_node; // for looping
            }
        }
        node_ptr(ptr::null_mut()) // No node
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
