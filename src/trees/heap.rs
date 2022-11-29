use std::collections::VecDeque;
use std::fmt::{Debug, Display};

/// A Heap is a special Tree-based data structure in which the tree is
/// a complete binary tree.
#[allow(unused)]
#[derive(Debug)]
pub struct MaxHeap<T> {
    pub tree: VecDeque<T>,
}

#[allow(unused)]
impl<T: Display + PartialEq + PartialOrd> MaxHeap<T> {
    /// Creates an empty heap
    pub fn create_heap() -> MaxHeap<T> {
        let tree: VecDeque<T> = VecDeque::new();
        MaxHeap {
            tree
        }
    }

    /// Create a heap out of given array elements
    pub fn heapify(&self, elements: &mut VecDeque<T>, index: i32) {
        let mut left = 2 * index;
        let mut right = 2 * index;
        let mut largest = index;
        if left <= elements.len() as i32
            && elements.get(left as usize) > elements.get(largest as usize) {
            largest = left;
        }
        if right <= elements.len() as i32
            && elements.get(right as usize) > elements.get(largest as usize) {
            largest = right;
        }
        if largest != index {
            elements.swap(index as usize, largest as usize);
            self.heapify(elements, largest);
        }
    }

    /// Joining two heaps to form a valid new heap containing all the
    /// elements of both, preserving the original heaps
    pub fn merge(&mut self, other_heap: MaxHeap<T>) -> Self {
        unimplemented!()
    }

    /// Joining two heaps to form a valid new heap containing all the
    /// elements of both, destroying the original heaps
    pub fn meld(&mut self, other_heap: &mut MaxHeap<T>) -> Self {
        unimplemented!()
    }
}

#[allow(unused)]
impl<T: Debug + Display + PartialEq + PartialOrd> MaxHeap<T> {
    /// find a maximum item of a max-heap
    pub fn find_max(&self) -> &T {
        let max = self.tree.front().unwrap();
        max
    }

    /// Adding a new key to the heap (aka push)
    pub fn insert(&mut self, new_key: T) {
        // 1. add element to the bottom level of the heap at the leftmost open space.
        self.tree.push_back(new_key);
        println!("Tree: {:?}", self.tree);
        loop {
            let mut element_index = self.tree.len() - 1;
            let mut element = self.tree.get(element_index);
            println!("(element, index): {:?},{:?}", element, element_index);

            let mut parent_index = ((element_index as i32 - 1) / 2) as usize;
            let mut parent = self.tree.get(parent_index);
            println!("(parent, index): {:?},{:?}", parent, parent_index);

            if !parent.is_none() && !element.is_none() && parent.unwrap() > element.unwrap() {
                // 2. Compare the added element with its parent;
                //    if the are in the correct order, stop
                break;
            } else {
                // 3. If no, swap the element with its parent and return to the previous step.
                self.tree.swap(element_index, parent_index);
            }
        }
    }

    /// Returns the node of maximum value from a max heap after removing it
    /// from the heap
    pub fn extract_max(&mut self) -> Option<T> {
        self.tree.remove(0)
    }

    /// Removing the root node of a max heap
    pub fn delete_max(&mut self) {
        self.tree.pop_front();
    }

    /// pop root and push a new key.More efficient than pop followed by push, since only
    /// need to balance once, not twice, and appropriate for fixed-size heaps.
    pub fn replace(&self, new_key: T) -> Self {
        unimplemented!()
    }
}

#[allow(unused)]
impl<T> MaxHeap<T> {
    pub fn size(&self) -> usize {
        self.tree.len()
    }
    pub fn empty(&self) -> bool {
        if self.tree.len() == 0 {
            return true;
        }
        return false;
    }
}

#[allow(unused)]
impl<T> MaxHeap<T> {
    pub fn increase_key(&mut self, key: T) -> Self {
        unimplemented!()
    }
    pub fn delete(&mut self, key: T) -> Self {
        unimplemented!()
    }
    pub fn sift_up(&mut self, key: T) -> Self {
        unimplemented!()
    }
    pub fn sift_down(&mut self, key: T) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut heap: MaxHeap<i32> = MaxHeap::create_heap();
        assert_eq!(heap.empty(), true);
        let key = 45;
        heap.insert(key);
    }

    #[test]
    fn insert() {
        let mut heap: MaxHeap<i32> = MaxHeap::create_heap();
        heap.insert(4);
        heap.insert(5);
        // assert_eq!(4, k);
    }
}