/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
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
        println!("add");
        //TODO
        self.items.push(value);
        self.count += 1;
        // 向上调整
        let mut curIdx = self.count;
        let mut parentIdx = self.parent_idx(self.count);
        while parentIdx != 0 {
            dbg!(parentIdx);
            let parent_value = &self.items[parentIdx];
            let cur_value = &self.items[curIdx];
            if (self.comparator)(cur_value, parent_value) {
                self.items.swap(curIdx, parentIdx);
                curIdx = parentIdx;
                parentIdx = self.parent_idx(curIdx);
            }
            else {
                break;
            }
        }
    }

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
		if self.count >= 1 {
            1
        }
        else {
            0
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        println!("next");
        //TODO
        if self.count == 0 {
            return None;
        }
        else {
            // 交换1和count位置的
            self.items.swap(1, self.count);
            // 调整堆
            self.count -= 1;
            let mut idx = 1;
            while self.left_child_idx(idx) <= self.count {
                let mut min_child_idx = self.left_child_idx(idx);
                if self.right_child_idx(idx) <= self.count && (self.comparator)(&self.items[self.right_child_idx(idx)], &self.items[self.left_child_idx(idx)]) {
                    min_child_idx += 1;
                }
                // 调整
                if (self.comparator)(&self.items[min_child_idx], &self.items[idx]) {
                    // 交换
                    self.items.swap(idx, min_child_idx);
                    idx = min_child_idx;
                }
                else {
                    break;
                }
            }
            self.items.pop()
            // 移除count位置的
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        dbg!("test1");
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        dbg!("test2");
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        dbg!("finish add");
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}