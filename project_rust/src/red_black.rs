/*
1- Insert a node to the red-black tree.
2- Delete a node from the red-black tree.
3- Count the number of leaves in a tree.
4- Return the height of a tree.
5- Print In-order traversal of the tree.
6- Check if the tree is empty.
*/

//TODO IMPLEMENT INTOITER
//TODO BENCHMARKS and BENCHMARKS GRAPHS

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
pub enum RBBaseErr {
    DuplicateErr,
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


struct TreeNode<K: Ord, V> { //each tree node has
    color: NodeColor,  //node color either red or black (red for new nodes, black for root)
    key: K, //key of node
    value: V, //value of node
    parent: node_ptr<K, V>, // reference to parent
    left: node_ptr<K, V>, //reference to left child
    right: node_ptr<K, V>, //reference to get_right child
    level: usize, //height of node
}

impl<K: Ord, V> TreeNode<K, V> {
    #[inline]
    fn pair(self) -> (K, V){
        (self.key, self.value)
    }
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
    fn cmp(&self, other_node: &node_ptr<K, V>) -> Ordering {
        unsafe{
            (*self.0).key.cmp(&(*other_node.0).key)
        }
    }
}

impl<K: Ord, V> Eq for node_ptr<K, V> {} //requried for Ord

impl<K: Ord, V> PartialOrd for node_ptr<K, V> { //required for Ord
    fn partial_cmp(&self, other_node: &node_ptr<K, V>) -> Option<Ordering> {
        unsafe { Some((*self.0).key.cmp(&(*other_node.0).key)) }
    }
}

impl<K: Ord, V> PartialEq for node_ptr<K, V> { //required for Eq and PartialOrd
    fn eq(&self, other_node: &node_ptr<K, V>) -> bool {
        self.0 == other_node.0
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
            level: 0, //TODO implement height in insertion
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

    #[inline]
    fn swap_color(&mut self){
        if self.is_red(){
            self.set_black();
        }else{
            self.set_red();
        }
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

    //setting get_right child of current node
    #[inline]
    fn set_right(&mut self, get_right: node_ptr<K, V>){
        if self.is_null(){
            return;
        }
        unsafe{
            (*self.0).right = get_right
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

    //getting clone of current node get_right child
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

    //check if current node is get_right child
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

    #[inline]
    fn node_min(self) -> node_ptr<K, V>{
        let mut temp_node = self.clone();
        while !temp_node.get_left().is_null(){
            temp_node = temp_node.get_left();
        }

        return temp_node;
    }

    #[inline]
    fn is_child(self, node: node_ptr<K, V>) -> bool{
        node.get_left() == self || node.get_right() == self
    }

    #[inline]
    fn replace_val(self, node: node_ptr<K, V>){
        unsafe{
            let ans = Box::from_raw(node.0);
            let (k,v) = ans.pair();
            (*self.0).key = k;
            (*self.0).value = v;
        }
    }

}

#[derive(Debug)]
pub struct RBTree<K: Ord, V> {
    root: node_ptr<K, V>,
    len: usize,
}

//######################## PRINTING TREE ################################

const PRINT_SPACE_GLOBAL: u32 = 5;

//TreeNode display
impl<K, V> Debug for TreeNode<K, V>
where
    K: Ord + Debug + fmt::Display,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.color == NodeColor::Black{
            
            write!(f, "{}", self.key.to_string().black().on_white())
        }else{
            write!(f, "{}", self.key.to_string().red().on_white())
        }
        
    }
}


//RBTree display
impl<K: Ord + Debug + fmt::Display, V: Debug> RBTree<K, V> {
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

    pub fn print_subtree(&self, space:u32, k: &K){
        if self.root.is_null() {
            println!("[NOTE] Tree is Empty");
            return;
        }
        let node = self.find_node(k);
        self.print_rec(node, space);
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

impl<K: Ord + Debug + fmt::Display, V: Debug> RBTree<K, V> {
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
    #[inline]
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
                Ordering::Greater => {
                    temp_root = temp_root.get_right();
                },
                _ => {
                    return Err(RBBaseErr::DuplicateErr);
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
    /*
    * RBTree Properties:
        1- Root is always black (done)
        2- Every null leaf is black (done)
        3- If the node is red then children are black
        4- Every path from a node to any of its descendent Null nodes has same number of black nodes
    
    * RBTree insertion rules:
        1- If the tree is empty, create new node as root node with color black (done)
        2- If tree is not empty, create new node as leaf node with color Red (done)
        3- If parent of new node is black then exit (done)
        4- If parent of new node is Red, then check the color of parents sibling of new node:
            a- If color is black or null then do suitable rotation and recolor
            b- If color is Red, then recolor and also check if parent's parent of newnode is not root node then recolor it and recheck (done)

    */
    #[inline]
    fn insert_balance(&mut self, mut node: node_ptr<K, V>) -> Result<(), RBBaseErr>{
        
        let mut node_parent = node.get_parent();
        
        //self.print_tree(1);
        
        
        
        //println!("{}", node == self.root);
        if node == self.root{
            node.set_black();
            return Ok(());
        }else if node_parent == self.root{
            node_parent.set_black();
            return Ok(());
        }
        
        // unsafe{
        //     println!("{:?} {:?}", *node.0, *node_parent.0);
        // }

        if node_parent.is_black(){ //3
            return Ok(());
        }

        
        

        if node_parent.is_red(){

            let mut node_gparent = node_parent.get_parent();


            if node_parent == node_gparent.get_left(){ 
                /*
                 G
                / \
                P           
                */
                let mut node_uncle = node_gparent.get_right();
                /*
                 G
                / \
                P  U        
                */
                    if node_uncle.is_red(){ //4a
                        node_parent.set_black();
                        node_uncle.set_black();
                        node_gparent.set_red();
                        self.insert_balance(node_gparent).unwrap();

                    }else{ //uncle is black or null
                        //1: Left Left Case(LL rotation)
                        /*
                         G
                        / \
                       P  U 
                      /
                     N
                        */
                        if node_parent.get_left() == node{
                                self.left_left_rotation(node);
                        }
                        //2: Left Right Case (LR rotation)
                        /*
                         G
                        / \
                       P  U 
                        \
                        N
                        */
                        else{
                            self.left_right_rotation(node);
                        }

                    }

            }else{ //parent is get_right child of gparent
                /*
                 G
                / \
                    P           
                */
                let mut node_uncle = node_gparent.get_left();
                /*
                 G
                / \
                U  P   
                */
                if node_uncle.is_red(){
                    node_parent.set_black();
                    node_uncle.set_black();
                    node_gparent.set_red();
                    self.insert_balance(node_gparent).unwrap();

                }else{ //uncle is black or null
                    //3: Right Right Case(RR rotation)
                    /*
                         G
                        / \
                       U  P 
                           \
                            N
                    */
                    if node_parent.get_right() == node{
                        self.right_right_rotation(node);

                    }
                    //4: Rigth Left Case (RL rotation)
                    /*
                         G
                        / \
                       U  P 
                         /
                        N
                     */
                    else{
                        self.right_left_rotation(node);

                    }

                    

                }

            }

    }
    if node == self.root{
        node.set_black();
    }
        
        Ok(())
    }

    #[inline]
    fn left_left_rotation(&mut self, node: node_ptr<K, V>){
        let mut node_parent = node.get_parent();
        let mut right_parent = node_parent.get_right();
        let mut node_gparent = node_parent.get_parent();

        node_gparent.set_left(right_parent);
        node_parent.set_right(node_gparent);

        node_parent.set_black();
        node_gparent.set_red();

        let mut ggparent = node_gparent.get_parent();

        if node_gparent == ggparent.get_right(){
            //println!("here");
            ggparent.set_right(node_parent);
        }else{
            //println!("there");
            ggparent.set_left(node_parent);
        }

        if node_gparent == self.root{
            self.set_root(node_parent);
        }
        
        right_parent.set_parent(node_gparent);
        node_gparent.set_parent(node_parent);
        node_parent.set_parent(ggparent);

    }

    #[inline]
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

    #[inline]
    fn right_right_rotation(&mut self, node: node_ptr<K, V>){
        let mut node_parent = node.get_parent();
        let mut left_parent = node_parent.get_left();
        let mut node_gparent = node_parent.get_parent();

        
        node_gparent.set_right(left_parent);
        node_parent.set_left(node_gparent);

        node_parent.set_black();
        node_gparent.set_red();
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

    #[inline]
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

    #[inline]
    fn delete(&mut self, mut node: node_ptr<K, V>) -> (K,V){
        //self.print_tree(1);

        self.len -= 1;

        let mut temp_c = node_ptr(ptr::null_mut());
        let mut temp_p;
        let temp_color;

        let mut node_r = node.get_right();
        let mut node_l = node.get_left();
        let mut node_p = node.get_parent();


        //CASE: 1 child
        if !node_l.is_null() && node_r.is_null() {
            temp_c = node_l;
        } else if node_l.is_null() && !node_r.is_null() {
            temp_c = node_r;
        }
        
        //CASE: two children
        if !node_r.is_null() && !node_l.is_null() {
            
            let mut temp_node = node_r.node_min();

            if node == self.root {
                self.root = temp_node;
            } else {
                if node_p.get_left() == node {
                    node_p.set_left(temp_node);
                } else {
                    node_p.set_right(temp_node);
                }
            }

            temp_c = temp_node.get_right();
            temp_p = temp_node.get_parent();
            temp_color = temp_node.get_color();

            if temp_p == node {
                temp_p = temp_node;
            } else {
                if !temp_c.is_null() {
                    temp_c.set_parent(temp_p);
                }
                temp_p.set_left(temp_c);
                node_r = node.get_right();
                temp_node.set_right(node_r);
                node_r.set_parent(temp_node);
            }

            node_p = node.get_parent();
            node_l = node.get_left();

            temp_node.set_parent(node_p);
            temp_node.set_color(node.get_color());
            temp_node.set_left(node_l);
            node_l.set_parent(temp_node);

            if temp_color == NodeColor::Black {
                self.delete_balance(temp_c, temp_p).unwrap();
            }
            unsafe{
                let ans = Box::from_raw(node.0);
                return ans.pair();
            }
        }

        

        temp_p = node.get_parent();
        temp_color = node.get_color();
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

        if temp_color == NodeColor::Black {
            self.delete_balance(temp_c, temp_p).unwrap();
        }
        unsafe{
            let ans = Box::from_raw(node.0);
            return ans.pair();
        }

    }

    #[inline]
    fn delete_balance(&mut self, mut node: node_ptr<K, V>, mut parent: node_ptr<K,V>) -> Result<(), RBBaseErr>{

        let mut temp_node;
        let mut node_p_l;
        let mut node_p_r;

        while node != self.root && node.is_black() {
            node_p_l = parent.get_left();
            node_p_r = parent.get_right();
            if  node_p_l == node {
                temp_node = node_p_r;

                if temp_node.is_red() {
                    temp_node.set_black();
                    parent.set_red();
                    self.rotate_left(parent);
                    temp_node = parent.get_right();
                }

                if temp_node.get_left().is_black() && temp_node.get_right().is_black() {
                    temp_node.set_red();
                    node = parent;
                    parent = node.get_parent();
                } else {

                    if temp_node.get_right().is_black() {
                        temp_node.get_left().set_black();
                        temp_node.set_red();
                        self.rotate_right(temp_node);
                        temp_node = parent.get_right();
                    }

                    temp_node.set_color(parent.get_color());
                    parent.set_black();
                    temp_node.get_right().set_black();
                    self.rotate_left(parent);
                    node = self.root;
                    break;
                }
            } else {
                temp_node = parent.get_left();

                if temp_node.is_red() {
                    temp_node.set_black();
                    parent.set_red();
                    self.rotate_right(parent);
                    temp_node = parent.get_left();
                }


                if temp_node.get_left().is_black() && temp_node.get_right().is_black() {
                    temp_node.set_red();
                    node = parent;
                    parent = node.get_parent();
                } else {

                    if temp_node.get_left().is_black() {
                        temp_node.get_right().set_black();
                        temp_node.set_red();
                        self.rotate_left(temp_node);
                        temp_node = parent.get_left();
                    }

                    temp_node.set_color(parent.get_color());
                    parent.set_black();
                    temp_node.get_left().set_black();
                    self.rotate_right(parent);
                    node = self.root;
                    break;
                }
            }
        }

        node.set_black();

        Ok(())
    }

    #[inline]
    fn rotate_left(&mut self, mut node: node_ptr<K, V>) {
        let mut temp_node = node.get_right();
        let mut temp_node_l = temp_node.get_left();
        let mut node_p = node.get_parent();

        node.set_right(temp_node_l);

        if !temp_node_l.is_null() {
            temp_node_l.set_parent(node);
        }

        temp_node.set_parent(node_p);
        if node == self.root {
            self.root = temp_node;
        } else{
            if node == node_p.get_left() {
                node_p.set_left(temp_node);
            } else {
                node_p.set_right(temp_node);
            }
        }
        temp_node.set_left(node);
        node.set_parent(temp_node);
    }

    #[inline]
    fn rotate_right(&mut self, mut node: node_ptr<K, V>) {
        let mut temp_node = node.get_left();
        let mut temp_node_r = temp_node.get_right();
        let mut node_p = node.get_parent();
        node.set_left(temp_node_r);

        if !temp_node_r.is_null() {
            temp_node_r.set_parent(node);
        }

        temp_node.set_parent(node_p);
        if node == self.root {
            self.root = temp_node;
        } else{
            if node == node_p.get_right() {
                node_p.set_right(temp_node);
            } else {
                node_p.set_left(temp_node);
            }
        }
        temp_node.set_right(node);
        node.set_parent(temp_node);
    }


    #[inline]
    pub fn remove_node(&mut self, k: &K) -> Option<(K,V)> {
        let node = self.find_node(k);
        if node.is_null(){ //node not found in tree
            return None;
        }

        Some(self.delete(node))

    }

    #[inline]
    fn find_node(&self, k: &K) -> node_ptr<K, V>{
        if self.is_empty(){ //tree is empty
            return node_ptr(ptr::null_mut());
        }

        let mut temp_node =  &self.root;
        unsafe{
            loop{
                let next_node = match k.cmp(&(*temp_node.0).key){
                    Ordering::Less => &mut (*temp_node.0).left,
                    Ordering::Greater => &mut (*temp_node.0).right,
                    Ordering::Equal => return *temp_node,
                };
                if next_node.is_null(){
                    break;
                }
                temp_node = next_node;
            }
        }
        node_ptr(ptr::null_mut())

    }

    #[inline]
    fn set_root(&mut self, mut node: node_ptr<K, V>){
        node.set_parent(node_ptr(ptr::null_mut()));
        self.root = node;
        
    }

    #[inline]
    pub fn get_height(&self) -> Option<usize>{
        let mut height = 0;
        if self.is_empty(){
            return Some(height);
        }else{
            
            height = self.get_height_rec(self.root,1);

        }

        return Some(height);
    }

    #[inline]
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
        let mut tree: RBTree<usize, usize> = RBTree::new();
        
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
        //println!("{}", "test".white().on_black());

        //assert_eq!(tree.len(), 9);
    }

    #[test]
    fn height_test(){
        let mut tree: RBTree<usize, usize> = RBTree::new();

        tree.insert(4, 4).unwrap();
        tree.insert(5, 5).unwrap();
        tree.insert(2, 2).unwrap();
        tree.insert(1, 1).unwrap();
        tree.insert(3, 3).unwrap();
        tree.insert(6, 6).unwrap();
        tree.insert(0, 0).unwrap();
        tree.insert(9, 9).unwrap();

        let h = tree.get_height().unwrap();
        //tree.print_tree(1);

        //tree.print_subtree(1, &2);
        println!("{}", h);
    }

    #[test]
    fn print_inorder_test(){
        let mut tree: RBTree<usize, usize> = RBTree::new();

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
        let mut tree: RBTree<usize, usize> = RBTree::new();

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
        let mut tree: RBTree<usize, usize> = RBTree::new();

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

    #[test]
    fn delete_test(){
        let mut tree: RBTree<usize, usize> = RBTree::new();

        
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

        tree.remove_node(&11).unwrap();

        tree.print_tree(1);
    }
}