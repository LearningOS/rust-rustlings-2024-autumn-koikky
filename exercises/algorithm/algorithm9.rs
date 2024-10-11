/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

static mut Flag:usize = 0;

pub struct Heap<T>
where
    T: Default + std::clone::Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 将新的值添加到items向量的末尾
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;

        // 上滤操作，将新添加的元素与其父节点进行比较并交换位置，如果需要的话
        while idx > 1 && ((self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)])) {
            let x= self.parent_idx(idx);
            self.items.swap(idx, x);
            idx = self.parent_idx(idx);
        }
    }
    // pub fn add(&mut self, mut value: T) {
    //     //TODO
    //     let mut count = 1;
    //     let mut val_item = value.clone();
    //     //self.items.push(value);
    //     if self.count == 0 {
    //         self.items.push(value);
    //         self.count += 1;
    //         return;
    //     }
    //     while self.children_present(count) {
    //         count = self.left_child_idx(count);
    //     }
    //     if (self.count - count + 1) % count != 0 {
    //         count = self.count;
    //     } 
    //     if (self.comparator)(&value, &self.items[count - 1]) {
    //         val_item = self.items[count - 1].clone();
    //         self.items[count - 1] = value;
    //         value = val_item;
    //         val_item = self.items[count - 1].clone();
    //         count = self.parent_idx(count);
    //         if count != 0 {  
    //             while (self.comparator)(&val_item, &self.items[count - 1]) {
    //                 let x = self.items[count - 1].clone();
    //                 self.items[count - 1] = val_item;
    //                 val_item = x;
    //                 val_item = self.items[count - 1].clone();
    //                 count = self.parent_idx(count);
    //                 if count == 0 {
    //                     break;
    //                 }   
    //             }
    //         }
    //     }
    //     self.items.push(value);
    //     self.count += 1;
    //     return;
    // }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::clone::Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + std::clone::Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty() {
            None
        } else {
            self.items.swap(1, self.count);
            self.count -= 1;
            Some(self.items.pop().unwrap())
        }
    
		
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        unsafe {Flag = 0}
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(9));
    }
}


// pub struct Heap<T>
// where
//     T: Default + std::clone::Clone,
// {
//     count: usize,
//     items: Vec<T>,
//     comparator: fn(&T, &T) -> bool,
// }

// impl<T> Heap<T>
// where
//     T: Default + Clone,
// {
//     pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
//         Self {
//             count: 0,
//             items: vec![T::default()],
//             comparator,
//         }
//     }

//     pub fn len(&self) -> usize {
//         self.count
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
//     fn parent_idx(&self, idx: usize) -> usize {
//         idx / 2
//     }

//     fn children_present(&self, idx: usize) -> bool {
//         self.left_child_idx(idx) <= self.count
//     }

//     fn left_child_idx(&self, idx: usize) -> usize {
//         idx * 2
//     }

//     fn right_child_idx(&self, idx: usize) -> usize {
//         self.left_child_idx(idx) + 1
//     }
// }

// pub struct MinHeap;

// impl MinHeap {
//     #[allow(clippy::new_ret_no_self)]
//     pub fn new<T>() -> Heap<T>
//     where
//         T: Default + Ord + std::clone::Clone,
//     {
//         Heap::new(|a, b| a < b)
//     }
// }

// pub struct MaxHeap;

// impl MaxHeap {
//     #[allow(clippy::new_ret_no_self)]
//     pub fn new<T>() -> Heap<T>
//     where
//         T: Default + Ord + std::clone::Clone,
//     {
//         Heap::new(|a, b| a > b)
//     }
// }
// fn test_min_heap() {
//     let mut heap = MinHeap::new();
//     heap.add(4);
//     heap.add(2);
//     heap.add(9);
//     heap.add(11);
//     assert_eq!(heap.len(), 4);
//     assert_eq!(heap.next(), Some(2));
//     assert_eq!(heap.next(), Some(4));
//     assert_eq!(heap.next(), Some(9));
//     heap.add(1);
//     assert_eq!(heap.next(), Some(1));
// }

// fn test_max_heap() {
//     unsafe {Flag = 0}
//     let mut heap = MaxHeap::new();
//     heap.add(4);
//     heap.add(2);
//     heap.add(9);
//     heap.add(11);
//     assert_eq!(heap.len(), 4);
//     assert_eq!(heap.next(), Some(11));
//     assert_eq!(heap.next(), Some(9));
//     assert_eq!(heap.next(), Some(4));
//     heap.add(1);
//     assert_eq!(heap.next(), Some(2));
// }