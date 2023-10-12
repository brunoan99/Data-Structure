use crate::Stack;

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
      } => Self::dequeue(&Self {
        head: Stack::Empty,
        tail: Stack::rev(&head),
      }),
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

  pub fn drop(queue: &Self) -> Option<Self> {
    match &queue {
      Self {
        head: Stack::Empty,
        tail: Stack::Empty,
      } => None,
      Self {
        head,
        tail: Stack::Empty,
      } => Self::drop(&Self {
        head: Stack::Empty,
        tail: Stack::rev(&head),
      }),
      Self { head, tail } => Some(Self {
        head: head.to_owned(),
        tail: Stack::pop(&tail).unwrap().1,
      }),
    }
  }

  pub fn get(queue: &Self) -> Option<T> {
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

  pub fn rev(queue: &Self) -> Self {
    match &queue {
      Queue {
        head: Stack::Node(..),
        tail: Stack::Node(..),
      } => Queue {
        head: queue.tail.to_owned(),
        tail: queue.head.to_owned(),
      },
      Queue { head, tail } => Queue {
        head: Stack::rev(&head),
        tail: Stack::rev(&tail),
      },
    }
  }

  pub fn concat(q1: &Self, q2: &Self) -> Self {
    Self {
      head: Stack::concat(&Stack::rev(&q2.tail), &q2.head),
      tail: Stack::concat(&Stack::rev(&q1.head), &q1.tail),
    }
  }

  pub fn split(queue: &Self, f: fn(&T) -> bool) -> (Self, Self) {
    let (h1, h2) = Stack::split(&queue.head, f);
    let (t1, t2) = Stack::split(&queue.tail, f);
    (Self { head: h1, tail: t1 }, Self { head: h2, tail: t2 })
  }
}

#[cfg(test)]
#[path = "./queue_test.rs"]
mod test;
