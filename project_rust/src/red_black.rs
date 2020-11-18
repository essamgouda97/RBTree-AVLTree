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

#[derive(Debug)]
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
            height: 0, //TODO implement height in insertion
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

#[derive(Debug)]
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
    //TODO FIX INSERTION BALANCE FUNC
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

    * Rules 4-a rotation rules:
        1- 

    */
    fn insert_balance(&mut self, mut node: node_ptr<K, V>) -> Result<(), RBBaseErr>{

        let mut node_parent = node.get_parent();

        if node_parent.is_black(){ //3
            return Ok(());
        }

        while node.get_parent().is_red(){
            
            node_parent = node.get_parent();
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
                    if node_uncle.is_red() && !node_uncle.is_null(){ //4a
                        node_parent.set_black();
                        node_uncle.set_black();
                        node_gparent.set_red();
                        node = node_gparent;
                        continue;

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
                                node = node_parent;
                                continue;
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
                            node = node_gparent;
                            continue;
                        }

                    }

            }else{ //parent is right child of gparent
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
                if node_uncle.is_red() && !node_uncle.is_null(){
                    node_parent.set_black();
                    node_uncle.set_black();
                    node_parent.set_black();
                    node_uncle.set_black();
                    node_gparent.set_red();
                    node = node_gparent;
                    continue;

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
                        node = node_gparent;
                        continue;

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
                        node = node_gparent;
                        continue;

                    }

                    

                }

            }

    }
        
        Ok(())
    }

    fn left_left_rotation(&mut self, node: node_ptr<K, V>){
        let mut node_parent = node.get_parent();
        let right_parent = node_parent.get_right();
        let mut node_gparent = node_parent.get_parent();
        node_gparent.set_left(right_parent);
        node_parent.set_right(node_gparent);
        if node_parent == self.root{
            node_gparent.swap_color();
            return;
        }
        node_parent.swap_color();
        node_gparent.swap_color();
    }

    fn left_right_rotation(&mut self, mut node: node_ptr<K, V>){
        let right_node = node.get_right();
        let left_node = node.get_left();
        let mut node_parent = node.get_parent();
        let mut node_gparent = node_parent.get_parent();
        let left_parent = node_parent.get_left();
        node_parent.set_right(right_node);
        node_parent.set_left(left_node);
        node.set_left(node_parent);
        node.set_right(left_parent);
        node_gparent.set_left(node);
        self.left_left_rotation(node_parent);
    }

    fn right_right_rotation(&mut self, node: node_ptr<K, V>){
        let mut node_parent = node.get_parent();
        let left_parent = node_parent.get_left();
        let mut node_gparent = node_parent.get_parent();
        node_gparent.set_right(left_parent);
        node_parent.set_left(node_gparent);
        if node_parent == self.root{
            node_gparent.swap_color();
            return;
        }
        node_parent.swap_color();
        node_gparent.swap_color();
    }

    fn right_left_rotation(&mut self, mut node: node_ptr<K, V>){
        let right_node = node.get_right();
        let mut node_parent = node.get_parent();
        let mut node_gparent = node_parent.get_parent();
        node_parent.set_left(right_node);
        node.set_right(node_parent);
        node_gparent.set_right(node);
        self.right_right_rotation(node_parent);
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
        /*
This tree size = 5, begin:-------------
'k:4 v:4 c:Black' is root node
k:2 v:2 c:Black is k:4 v:4 c:Black's "left" child 
k:1 v:1 c:Red is k:2 v:2 c:Black's "left" child 
k:3 v:3 c:Red is k:2 v:2 c:Black's "right" child 
k:5 v:5 c:Black is k:4 v:4 c:Black's "right" child 
end--------------------------
        */
        unsafe{
            println!("{:?}", *(tree.root.0));
            println!("{:?}", *(tree.root.get_left().0));
            println!("{:?}", *(tree.root.get_right().0));
            println!("{:?}", *(tree.root.get_left().get_left().0));
            println!("{:?}", *(tree.root.get_left().get_right().0));
        }

        assert_eq!(tree.len(), 5);
    }

    #[test]
    fn delete_test(){

    }


}