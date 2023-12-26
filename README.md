# stdng

The enhancement of Rust stand library


## Example

### BinaryHeap

```
use std::cmp::Ordering;

use stdng::collections::BinaryHeap;

struct IntCmpR {}

impl Cmp<i32> for IntCmpR {
    fn cmp(&self, t1: &i32, t2: &i32) -> Ordering {
        match t1.cmp(t2) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

fn main() {
    let mut head = BinaryHeap::new(IntCmpR {});
    head.push(1);
    head.push(2);

    let i = head.pop();
    assert_eq!(Some(1), i);
}
```
