use crate::Stack;

// Invariants:
//    - tail is empty only if head is also empty;
#[derive(Clone, PartialEq, Debug)]
pub struct Queue<T> {
  head: Stack<T>,
  tail: Stack<T>,
}

impl<T> Queue<T>
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
    Queue::queue(&Stack::push(&queue.head, item), &queue.tail)
  }

  pub fn dequeue(queue: &Self) -> Option<(T, Self)> {
    match &queue.tail {
      Stack::Empty => None,
      Stack::Node(value, stack_remaining) => {
        Some((value.clone(), Queue::queue(&queue.head, &*stack_remaining)))
      }
    }
  }

  pub fn drop(queue: &Self) -> Option<Self> {
    match (&queue.head, &queue.tail) {
      (Stack::Empty, Stack::Empty) => None,
      (head, Stack::Empty) => Self::drop(&Queue::queue(&head, &Stack::Empty)),
      (head, tail) => Some(Queue::queue(&head.clone(), &Stack::pop(&tail).unwrap().1)),
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
    Stack::len(&queue.head) + Stack::len(&queue.tail)
  }

  pub fn rev(queue: &Self) -> Self {
    match queue.tail {
      Stack::Empty => Queue::new(),
      Stack::Node(..) => Queue::queue(&queue.tail.clone(), &queue.head.clone()),
    }
  }

  pub fn concat(q1: &Self, q2: &Self) -> Self {
    Queue::queue(
      &Stack::concat(&q2.head, &Stack::rev(&q2.tail)),
      &Stack::concat(&q1.tail, &Stack::rev(&q1.head)),
    )
  }

  pub fn split(queue: &Self, f: fn(&T) -> bool) -> (Self, Self) {
    let (h1, h2) = Stack::split(&queue.head, f);
    let (t1, t2) = Stack::split(&queue.tail, f);
    (Queue::queue(&h1, &t1), Queue::queue(&h2, &t2))
  }

  pub fn any(queue: &Self, f: fn(&T) -> bool) -> bool {
    Stack::any(&queue.tail, f) || Stack::any(&queue.head, f)
  }

  pub fn all(queue: &Self, f: fn(&T) -> bool) -> bool {
    Stack::all(&queue.tail, f) && Stack::all(&queue.head, f)
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
    U: Clone + PartialEq + Copy,
  {
    Queue::<U>::queue(&Stack::map(&queue.head, f), &Stack::map(&queue.tail, f))
  }

  pub fn filter(queue: &Self, f: fn(&T) -> bool) -> Self {
    Queue::queue(
      &Stack::filter(&queue.head, f),
      &Stack::filter(&queue.tail, f),
    )
  }

  pub fn reduce<U>(queue: &Self, f: fn(&T, U) -> U, acc: U) -> U {
    Stack::reduce(&queue.head, f, Stack::reduce(&queue.tail, f, acc))
  }
}

#[cfg(test)]
#[path = "./queue_test.rs"]
mod test;
