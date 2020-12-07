mod red_black;
mod avl;
use self::red_black::RBTree;
use self::avl::AVLTree;
use std::io::{self,Write};
//mod testy;

fn rb_insert(tree: &mut RBTree<usize, usize>){
    loop{
        print!("\nEnter node value: ");
        io::stdout().flush().unwrap();
        let node_v = match input_num(){
            Some(-1) => {break},
            Some(i) => i,
            _ => {continue}
        };
        match tree.insert(node_v as usize, node_v as usize){
            Ok(_) => println!("[SUCCESS] Value inserted"),
            _ => println!("[ERROR] Duplicate Value !")
        }
    }

}
fn rb_delete(tree: &mut RBTree<usize, usize>){
    loop{
        print!("\nEnter node value: ");
        io::stdout().flush().unwrap();
        let node_v = match input_num(){
            Some(-1) => {break},
            Some(i) => i,
            _ => {continue}
        };
        match tree.remove_node(&(node_v as usize)){
            Some(i) => println!("[SUCCESS] Value {:?} deleted", i),
            _ => println!("[ERROR] Value not found !")
        }
    }
}
fn rb_count(tree: &RBTree<usize, usize>){
    println!("Tree has a total of {} leaves", tree.len());
}
fn rb_height(tree: &RBTree<usize, usize>){
    println!("Height of tree = {}", tree.get_height().unwrap());
}
fn rb_print(tree: &RBTree<usize, usize>){
    loop{
        println!("\nChoose printing style:");
        println!("(Note for large trees printing the whole tree won't be displayed properly)");
        println!("(1) Print whole tree");
        println!("(2) Print subtree");
        println!("(3) Inorder Tree Traversal");
        println!("(4) Preorder Tree Traversal");
        println!("(5) Postorder Tree Traversal");
        print!("Choice: ");
        io::stdout().flush().unwrap();
        let choice = input_num();
        match choice{
            Some(1) => {tree.print_tree(1);break;},
            Some(2) => {},
            Some(3) => {tree.inorder_trav_print();break;},
            Some(4) => {tree.preorder_trav_print();break;},
            Some(5) => {tree.postorder_trav_print();break;},
            Some(-1) => {break;}
            _ => {println!("Choose a valid option");continue;}
        }
        print!("Input root node of subtree: ");
        io::stdout().flush().unwrap();
        let node_v = match input_num(){
            Some(-1) => {break;},
            Some(i) => i,
            _ => {println!("Error");break;}
        };
        tree.print_subtree(1, &(node_v as usize));

    }
}
fn rb_empty(tree: &RBTree<usize, usize>){
    if tree.is_empty(){
        println!("Tree is empty");
    }else{
        println!("Tree isn't empty, has a total of {} leaves.", tree.len());
    }
}

fn avl_insert(tree: &mut AVLTree<usize, usize>){
    loop{
        print!("\nEnter node value: ");
        io::stdout().flush().unwrap();
        let node_v = match input_num(){
            Some(-1) => {break},
            Some(i) => i,
            _ => {continue}
        };
        match tree.insert(node_v as usize, node_v as usize){
            Ok(_) => println!("[SUCCESS] Value inserted"),
            _ => println!("[ERROR] Duplicate Value !")
        }
    }

}
fn avl_delete(tree: &mut AVLTree<usize, usize>){
    loop{
        print!("\nEnter node value: ");
        io::stdout().flush().unwrap();
        let node_v = match input_num(){
            Some(-1) => {break},
            Some(i) => i,
            _ => {continue}
        };
        match tree.remove_node(&(node_v as usize)){
            Some(i) => println!("[SUCCESS] Value {:?} deleted", i),
            _ => println!("[ERROR] Value not found !")
        }
    }
}
fn avl_count(tree: &AVLTree<usize, usize>){
    println!("Tree has a total of {} leaves", tree.len());
}
fn avl_height(tree: &AVLTree<usize, usize>){
    println!("Height of tree = {}", tree.get_height().unwrap());
}
fn avl_print(tree: &AVLTree<usize, usize>){
    loop{
        println!("\nChoose printing style:");
        println!("(Note for large trees printing the whole tree won't be displayed properly)");
        println!("(1) Print whole tree");
        println!("(2) Print subtree");
        println!("(3) Inorder Tree Traversal");
        println!("(4) Preorder Tree Traversal");
        println!("(5) Postorder Tree Traversal");
        print!("Choice: ");
        io::stdout().flush().unwrap();
        let choice = input_num();
        match choice{
            Some(1) => {tree.print_tree(1);break;},
            Some(2) => {},
            Some(3) => {tree.inorder_trav_print();break;},
            Some(4) => {tree.preorder_trav_print();break;},
            Some(5) => {tree.postorder_trav_print();break;},
            Some(-1) => {break;}
            _ => {println!("Choose a valid option");continue;}
        }
        print!("Input root node of subtree: ");
        io::stdout().flush().unwrap();
        let node_v = match input_num(){
            Some(-1) => {break;},
            Some(i) => i,
            _ => {println!("Error");break;}
        };
        tree.print_subtree(1, &(node_v as usize));

    }
}
fn avl_empty(tree: &AVLTree<usize, usize>){
    if tree.is_empty(){
        println!("Tree is empty");
    }else{
        println!("Tree isn't empty, has a total of {} leaves.", tree.len());
    }
}

fn input_num() -> Option<i8> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer){
        Ok(_) => match buffer.trim_end().parse(){
            Ok(x) => Some(x),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

fn print_welcome_tree(maxsize: u32){
    let mut x: u32;
    let mut k: u32 = 1;
    let mut z: u32 = 2;
    let mut r: u32 = maxsize/2;
    for j in (0..=r).rev(){
        for i in 1..=j{
            x = k*2;
            print!(" ");
            if i == j{
                while x != 0{
                    print!("@");
                    x-= 1;
                }
                break;
            }
        }
        if j != 0{
            print!("\n");
        }
        k += 1;
    }

    while z != 0{
        while r != 0{
            print!(" ");
            r -= 1;
        }
        println!("##");
        z -= 1;
        r = maxsize/2;
    }
}

fn main() {
    println!("\n\tTrees, Trees and More Trees\n");
    print_welcome_tree(37);
    loop{
        
        println!("\nPlease Choose Tree type: (Input -1 anytime to quit)");
        println!("(1) Red-Black Tree");
        println!("(2) AVL Tree");

        print!("Choice: ");
        io::stdout().flush().unwrap();
        let tree_choice = input_num();
        if tree_choice.is_none(){
            continue;
        }
        match tree_choice{
            Some(1) => {rb_tree_create();break;},
            Some(2) => {avl_tree_create();break;},
            Some(-1) => {println!("Thank you, have a great one !"); break;}
            _ => {
                println!("Invalid choice ! Please try again.");
                continue;
            }
        }

    }

}

fn rb_tree_create(){
    let mut tree: RBTree<usize, usize> = RBTree::new();
    loop{
        
        println!("\nRed-Black Tree selected, here is your options: ");
        println!("(1) Insert node");
        println!("(2) Delete node");
        println!("(3) Count number of leaves");
        println!("(4) Return height of Tree");
        println!("(5) Print Tree");
        println!("(6) Check if tree is empty");
        println!("(-1) Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();
        let choice = input_num();
        if choice.is_none(){
            continue;
        }
        match choice{
            Some(1) => {rb_insert(&mut tree);continue;},
            Some(2) => {rb_delete(&mut tree);continue;},
            Some(3) => {rb_count(&tree);continue;},
            Some(4) => {rb_height(&tree);continue;},
            Some(5) => {rb_print(&tree);continue;},
            Some(6) => {rb_empty(&tree);continue;},
            Some(-1) => {break;},
            _ => {
                println!("Invalid choice ! Please try again.");
                continue;
            }
        }
    }
}

fn avl_tree_create(){
    let mut tree: AVLTree<usize, usize> = AVLTree::new();
    loop{
        
        println!("\nAVL Tree selected, here is your options: ");
        println!("(1) Insert node");
        println!("(2) Delete node");
        println!("(3) Count number of leaves");
        println!("(4) Return height of Tree");
        println!("(5) Print Tree");
        println!("(6) Check if tree is empty");
        println!("(-1) Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();
        let choice = input_num();
        if choice.is_none(){
            continue;
        }
        match choice{
            Some(1) => {avl_insert(&mut tree);continue;},
            Some(2) => {avl_delete(&mut tree);continue;},
            Some(3) => {avl_count(&tree);continue;},
            Some(4) => {avl_height(&tree);continue;},
            Some(5) => {avl_print(&tree);continue;},
            Some(6) => {avl_empty(&tree);continue;},
            Some(-1) => {break;},
            _ => {
                println!("Invalid choice ! Please try again.");
                continue;
            }
        }
    }
}


