use super::*;

mod setup {
  use super::*;

  pub type DequeT = Deque<i32>;

  pub fn node<T>(head: T, tail: Stack<T>) -> Stack<T> {
    Stack::Node(head, Box::new(tail))
  }

  pub fn queue_empty_on_both() -> DequeT {
    DequeT {
      head: Stack::<i32>::Empty,
      tail: Stack::<i32>::Empty,
    }
  }

  pub fn queue_filled_on_tail() -> Deque<i32> {
    Deque {
      head: Stack::Empty,
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
    }
  }

  pub fn queue_filled_on_both() -> Deque<i32> {
    Deque {
      head: node(7, node(6, node(5, node(4, Stack::Empty)))),
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
    }
  }
}

#[cfg(test)]
mod new {
  use super::*;

  #[test]
  fn single_case() {
    let op = Deque::<i32>::new();
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod queue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let op = Deque::queue(&Stack::Empty, &Stack::Empty);
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_head() {
    let op = Deque::queue(
      &setup::node(
        3,
        setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
      ),
      &Stack::Empty,
    );
    let expected = Deque {
      head: Stack::Empty,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let op = Deque::queue(
      &Stack::Empty,
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    );
    let expected = Deque {
      head: Stack::Empty,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let op = Deque::queue(
      &setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    );
    let expected = Deque {
      head: setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod is_empty {
  use super::*;

  #[test]
  fn to_both_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Deque::is_empty(&queue);
    assert_eq!(op, true)
  }

  #[test]
  fn filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Deque::is_empty(&queue);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Deque::is_empty(&queue);
    assert_eq!(op, false)
  }
}
