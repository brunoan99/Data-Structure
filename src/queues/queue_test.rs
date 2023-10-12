use super::*;

type QueueT = Queue<i32>;

fn node<T>(head: T, tail: Stack<T>) -> Stack<T> {
  Stack::Node(head, Box::new(tail))
}

fn queue_empty_on_both() -> QueueT {
  QueueT {
    head: Stack::<i32>::Empty,
    tail: Stack::<i32>::Empty,
  }
}

fn queue_filled_on_head() -> Queue<i32> {
  Queue {
    head: node(3, node(2, node(1, node(0, Stack::Empty)))),
    tail: Stack::Empty,
  }
}

fn queue_filled_on_tail() -> Queue<i32> {
  Queue {
    head: Stack::Empty,
    tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
  }
}

fn queue_filled_on_both() -> Queue<i32> {
  Queue {
    head: node(7, node(6, node(5, node(4, Stack::Empty)))),
    tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
  }
}

#[test]
fn is_empty_to_both_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::empty(&queue);
  assert_eq!(op, true)
}

#[test]
fn is_empty_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::empty(&queue);
  assert_eq!(op, false)
}

#[test]
fn is_empty_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::empty(&queue);
  assert_eq!(op, false)
}

#[test]
fn is_empty_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::empty(&queue);
  assert_eq!(op, false)
}

#[test]
fn enqueue_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::enqueue(&queue, 0);
  let expected = QueueT {
    head: node(0, Stack::Empty),
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn enqueue_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::enqueue(&queue, 4);
  let expected = QueueT {
    head: node(4, queue.head),
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn enqueue_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::enqueue(&queue, 4);
  let expected = QueueT {
    head: node(4, Stack::Empty),
    tail: queue.tail,
  };
  assert_eq!(op, expected)
}

#[test]
fn enqueue_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::enqueue(&queue, 8);
  let expected = QueueT {
    head: node(8, queue.head),
    tail: queue.tail,
  };
  assert_eq!(op, expected)
}

#[test]
fn dequeue_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::dequeue(&queue);
  assert_eq!(op, None)
}

#[test]
fn dequeue_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::dequeue(&queue);
  let expected = Some((
    0,
    Queue {
      head: Stack::Empty,
      tail: node(1, node(2, node(3, Stack::Empty))),
    },
  ));
  assert_eq!(op, expected)
}

#[test]
fn dequeue_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::dequeue(&queue);
  let expected = Some((
    0,
    Queue {
      head: Stack::Empty,
      tail: node(1, node(2, node(3, Stack::Empty))),
    },
  ));
  assert_eq!(op, expected)
}

#[test]
fn dequeue_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::dequeue(&queue);
  let expected = Some((
    0,
    Queue {
      head: node(7, node(6, node(5, node(4, Stack::Empty)))),
      tail: node(1, node(2, node(3, Stack::Empty))),
    },
  ));
  assert_eq!(op, expected)
}

#[test]
fn get_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::get(&queue);
  assert_eq!(op, None)
}

#[test]
fn get_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::get(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn get_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::get(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn get_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::get(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn len_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::len(&queue);
  assert_eq!(op, 0)
}

#[test]
fn len_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::len(&queue);
  assert_eq!(op, 4)
}

#[test]
fn len_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::len(&queue);
  assert_eq!(op, 4)
}

#[test]
fn len_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::len(&queue);
  assert_eq!(op, 8)
}

#[test]
fn rev_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::rev(&queue);
  assert_eq!(
    op,
    Queue {
      head: Stack::Empty,
      tail: Stack::Empty,
    }
  )
}

#[test]
fn rev_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::rev(&queue);
  let expected = Queue {
    head: node(0, node(1, node(2, node(3, Stack::Empty)))),
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn rev_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::rev(&queue);
  let expected = Queue {
    head: Stack::Empty,
    tail: node(3, node(2, node(1, node(0, Stack::Empty)))),
  };
  assert_eq!(op, expected)
}

#[test]
fn rev_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::rev(&queue);
  let expected = Queue {
    head: node(0, node(1, node(2, node(3, Stack::Empty)))),
    tail: node(7, node(6, node(5, node(4, Stack::Empty)))),
  };
  assert_eq!(op, expected)
}

#[test]
fn concat_to_both_filled_on_both() {
  let q1 = Queue {
    head: node(4, node(3, Stack::Empty)),
    tail: node(1, node(2, Stack::Empty)),
  };
  let q2 = Queue {
    head: node(8, node(7, Stack::Empty)),
    tail: node(5, node(6, Stack::Empty)),
  };
  let op = Queue::concat(&q1, &q2);
  let expect = Queue {
    head: node(8, node(7, node(6, node(5, Stack::Empty)))),
    tail: node(1, node(2, node(3, node(4, Stack::Empty)))),
  };
  assert_eq!(op, expect)
}

#[test]
fn split_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::split(&queue, |item| item >= &1);
  assert_eq!(
    op,
    (
      Queue {
        head: Stack::Empty,
        tail: Stack::Empty,
      },
      Queue {
        head: Stack::Empty,
        tail: Stack::Empty,
      }
    )
  )
}

#[test]
fn split_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::split(&queue, |item| item > &1);
  assert_eq!(
    op,
    (
      Queue {
        head: node(1, node(0, Stack::Empty)),
        tail: Stack::Empty,
      },
      Queue {
        head: node(3, node(2, Stack::Empty)),
        tail: Stack::Empty,
      }
    )
  )
}

#[test]
fn split_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::split(&queue, |item| item > &1);
  let expected = (
    Queue {
      head: Stack::Empty,
      tail: node(0, node(1, Stack::Empty)),
    },
    Queue {
      head: Stack::Empty,
      tail: node(2, node(3, Stack::Empty)),
    },
  );
  assert_eq!(op, expected)
}

#[test]
fn split_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::split(&queue, |item| item % 2 == 1);
  let expected = (
    Queue {
      head: node(6, node(4, Stack::Empty)),
      tail: node(0, node(2, Stack::Empty)),
    },
    Queue {
      head: node(7, node(5, Stack::Empty)),
      tail: node(1, node(3, Stack::Empty)),
    },
  );
  assert_eq!(op, expected)
}

#[test]
fn split_to_filled_on_both_with_only_false_return() {
  let queue = queue_filled_on_both();
  let op = Queue::split(&queue, |_| false);
  let expected = (
    Queue {
      head: node(7, node(6, node(5, node(4, Stack::Empty)))),
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
    },
    Queue {
      head: Stack::Empty,
      tail: Stack::Empty,
    },
  );
  assert_eq!(op, expected)
}

#[test]
fn split_to_filled_on_both_with_only_true_return() {
  let queue = queue_filled_on_both();
  let op = Queue::split(&queue, |_| true);
  let expected = (
    Queue {
      head: Stack::Empty,
      tail: Stack::Empty,
    },
    Queue {
      head: node(7, node(6, node(5, node(4, Stack::Empty)))),
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
    },
  );
  assert_eq!(op, expected)
}
