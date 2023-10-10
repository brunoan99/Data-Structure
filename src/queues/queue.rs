pub use crate::Stack;

#[derive(Clone, PartialEq, Debug)]
pub struct Queue<T> {
  head: Stack<T>,
  tail: Stack<T>,
}

impl<T> Queue<T>
where
  T: PartialEq + Clone + Copy,
{
  pub fn empty(queue: &Self) -> bool {
    matches!(
      queue,
      Self {
        head: Stack::Empty,
        tail: Stack::Empty,
      }
    )
  }

  pub fn enqueue(queue: &Self, item: T) -> Self {
    Self {
      head: Stack::push(&queue.head, item),
      tail: queue.tail.to_owned(),
    }
  }

  pub fn dequeue(queue: &Self) -> Option<(T, Self)> {
    match &queue {
      Self {
        head: Stack::Empty,
        tail: Stack::Empty,
      } => None,
      Self {
        head,
        tail: Stack::Empty,
      } => {
        let rev_head = Stack::rev(&head);
        Self::dequeue(&Self {
          head: Stack::Empty,
          tail: rev_head,
        })
      }
      Self { head, tail } => {
        let (poped, new_tail) = Stack::pop(&tail).unwrap();
        Some((
          poped,
          Self {
            head: head.to_owned(),
            tail: new_tail,
          },
        ))
      }
    }
  }

  pub fn peek(queue: &Self) -> Option<T> {
    match &queue {
      Self {
        head: Stack::Empty,
        tail: Stack::Empty,
      } => None,
      Self {
        head: _,
        tail: Stack::Node(value, _),
      } => Some(value.clone()),
      Self { head, tail: _ } => {
        let rev = Stack::rev(head).clone();
        let out = Stack::peek(&rev);
        out.copied()
      }
    }
  }

  pub fn len(queue: &Self) -> i32 {
    Stack::len(&queue.head) + Stack::len(&queue.tail)
  }
}

#[cfg(test)]
#[path = "./queue_test.rs"]
mod test;
