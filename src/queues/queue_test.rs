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
fn head_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::head(&queue);
  assert_eq!(op, None)
}

#[test]
fn head_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::head(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn head_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::head(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn head_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::head(&queue);
  let expected = Some(0);
  assert_eq!(op, expected)
}

#[test]
fn daeh_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::daeh(&queue);
  assert_eq!(op, None)
}

#[test]
fn daeh_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::daeh(&queue);
  let expected = Some(3);
  assert_eq!(op, expected)
}

#[test]
fn daeh_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::daeh(&queue);
  let expected = Some(3);
  assert_eq!(op, expected)
}

#[test]
fn daeh_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::daeh(&queue);
  let expected = Some(7);
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

#[test]
fn any_to_empty_with_only_true_return() {
  let queue = queue_empty_on_both();
  let op = Queue::any(&queue, |_| true);
  assert_eq!(op, false)
}

#[test]
fn any_to_empty_with_only_false_return() {
  let queue = queue_empty_on_both();
  let op = Queue::any(&queue, |_| false);
  assert_eq!(op, false)
}

#[test]
fn any_to_filled_on_head_with_only_true_return() {
  let queue = queue_filled_on_head();
  let op = Queue::any(&queue, |_| true);
  assert_eq!(op, true)
}

#[test]
fn any_to_filled_on_head_with_only_false_return() {
  let queue = queue_filled_on_head();
  let op = Queue::any(&queue, |_| false);
  assert_eq!(op, false)
}

#[test]
fn any_to_filled_on_tail_with_only_true_return() {
  let queue = queue_filled_on_tail();
  let op = Queue::any(&queue, |_| true);
  assert_eq!(op, true)
}

#[test]
fn any_to_filled_on_tail_with_only_false_return() {
  let queue = queue_filled_on_tail();
  let op = Queue::any(&queue, |_| false);
  assert_eq!(op, false)
}

#[test]
fn any_to_filled_on_both_with_only_true_return() {
  let queue = queue_filled_on_both();
  let op = Queue::any(&queue, |_| true);
  assert_eq!(op, true)
}

#[test]
fn any_to_filled_on_both_without_false_return() {
  let queue = queue_filled_on_both();
  let op = Queue::any(&queue, |_| false);
  assert_eq!(op, false)
}

#[test]
fn all_to_empty_with_only_true_return() {
  let queue = queue_empty_on_both();
  let op = Queue::all(&queue, |_| true);
  assert_eq!(op, true)
}

#[test]
fn all_to_empty_with_only_false_return() {
  let queue = queue_empty_on_both();
  let op = Queue::all(&queue, |_| false);
  assert_eq!(op, true)
}

#[test]
fn all_to_filled_on_head_with_only_true_return() {
  let queue = queue_filled_on_head();
  let op = Queue::all(&queue, |_| true);
  assert_eq!(op, true)
}

#[test]
fn all_to_filled_on_head_with_only_false_return() {
  let queue = queue_filled_on_head();
  let op = Queue::all(&queue, |_| false);
  assert_eq!(op, false)
}

#[test]
fn all_to_filled_on_tail_with_only_true_return() {
  let queue = queue_filled_on_tail();
  let op = Queue::all(&queue, |_| true);
  assert_eq!(op, true)
}

#[test]
fn all_to_filled_on_tail_with_only_false_return() {
  let queue = queue_filled_on_tail();
  let op = Queue::all(&queue, |_| false);
  assert_eq!(op, false)
}

#[test]
fn all_to_filled_on_both_with_only_true_return() {
  let queue = queue_filled_on_both();
  let op = Queue::all(&queue, |_| true);
  assert_eq!(op, true)
}

#[test]
fn all_to_filled_on_both_with_only_false_return() {
  let queue = queue_filled_on_both();
  let op = Queue::all(&queue, |_| false);
  assert_eq!(op, false)
}

#[test]
fn find_to_empty_with_only_false_return() {
  let queue = queue_empty_on_both();
  let op = Queue::find(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_to_empty_with_only_true_return() {
  let queue = queue_empty_on_both();
  let op = Queue::find(&queue, |_| true);
  assert_eq!(op, None)
}

#[test]
fn find_to_filled_on_head_without_match() {
  let queue = queue_filled_on_tail();
  let op = Queue::find(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_to_filled_on_head_with_match() {
  let queue = queue_filled_on_head();
  let op = Queue::find(&queue, |item| item == &1);
  assert_eq!(op, Some(&1))
}

#[test]
fn find_first_to_filled_on_head() {
  let queue = Queue {
    head: node(
      (0, 4),
      node((0, 3), node((0, 2), node((0, 1), Stack::Empty))),
    ),
    tail: Stack::Empty,
  };
  let op = Queue::find(&queue, |item| matches!(item, (0, _)));
  // the first on head filled case is the last pushed on head-stack, then it is (0, 1)
  assert_eq!(op, Some(&(0, 1)))
}

#[test]
fn find_to_filled_on_tail_without_match() {
  let queue = queue_filled_on_tail();
  let op = Queue::find(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_to_filled_on_tail_with_match() {
  let queue = queue_filled_on_tail();
  let op = Queue::find(&queue, |item| item == &1);
  assert_eq!(op, Some(&1))
}

#[test]
fn find_first_to_filled_on_tail() {
  let queue = Queue {
    head: Stack::Empty,
    tail: node(
      (0, 1),
      node((0, 2), node((0, 3), node((0, 4), Stack::Empty))),
    ),
  };
  let op = Queue::find(&queue, |item| matches!(item, (0, _)));
  // the first on tail filled case is the first on tail stack, then it is (0, 1)
  assert_eq!(op, Some(&(0, 1)))
}

#[test]
fn find_to_filled_on_both_without_match() {
  let queue = queue_filled_on_both();
  let op = Queue::find(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_to_filled_on_both_with_match() {
  let queue = queue_filled_on_both();
  let op = Queue::find(&queue, |item| item == &1);
  assert_eq!(op, Some(&1))
}

#[test]
fn find_first_to_filled_on_both() {
  let queue = Queue {
    head: node(
      (0, 8),
      node((0, 7), node((0, 6), node((0, 5), Stack::Empty))),
    ),
    tail: node(
      (0, 1),
      node((0, 2), node((0, 3), node((0, 4), Stack::Empty))),
    ),
  };
  let op = Queue::find(&queue, |item| matches!(item, (0, _)));
  // the first on both filled is just the same as the first on tail.
  // the first on tail filled is the first on tail stack, then it is (0, 1)
  assert_eq!(op, Some(&(0, 1)))
}

#[test]
fn find_r_to_empty_with_only_false_return() {
  let queue = queue_empty_on_both();
  let op = Queue::find_r(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_r_to_empty_with_only_true_return() {
  let queue = queue_empty_on_both();
  let op = Queue::find_r(&queue, |_| true);
  assert_eq!(op, None)
}

#[test]
fn find_r_to_filled_on_head_without_match() {
  let queue = queue_filled_on_tail();
  let op = Queue::find_r(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_r_to_filled_on_head_with_match() {
  let queue = queue_filled_on_head();
  let op = Queue::find_r(&queue, |item| item % 2 == 0);
  assert_eq!(op, Some(&2))
}

#[test]
fn find_r_first_to_filled_on_head() {
  let queue = Queue {
    head: node(
      (0, 4),
      node((0, 3), node((0, 2), node((0, 1), Stack::Empty))),
    ),
    tail: Stack::Empty,
  };
  let op = Queue::find_r(&queue, |item| matches!(item, (0, _)));
  // the last on head filled case is the head on head-stack, then it is (0, 4)
  assert_eq!(op, Some(&(0, 4)))
}

#[test]
fn find_r_to_filled_on_tail_without_match() {
  let queue = queue_filled_on_tail();
  let op = Queue::find_r(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_r_to_filled_on_tail_with_match() {
  let queue = queue_filled_on_tail();
  let op = Queue::find_r(&queue, |item| item % 2 == 0);
  assert_eq!(op, Some(&2))
}

#[test]
fn find_r_to_filled_on_tail() {
  let queue = Queue {
    head: Stack::Empty,
    tail: node(
      (0, 1),
      node((0, 2), node((0, 3), node((0, 4), Stack::Empty))),
    ),
  };
  let op = Queue::find_r(&queue, |item| matches!(item, (0, _)));
  // the last on tail filled is the last on tail stack, then it is (0, 4)
  assert_eq!(op, Some(&(0, 4)))
}

#[test]
fn find_r_to_filled_on_both_without_match() {
  let queue = queue_filled_on_both();
  let op = Queue::find_r(&queue, |_| false);
  assert_eq!(op, None)
}

#[test]
fn find_r_to_filled_on_both_with_match() {
  let queue = queue_filled_on_both();
  let op = Queue::find_r(&queue, |item| item % 2 == 0);
  assert_eq!(op, Some(&6))
}

#[test]
fn find_r_to_filled_on_both() {
  let queue = Queue {
    head: node(
      (0, 8),
      node((0, 7), node((0, 6), node((0, 5), Stack::Empty))),
    ),
    tail: node(
      (0, 1),
      node((0, 2), node((0, 3), node((0, 4), Stack::Empty))),
    ),
  };
  let op = Queue::find_r(&queue, |item| matches!(item, (0, _)));
  // the last on both filled is just the same as the last on tail.
  // the last on tail filled is the last on tail stack, then it is (0, 8)
  assert_eq!(op, Some(&(0, 8)))
}

#[test]
fn map_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::map(&queue, |item| item.to_string());
  let expected = Queue {
    head: Stack::Empty,
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn map_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::map(&queue, |item| item.to_string());
  let expected: Queue<String> = Queue {
    head: node(
      String::from("3"),
      node(
        String::from("2"),
        node(String::from("1"), node(String::from("0"), Stack::Empty)),
      ),
    ),
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn map_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::map(&queue, |item| item.to_string());
  let expected: Queue<String> = Queue {
    head: Stack::Empty,
    tail: node(
      String::from("0"),
      node(
        String::from("1"),
        node(String::from("2"), node(String::from("3"), Stack::Empty)),
      ),
    ),
  };
  assert_eq!(op, expected)
}

#[test]
fn map_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::map(&queue, |item| item.to_string());
  let expected: Queue<String> = Queue {
    head: node(
      String::from("7"),
      node(
        String::from("6"),
        node(String::from("5"), node(String::from("4"), Stack::Empty)),
      ),
    ),
    tail: node(
      String::from("0"),
      node(
        String::from("1"),
        node(String::from("2"), node(String::from("3"), Stack::Empty)),
      ),
    ),
  };
  assert_eq!(op, expected)
}

#[test]
fn filter_to_empty_on_both() {
  let queue = queue_empty_on_both();
  let op = Queue::filter(&queue, |_| true);
  let expected = Queue {
    head: Stack::Empty,
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn filter_to_filled_on_head() {
  let queue = queue_filled_on_head();
  let op = Queue::filter(&queue, |item| item % 2 == 0);
  let expected = Queue {
    head: node(2, node(0, Stack::Empty)),
    tail: Stack::Empty,
  };
  assert_eq!(op, expected)
}

#[test]
fn filter_to_filled_on_tail() {
  let queue = queue_filled_on_tail();
  let op = Queue::filter(&queue, |item| item % 2 == 0);
  let expected = Queue {
    head: Stack::Empty,
    tail: node(0, node(2, Stack::Empty)),
  };
  assert_eq!(op, expected)
}

#[test]
fn filter_to_filled_on_both() {
  let queue = queue_filled_on_both();
  let op = Queue::filter(&queue, |item| item % 2 == 0);
  let expected = Queue {
    head: node(6, node(4, Stack::Empty)),
    tail: node(0, node(2, Stack::Empty)),
  };
  assert_eq!(op, expected)
}
