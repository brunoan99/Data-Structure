use crate::Stack;

// Invariants:
//    - |head| = len_head;
//    - |tail| = len_tail;
//    - |head| >= |tail|
#[derive(Clone, PartialEq, Debug)]
pub struct BankerQueue<T> {
  head: Stack<T>,
  len_head: i32,
  tail: Stack<T>,
  len_tail: i32,
}

mod private {
  use super::Stack;

  pub fn split_stack_aux<T>(
    stack: &Stack<T>,
    f: fn(&T) -> bool,
    acc1: Stack<T>,
    count1: i32,
    acc2: Stack<T>,
    count2: i32,
  ) -> (Stack<T>, i32, Stack<T>, i32)
  where
    T: PartialEq + Clone + Copy,
  {
    match stack {
      Stack::Empty => (Stack::rev(&acc1), count1, Stack::rev(&acc2), count2),
      Stack::Node(value, stack_remaining) => {
        if f(value) {
          split_stack_aux(
            stack_remaining,
            f,
            acc1,
            count1,
            Stack::push(&acc2, value.clone()),
            count2 + 1,
          )
        } else {
          split_stack_aux(
            stack_remaining,
            f,
            Stack::push(&acc1, value.clone()),
            count1 + 1,
            acc2,
            count2,
          )
        }
      }
    }
  }
}

impl<T> BankerQueue<T>
where
  T: PartialEq + Clone + Copy,
{
  pub fn new() -> Self {
    Self {
      head: Stack::Empty,
      len_head: 0,
      tail: Stack::Empty,
      len_tail: 0,
    }
  }

  pub fn queue(head: &Stack<T>, len_head: i32, tail: &Stack<T>, len_tail: i32) -> Self {
    if len_head <= len_tail {
      BankerQueue {
        head: head.clone(),
        len_head,
        tail: tail.clone(),
        len_tail,
      }
    } else {
      BankerQueue {
        head: Stack::Empty,
        len_head: 0,
        tail: Stack::concat(tail, &Stack::rev(head)),
        len_tail: len_tail + len_head,
      }
    }
  }

  pub fn is_empty(queue: &Self) -> bool {
    queue.len_tail == 0
  }

  pub fn enqueue(queue: &Self, item: T) -> Self {
    Self::queue(
      &Stack::push(&queue.head, item),
      queue.len_head + 1,
      &queue.tail,
      queue.len_tail,
    )
  }

  pub fn dequeue(queue: &Self) -> Option<(T, Self)> {
    match &queue.tail {
      Stack::Empty => None,
      Stack::Node(value, stack_remaining) => Some((
        value.clone(),
        Self::queue(
          &queue.head,
          queue.len_head,
          &stack_remaining,
          queue.len_tail - 1,
        ),
      )),
    }
  }

  pub fn drop(queue: &Self) -> Option<Self> {
    match &queue.tail {
      Stack::Empty => None,
      Stack::Node(_, stack_remaining) => Some(Self::queue(
        &queue.head,
        queue.len_head,
        &stack_remaining,
        queue.len_tail - 1,
      )),
    }
  }

  pub fn head(queue: &Self) -> Option<T> {
    match &queue.tail {
      Stack::Empty => None,
      Stack::Node(value, _) => Some(value.clone()),
    }
  }

  pub fn daeh(queue: &Self) -> Option<T> {
    match (&queue.head, &queue.tail) {
      (_, Stack::Empty) => None,
      (Stack::Node(value, _), _) => Some(value.clone()),
      (_, tail) => Stack::peek(&Stack::rev(tail).clone()).copied(),
    }
  }

  pub fn len(queue: &Self) -> i32 {
    queue.len_head + queue.len_tail
  }

  pub fn rev(queue: &Self) -> Self {
    match queue.tail {
      Stack::Empty => Self::new(),
      Stack::Node(..) => Self::queue(
        &queue.tail.clone(),
        queue.len_tail,
        &queue.head.clone(),
        queue.len_head,
      ),
    }
  }

  pub fn concat(q1: &Self, q2: &Self) -> Self {
    Self::queue(
      &Stack::concat(&q2.head, &Stack::rev(&q2.tail)),
      Self::len(&q2),
      &Stack::concat(&q1.tail, &Stack::rev(&q1.head)),
      Self::len(&q1),
    )
  }

  pub fn split(queue: &Self, f: fn(&T) -> bool) -> (Self, Self) {
    let (h1, lh1, h2, lh2) =
      private::split_stack_aux(&queue.head, f, Stack::Empty, 0, Stack::Empty, 0);
    let (t1, lt1, t2, lt2) =
      private::split_stack_aux(&queue.tail, f, Stack::Empty, 0, Stack::Empty, 0);
    (
      Self::queue(&h1, lh1, &t1, lt1),
      Self::queue(&h2, lh2, &t2, lt2),
    )
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
