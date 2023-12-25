/*
Copyright 2023 The xflops Authors.
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#![allow(dead_code)]
#![allow(unused_variables)]
use super::Cmp;

use std::cmp::Ordering;
use std::collections;
use std::sync::Arc;

pub struct BinaryHeap<T> {
    cmp: Arc<dyn Cmp<T>>,
    heap: collections::BinaryHeap<Wrapper<T>>,
}

impl<T> BinaryHeap<T> {
    pub fn new(cmp: Arc<dyn Cmp<T>>) -> Self {
        BinaryHeap {
            cmp,
            heap: collections::BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.heap.push(Wrapper {
            data: t,
            cmp: self.cmp.clone(),
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.pop() {
            None => None,
            Some(item) => Some(item.data),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }
}

struct Wrapper<T> {
    cmp: Arc<dyn Cmp<T>>,
    data: T,
}

impl<T> Cmp<T> for Wrapper<T> {
    fn cmp(&self, t1: &T, t2: &T) -> Ordering {
        self.cmp.cmp(t1, t2)
    }
}

impl<T> Ord for Wrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp.cmp(&self.data, &other.data)
    }
}

impl<T> Eq for Wrapper<T> {}

impl<T> PartialOrd for Wrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp.cmp(&self.data, &other.data))
    }
}

impl<T> PartialEq for Wrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp.cmp(&self.data, &other.data) == Ordering::Equal
    }

    fn ne(&self, other: &Self) -> bool {
        self.cmp.cmp(&self.data, &other.data) != Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct IntCmpR {}

    impl Cmp<i32> for IntCmpR {
        fn cmp(&self, t1: &i32, t2: &i32) -> Ordering {
            if t1 == t2 {
                Ordering::Equal
            } else if t1 < t2 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }

    #[test]
    fn test_push() {
        let mut head = BinaryHeap::new(Arc::new(IntCmpR {}));
        head.push(1);
        head.push(2);

        let i = head.pop();
        assert_eq!(Some(1), i);
    }
}
