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

  pub fn head(queue: &Self) -> Option<T> {
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

  pub fn daeh(queue: &Self) -> Option<T> {
    match &queue {
      Self {
        head: Stack::Empty,
        tail: Stack::Empty,
      } => None,
      Self {
        head: Stack::Node(value, _),
        tail: _,
      } => Some(value.clone()),
      Self { head: _, tail } => {
        let rev = Stack::rev(tail).clone();
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

  pub fn any(queue: &Self, f: fn(&T) -> bool) -> bool {
    Stack::any(&queue.head, f) || Stack::any(&queue.tail, f)
  }

  pub fn all(queue: &Self, f: fn(&T) -> bool) -> bool {
    Stack::all(&queue.head, f) && Stack::all(&queue.tail, f)
  }

  pub fn find(queue: &Self, f: fn(&T) -> bool) -> Option<&T> {
    match Stack::find(&queue.tail, f) {
      Some(value) => Some(value),
      None => Stack::find_r(&queue.head, f),
    }
  }

  pub fn find_r(queue: &Self, f: fn(&T) -> bool) -> Option<&T> {
    match Stack::find(&queue.head, f) {
      Some(value) => Some(value),
      None => Stack::find_r(&queue.tail, f),
    }
  }

  pub fn map<U>(queue: &Self, f: fn(&T) -> U) -> Queue<U>
  where
    U: Clone + PartialEq,
  {
    Queue {
      head: Stack::map(&queue.head, f),
      tail: Stack::map(&queue.tail, f),
    }
  }

  pub fn filter(queue: &Self, f: fn(&T) -> bool) -> Self {
    Queue {
      head: Stack::filter(&queue.head, f),
      tail: Stack::filter(&queue.tail, f),
    }
  }
}

#[cfg(test)]
#[path = "./queue_test.rs"]
mod test;
