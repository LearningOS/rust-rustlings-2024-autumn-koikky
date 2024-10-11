/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

// #[derive(Debug)]
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T: std::cmp::PartialOrd + Clone> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: std::cmp::PartialOrd + Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
//: std::cmp::PartialOrd
impl<T: std::cmp::PartialOrd + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    /// Merges two sorted linked lists into a new sorted linked list.
    pub fn merge_sorted_lists(list1: &mut Self, list2: &mut Self) -> LinkedList<T> {
        unsafe{
            let mut merged_list = LinkedList::new();

            let mut current1 = list1.start;
            let mut current2 = list2.start;

            while let (Some(node1), Some(node2)) = (current1, current2) {
                if (*node1.as_ptr()).val <= (*node2.as_ptr()).val {
                    merged_list.add((*node1.as_ptr()).val.clone());
                    current1 = (*node1.as_ptr()).next;
                } else {
                    merged_list.add((*node2.as_ptr()).val.clone());
                    current2 = (*node2.as_ptr()).next;
                }
            }

            // Append remaining elements from either list
            while let Some(node) = current1 {
                merged_list.add((*node.as_ptr()).val.clone());
                current1 = (*node.as_ptr()).next;
            }

            while let Some(node) = current2 {
                merged_list.add((*node.as_ptr()).val.clone());
                current2 = (*node.as_ptr()).next;
            }

            merged_list
        }
    }

	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
        todo!();
        // unsafe {
        //     //TODO
        //     let mut C_list = Self {
        //         length: 0,
        //         start: None,
        //         end: None,
        //     };
        //     let mut a_ptr:* mut Node<T> = std::ptr::null_mut(); 
        //     let mut b_ptr:* mut Node<T> = std::ptr::null_mut();
        //     if let Some(i) = list_a.start {a_ptr = i.as_ptr();}
        //     if let Some(i) = list_b.start {b_ptr = i.as_ptr();}
        //     while None != (*a_ptr).next || None != (*b_ptr).next{
        //         if (*a_ptr).val <= (*b_ptr).val {
        //             C_list.add((*a_ptr).val.clone());
        //             if let Some(i) = (*a_ptr).next {a_ptr = i.as_ptr();}
        //         } else {
        //             C_list.add((*b_ptr).val.clone());
        //             if let Some(i) = (*b_ptr).next {b_ptr = i.as_ptr();}
        //         }
                
        //     }
        //     if None == (*a_ptr).next {
        //         if let Some(i) = (*b_ptr).next {b_ptr = i.as_ptr();}
        //         C_list.add((*b_ptr).val.clone());
        //     } else {
        //         if let Some(i) = (*a_ptr).next {a_ptr = i.as_ptr();}
        //         C_list.add((*a_ptr).val.clone());
        //     }
        //     C_list 
        // }
        
    //     unsafe {
    //         //TODO
    //         let mut C_list = Self {
    //             length: 0,
    //             start: None,
    //             end: None,
    //         };
    //         let NODe = Node {
    //             val:0,
    //             next: None,
    //         };
    //         let mut a_ptr = NonNull::new(NODe.as_ptr());
    //         let mut b_ptr = NonNull::new(NODe.as_ptr());
    //         if let Some(i) = list_a.start {let mut a_ptr = NonNull::new(i.as_ptr());}
    //         if let Some(i) = list_b.start {let mut b_ptr = NonNull::new(i.as_ptr());}
    //         while None != a_ptr.as_ptr().read().next || None != b_ptr.as_ptr().read().next{
    //             if a_ptr.as_ptr().read().val <= b_ptr.as_ptr().read().val {
    //                 C_list.add(a_ptr.as_ptr().read().val);
    //                 if let Some(i) = a_ptr.as_ptr().read().next {let mut a_ptr = NonNull::new(i.as_ptr());}
    //             } else {
    //                 C_list.add(b_ptr.as_ptr().read().val);
    //                 if let Some(i) = b_ptr.as_ptr().read().next {let mut b_ptr = NonNull::new(i.as_ptr());}
    //             }
                
    //         }
    //         C_list 
    //     }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display+ std::clone::Clone+ std::cmp::PartialOrd,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge_sorted_lists(&mut list_a,&mut list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge_sorted_lists(&mut list_a,&mut list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}