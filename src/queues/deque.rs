use crate::Stack;

// Invariants:
//    - tail is empty only if head is also empty;
#[derive(Clone, PartialEq, Debug)]
pub struct Deque<T> {
  head: Stack<T>,
  tail: Stack<T>,
}

mod private {
  use super::Stack;

  pub fn drop_r_stack_aux<T>(stack: &Stack<T>, acc: Stack<T>) -> Option<(T, Stack<T>)>
  where
    T: PartialEq + Clone + Copy,
  {
    match stack {
      Stack::Empty => None,
      Stack::Node(value, stack_remaining) => match *stack_remaining.clone() {
        Stack::Empty => Some((value.clone(), Stack::rev(&acc))),
        Stack::Node(..) => drop_r_stack_aux(&stack_remaining, Stack::push(&acc, value.clone())),
      },
    }
  }
}

impl<T> Deque<T>
where
  T: PartialEq + Clone + Copy,
{
  pub fn new() -> Self {
    Self {
      head: Stack::Empty,
      tail: Stack::Empty,
    }
  }

  pub fn queue(head: &Stack<T>, tail: &Stack<T>) -> Self {
    match (head, tail) {
      (_, Stack::Empty) => Self {
        head: Stack::Empty,
        tail: Stack::rev(&head),
      },
      _ => Self {
        head: head.clone(),
        tail: tail.clone(),
      },
    }
  }

  pub fn is_empty(queue: &Self) -> bool {
    Stack::is_empty(&queue.tail)
  }

  pub fn enqueue(queue: &Self, item: T) -> Self {
    Self::queue(&Stack::push(&queue.head, item), &queue.tail)
  }

  pub fn enqueue_r(queue: &Self, item: T) -> Self {
    Self::queue(&queue.head, &Stack::push(&queue.tail, item))
  }

  pub fn dequeue(queue: &Self) -> Option<(T, Self)> {
    match &queue.tail {
      Stack::Empty => None,
      Stack::Node(value, stack_remaining) => {
        Some((value.clone(), Self::queue(&queue.head, &*stack_remaining)))
      }
    }
  }

  pub fn dequeue_r(queue: &Self) -> Option<(T, Self)> {
    match &queue.head {
      Stack::Node(value, stack_remaining) => {
        Some((value.clone(), Self::queue(stack_remaining, &queue.tail)))
      }
      Stack::Empty => match &queue.tail {
        Stack::Empty => None,
        Stack::Node(..) => match private::drop_r_stack_aux(&queue.tail, Stack::Empty) {
          None => None,
          Some((value, stack_remaining)) => {
            Some((value.clone(), Self::queue(&queue.head, &stack_remaining)))
          }
        },
      },
    }
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

  pub fn map<U>(queue: &Self, f: fn(&T) -> U) -> Deque<U>
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
#[path = "./deque_test.rs"]
mod test;
