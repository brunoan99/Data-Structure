use crate::Stack;

// Invariants:
//    - |head| = len_head;
//    - |tail| = len_tail;
//    - |head| >= |tail|
#[derive(Clone, PartialEq, Debug)]
pub struct BankerQueue<T> {
  pub head: Stack<T>,
  pub len_head: i32,
  pub tail: Stack<T>,
  pub len_tail: i32,
}

impl<T> BankerQueue<T>
where
  T: PartialEq + Clone + Copy,
{
  pub fn new() -> Self {
    todo!()
  }

  pub fn queue() -> Self {
    todo!()
  }

  pub fn is_empty(queue: &Self) -> bool {
    let _ = queue;
    todo!()
  }

  pub fn enqueue(queue: &Self, item: T) -> Self {
    let _ = queue;
    let _ = item;
    todo!()
  }

  pub fn dequeue(queue: &Self) -> Option<(T, Self)> {
    let _ = queue;
    todo!()
  }

  pub fn drop(queue: &Self) -> Option<Self> {
    let _ = queue;
    todo!()
  }

  pub fn head(queue: &Self) -> Option<T> {
    let _ = queue;
    todo!()
  }

  pub fn daeh(queue: &Self) -> Option<T> {
    let _ = queue;
    todo!()
  }

  pub fn len(queue: &Self) -> i32 {
    let _ = queue;
    todo!()
  }

  pub fn rev(queue: &Self) -> Self {
    let _ = queue;
    todo!()
  }

  pub fn concat(q1: &Self, q2: &Self) -> Self {
    let _ = q1;
    let _ = q2;
    todo!()
  }

  pub fn split(queue: &Self, f: fn(&T) -> bool) -> (Self, Self) {
    let _ = queue;
    let _ = f;
    todo!()
  }

  pub fn any(queue: &Self, f: fn(&T) -> bool) -> bool {
    let _ = queue;
    let _ = f;
    todo!()
  }

  pub fn all(queue: &Self, f: fn(&T) -> bool) -> bool {
    let _ = queue;
    let _ = f;
    todo!()
  }

  pub fn find(queue: &Self, f: fn(&T) -> bool) -> Option<&T> {
    let _ = queue;
    let _ = f;
    todo!()
  }

  pub fn find_r(queue: &Self, f: fn(&T) -> bool) -> Option<&T> {
    let _ = queue;
    let _ = f;
    todo!()
  }

  pub fn map<U>(queue: &Self, f: fn(&T) -> U) -> BankerQueue<U>
  where
    U: Clone + PartialEq + Copy,
  {
    let _ = queue;
    let _ = f;
    todo!()
  }

  pub fn filter(queue: &Self, f: fn(&T) -> bool) -> Self {
    let _ = queue;
    let _ = f;
    todo!()
  }

  pub fn reduce<U>(queue: &Self, f: fn(&T, U) -> U, acc: U) -> U {
    let _ = queue;
    let _ = f;
    let _ = acc;
    todo!()
  }
}

#[cfg(test)]
#[path = "./banker-queue_test.rs"]
mod test;
