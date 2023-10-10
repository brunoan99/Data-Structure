use super::*;

type QueueT = Queue<i32>;

fn queue_empty_on_both() -> QueueT {
  QueueT {
    head: Stack::<i32>::Empty,
    tail: Stack::<i32>::Empty,
  }
}

fn queue_filled_on_head() -> Queue<i32> {
  Queue {
    head: Stack::Node(
      3,
      Box::new(Stack::Node(
        2,
        Box::new(Stack::Node(
          1,
          Box::new(Stack::Node(0, Box::new(Stack::Empty))),
        )),
      )),
    ),
    tail: Stack::Empty,
  }
}

fn queue_filled_on_tail() -> Queue<i32> {
  Queue {
    head: Stack::Empty,
    tail: Stack::Node(
      0,
      Box::new(Stack::Node(
        1,
        Box::new(Stack::Node(
          2,
          Box::new(Stack::Node(3, Box::new(Stack::Empty))),
        )),
      )),
    ),
  }
}

fn queue_filled_on_both() -> Queue<i32> {
  Queue {
    head: Stack::Node(
      7,
      Box::new(Stack::Node(
        6,
        Box::new(Stack::Node(
          5,
          Box::new(Stack::Node(4, Box::new(Stack::Empty))),
        )),
      )),
    ),
    tail: Stack::Node(
      0,
      Box::new(Stack::Node(
        1,
        Box::new(Stack::Node(
          2,
          Box::new(Stack::Node(3, Box::new(Stack::Empty))),
        )),
      )),
    ),
  }
}

#[test]
fn test_is_empty_to_both_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::empty(&queue);
  assert_eq!(op, true)
}

#[test]
fn test_is_empty_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::empty(&queue);
  assert_eq!(op, false)
}

#[test]
fn test_is_empty_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::empty(&queue);
  assert_eq!(op, false)
}

#[test]
fn test_is_empty_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::empty(&queue);
  assert_eq!(op, false)
}

#[test]
fn test_enqueue_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::enqueue(&queue, 0);
  let expected = QueueT {
    head: Stack::Node(0, Box::new(Stack::Empty)),
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn test_enqueue_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::enqueue(&queue, 9);
  let expected = QueueT {
    head: Stack::Node(9, Box::new(queue.head)),
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn test_enqueue_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::enqueue(&queue, 9);
  let expected = QueueT {
    head: Stack::Node(9, Box::new(Stack::Empty)),
    tail: queue.tail,
  };
  assert_eq!(op, expected)
}

#[test]
fn test_enqueue_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::enqueue(&queue, 9);
  let expected = QueueT {
    head: Stack::Node(9, Box::new(queue.head)),
    tail: queue.tail,
  };
  assert_eq!(op, expected)
}

#[test]
fn test_dequeue_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::dequeue(&queue);
  assert_eq!(op, None)
}

#[test]
fn test_dequeue_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::dequeue(&queue);
  let expected_queue = QueueT {
    head: Stack::Empty,
    tail: Stack::Node(
      1,
      Box::new(Stack::Node(
        2,
        Box::new(Stack::Node(3, Box::new(Stack::Empty))),
      )),
    ),
  };
  assert_eq!(op, Some((0, expected_queue)))
}

#[test]
fn test_dequeue_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::dequeue(&queue);
  let expected_queue = QueueT {
    head: Stack::Empty,
    tail: Stack::Node(
      1,
      Box::new(Stack::Node(
        2,
        Box::new(Stack::Node(3, Box::new(Stack::Empty))),
      )),
    ),
  };
  assert_eq!(op, Some((0, expected_queue)))
}

#[test]
fn test_dequeue_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::dequeue(&queue);
  let expected_queue = Queue {
    head: Stack::Node(
      7,
      Box::new(Stack::Node(
        6,
        Box::new(Stack::Node(
          5,
          Box::new(Stack::Node(4, Box::new(Stack::Empty))),
        )),
      )),
    ),
    tail: Stack::Node(
      1,
      Box::new(Stack::Node(
        2,
        Box::new(Stack::Node(3, Box::new(Stack::Empty))),
      )),
    ),
  };
  assert_eq!(op, Some((0, expected_queue)))
}

#[test]
fn test_peek_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::peek(&queue);
  assert_eq!(op, None)
}

#[test]
fn test_peek_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::peek(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn test_peek_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::peek(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn test_peek_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::peek(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn test_len_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::len(&queue);
  assert_eq!(op, 0)
}

#[test]
fn test_len_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::len(&queue);
  assert_eq!(op, 4)
}

#[test]
fn test_len_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::len(&queue);
  assert_eq!(op, 4)
}

#[test]
fn test_len_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::len(&queue);
  assert_eq!(op, 8)
}
