/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug,Clone)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// 递归插入方法
    fn insert_recursive(&mut self, mut node: Box<TreeNode<T>>, value: T) -> Box<TreeNode<T>> {
        if value < node.value {
            if let Some(mut left) = node.left.take() {
                left = self.insert_recursive(left, value);
                node.left = Some(left);
            } else {
                node.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(mut right) = node.right.take() {
                right = self.insert_recursive(right, value);
                node.right = Some(right);
            } else {
                node.right = Some(Box::new(TreeNode::new(value)));
            }
        }
        node
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.search(value.clone()) {
            return;
        }
        if let Some(mut root_node) = self.root.take() {
            root_node = self.insert_recursive(root_node, value);
            self.root = Some(root_node);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
        //TODO
        // unsafe {
        //     let node_stock = Box::new(TreeNode::new(value.clone()));
        //     let mut node_tree = &self.root;
        //     if let Some(node) = node_tree {
        //         loop {
        //             let mut node_val = (*Box::into_raw((*node).clone())).value.clone();
        //             while value < node_val {
        //                 if let Some(node) = node_tree {
        //                     if let None = (*Box::into_raw((*node).clone())).left {
        //                         //(*(*node)).left = Some(node_stock);
        //                         (*Box::into_raw((*node).clone())).left = Some(node_stock);
        //                         return;
        //                     } else {
        //                         node_tree = &(*Box::into_raw((*node).clone())).left;
        //                         node_val = (*Box::into_raw((*node).clone())).value.clone();
        //                     }
        //                 }
        //             }
        //             while value > node_val {
        //                 if let Some(node) = node_tree {
        //                     if let None = (*Box::into_raw((*node).clone())).right {
        //                         (*Box::into_raw((*node).clone())).right = Some(node_stock);
        //                         return;
        //                     } else {
        //                         node_tree = &(*Box::into_raw((*node).clone())).right;
        //                         node_val = (*Box::into_raw((*node).clone())).value.clone();
        //                     }
        //                 }
        //             }
        //         }
        //     } else {
        //         self.root = Some(node_stock);
        //         return;
        //     }
        // }
    }
    fn search_recursive(&self, node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => {
                if value == &n.value {
                    true
                } else if value < &n.value {
                    self.search_recursive(&n.left, value)
                } else {
                    self.search_recursive(&n.right, value)
                }
            }
        }
    }
    // // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        self.search_recursive(&self.root, &value)
        //TODO
        // unsafe {
        //     let mut node_tree = &self.root;
        //     //let mut node_val:T;
        //     if let Some(node) = node_tree {
        //         let mut node_val = (*Box::into_raw((*node).clone())).value.clone();
            
        //         loop {
        //             while value < node_val {
        //                 if let Some(node) = node_tree {
        //                     node_tree = &(*Box::into_raw((*node).clone())).left;
        //                     node_val = (*Box::into_raw((*node).clone())).value.clone();
        //                 }
        //                 else {
        //                     return false;
        //                 }
        //             }
        //             while value > node_val {
        //                 if let Some(node) = node_tree {
        //                     node_tree = &(*Box::into_raw((*node).clone())).right;
        //                     node_val = (*Box::into_raw((*node).clone())).value.clone();
        //                 }
        //                 else {
        //                     return false;
        //                 }
        //             }   
        //             if value == node_val {
        //                 return true;
        //             }
        //         }
        //     } else {return false;}
        // }
    }
    // Search for a value in the BST
    // fn search(&self, value: T) -> bool {
    //     //TODO
    //     unsafe {
    //         let mut node_tree = &self.root;
    //         let mut node_val:T;
    //         if let Some(node) = node_tree {
    //             node_val = (*node).value.clone();
    //         }
    //         loop {
    //             while value < node_val {
    //                 if let Some(node) = node_tree {
    //                     node_tree = &(*node).left;
    //                     node_val = (*node).value.clone();
    //                 }
    //                 else {
    //                     return false;
    //                 }
    //             }
    //             while value > node_val {
    //                 if let Some(node) = node_tree {
    //                     node_tree = &(*node).right;
    //                     node_val = (*node).value.clone();
    //                 }
    //                 else {
    //                     return false;
    //                 }
    //             }   
    //             if value == node_val {
    //                 return true;
    //             }
    //         }
    //     }
    // }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        self.value = value;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


