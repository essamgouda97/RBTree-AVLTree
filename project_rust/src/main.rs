mod red_black;
mod avl;
use self::red_black::RBTree;

fn main() {
    let mut tree: RBTree<usize, usize> = RBTree::new();

    println!("Is the tree empty?: {}", tree.is_empty());
        
    tree.insert(4, 4).unwrap();
    tree.insert(5, 5).unwrap();
    tree.insert(1, 1).unwrap();
    tree.delete(5, 5);
    println!("Tree height is: {:?}", tree.get_height());
    println!("The number of leaves in the red black tree is: {}", tree.len());

    tree.inorder_trav_print();
    println!("Is the tree empty?: {}", tree.is_empty());
}
