use super::*;

mod setup {
  use super::*;

  pub type QueueT = Queue<i32>;

  pub fn node<T>(head: T, tail: Stack<T>) -> Stack<T> {
    Stack::Node(head, Box::new(tail))
  }

  pub fn queue_empty_on_both() -> QueueT {
    QueueT {
      head: Stack::<i32>::Empty,
      tail: Stack::<i32>::Empty,
    }
  }

  pub fn queue_filled_on_tail() -> Queue<i32> {
    Queue {
      head: Stack::Empty,
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
    }
  }

  pub fn queue_filled_on_both() -> Queue<i32> {
    Queue {
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
    let op = Queue::<i32>::new();
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod queue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let op = Queue::queue(&Stack::Empty, &Stack::Empty);
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_head() {
    let op = Queue::queue(
      &setup::node(
        3,
        setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
      ),
      &Stack::Empty,
    );
    let expected = Queue {
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
    let op = Queue::queue(
      &Stack::Empty,
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    );
    let expected = Queue {
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
    let op = Queue::queue(
      &setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    );
    let expected = Queue {
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
    let op = Queue::is_empty(&queue);
    assert_eq!(op, true)
  }

  #[test]
  fn filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::is_empty(&queue);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::is_empty(&queue);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod enqueue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::enqueue(&queue, 0);
    let expected = Queue {
      head: Stack::Empty,
      tail: setup::node(0, Stack::Empty),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::enqueue(&queue, 4);
    let expected = Queue {
      head: setup::node(4, Stack::Empty),
      tail: queue.tail,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::enqueue(&queue, 8);
    let expected = Queue {
      head: setup::node(8, queue.head),
      tail: queue.tail,
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod dequeue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::dequeue(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::dequeue(&queue);
    let expected = Some((
      0,
      Queue {
        head: Stack::Empty,
        tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      },
    ));
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::dequeue(&queue);
    let expected = Some((
      0,
      Queue {
        head: setup::node(
          7,
          setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
        ),
        tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      },
    ));
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod head {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::head(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::head(&queue);
    let expected = Some(0);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::head(&queue);
    let expected = Some(0);
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod daeh {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::daeh(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::daeh(&queue);
    let expected = Some(3);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::daeh(&queue);
    let expected = Some(7);
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod len {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::len(&queue);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::len(&queue);
    assert_eq!(op, 4)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::len(&queue);
    assert_eq!(op, 8)
  }
}

#[cfg(test)]
mod rev {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
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
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::rev(&queue);
    let expected = Queue {
      head: Stack::Empty,
      tail: setup::node(
        3,
        setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::rev(&queue);
    let expected = Queue {
      head: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      tail: setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod concat {
  use super::*;

  #[test]
  fn both_empty_both() {
    let q1 = setup::queue_empty_on_both();
    let q2 = setup::queue_empty_on_both();
    let op = Queue::concat(&q1, &q2);
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }

  #[test]
  fn first_filled_second_empty() {
    let q1 = setup::queue_filled_on_tail();
    let q2 = setup::queue_empty_on_both();
    let op = Queue::concat(&q1, &q2);
    let expected = Queue {
      head: Stack::Empty,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn first_empty_second_filled() {
    let q1 = setup::queue_empty_on_both();
    let q2 = setup::queue_filled_on_tail();
    let op = Queue::concat(&q1, &q2);
    let expected = Queue {
      head: Stack::Empty,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn both_filled() {
    let q1 = Queue {
      head: setup::node(4, setup::node(3, Stack::Empty)),
      tail: setup::node(1, setup::node(2, Stack::Empty)),
    };
    let q2 = Queue {
      head: setup::node(8, setup::node(7, Stack::Empty)),
      tail: setup::node(5, setup::node(6, Stack::Empty)),
    };
    let op = Queue::concat(&q1, &q2);
    let expect = Queue {
      head: setup::node(
        8,
        setup::node(7, setup::node(6, setup::node(5, Stack::Empty))),
      ),
      tail: setup::node(
        1,
        setup::node(2, setup::node(3, setup::node(4, Stack::Empty))),
      ),
    };
    assert_eq!(op, expect)
  }
}

#[cfg(test)]
mod split {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
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
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::split(&queue, |item| item > &1);
    let expected = (
      Queue {
        head: Stack::Empty,
        tail: setup::node(0, setup::node(1, Stack::Empty)),
      },
      Queue {
        head: Stack::Empty,
        tail: setup::node(2, setup::node(3, Stack::Empty)),
      },
    );
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::split(&queue, |item| item % 2 == 1);
    let expected = (
      Queue {
        head: setup::node(6, setup::node(4, Stack::Empty)),
        tail: setup::node(0, setup::node(2, Stack::Empty)),
      },
      Queue {
        head: setup::node(7, setup::node(5, Stack::Empty)),
        tail: setup::node(1, setup::node(3, Stack::Empty)),
      },
    );
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod any {
  use super::*;

  #[test]
  fn to_empty_with_only_true_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::any(&queue, |_| true);
    assert_eq!(op, false)
  }

  #[test]
  fn to_empty_with_only_false_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::any(&queue, |_| false);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_tail_with_only_true_return() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::any(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail_with_only_false_return() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::any(&queue, |_| false);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both_with_only_true_return() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::any(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_both_without_false_return() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::any(&queue, |_| false);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod all {
  use super::*;

  #[test]
  fn to_empty_with_only_true_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::all(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_empty_with_only_false_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::all(&queue, |_| false);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail_with_only_true_return() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::all(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail_with_only_false_return() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::all(&queue, |_| false);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both_with_only_true_return() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::all(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_both_with_only_false_return() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::all(&queue, |_| false);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod find {
  use super::*;

  #[test]
  fn to_empty_with_only_false_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::find(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_empty_with_only_true_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::find(&queue, |_| true);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_head_without_match() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::find(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_head_check_if_gets_first() {
    let queue = Queue {
      head: setup::node(
        (0, 4),
        setup::node(
          (0, 3),
          setup::node((0, 2), setup::node((0, 1), Stack::Empty)),
        ),
      ),
      tail: Stack::Empty,
    };
    let op = Queue::find(&queue, |item| matches!(item, (0, _)));
    // the first on head filled case is the last pushed on head-stack, then it is (0, 1)
    assert_eq!(op, Some(&(0, 1)))
  }

  #[test]
  fn to_filled_on_tail_without_match() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::find(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail_with_match() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::find(&queue, |item| item == &1);
    assert_eq!(op, Some(&1))
  }

  #[test]
  fn to_filled_on_tail_check_if_gets_first() {
    let queue = Queue {
      head: Stack::Empty,
      tail: setup::node(
        (0, 1),
        setup::node(
          (0, 2),
          setup::node((0, 3), setup::node((0, 4), Stack::Empty)),
        ),
      ),
    };
    let op = Queue::find(&queue, |item| matches!(item, (0, _)));
    // the first on tail filled case is the first on tail stack, then it is (0, 1)
    assert_eq!(op, Some(&(0, 1)))
  }

  #[test]
  fn to_filled_on_both_without_match() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::find(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_both_with_match() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::find(&queue, |item| item == &1);
    assert_eq!(op, Some(&1))
  }

  #[test]
  fn to_filled_on_both_check_if_gets_first() {
    let queue = Queue {
      head: setup::node(
        (0, 8),
        setup::node(
          (0, 7),
          setup::node((0, 6), setup::node((0, 5), Stack::Empty)),
        ),
      ),
      tail: setup::node(
        (0, 1),
        setup::node(
          (0, 2),
          setup::node((0, 3), setup::node((0, 4), Stack::Empty)),
        ),
      ),
    };
    let op = Queue::find(&queue, |item| matches!(item, (0, _)));
    // the first on both filled is just the same as the first on tail.
    // the first on tail filled is the first on tail stack, then it is (0, 1)
    assert_eq!(op, Some(&(0, 1)))
  }
}

#[cfg(test)]
mod find_r {
  use super::*;

  #[test]
  fn to_empty_with_only_false_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::find_r(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_empty_with_only_true_return() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::find_r(&queue, |_| true);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_head_without_match() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::find_r(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_head_check_if_gets_last() {
    let queue = Queue {
      head: setup::node(
        (0, 4),
        setup::node(
          (0, 3),
          setup::node((0, 2), setup::node((0, 1), Stack::Empty)),
        ),
      ),
      tail: Stack::Empty,
    };
    let op = Queue::find_r(&queue, |item| matches!(item, (0, _)));
    // the last on head filled case is the head on head-stack, then it is (0, 4)
    assert_eq!(op, Some(&(0, 4)))
  }

  #[test]
  fn to_filled_on_tail_without_match() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::find_r(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail_with_match() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::find_r(&queue, |item| item % 2 == 0);
    assert_eq!(op, Some(&2))
  }

  #[test]
  fn to_filled_on_tail_check_if_gets_last() {
    let queue = Queue {
      head: Stack::Empty,
      tail: setup::node(
        (0, 1),
        setup::node(
          (0, 2),
          setup::node((0, 3), setup::node((0, 4), Stack::Empty)),
        ),
      ),
    };
    let op = Queue::find_r(&queue, |item| matches!(item, (0, _)));
    // the last on tail filled is the last on tail stack, then it is (0, 4)
    assert_eq!(op, Some(&(0, 4)))
  }

  #[test]
  fn to_filled_on_both_without_match() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::find_r(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_both_with_match() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::find_r(&queue, |item| item % 2 == 0);
    assert_eq!(op, Some(&6))
  }

  #[test]
  fn to_filled_on_both_check_if_gets_last() {
    let queue = Queue {
      head: setup::node(
        (0, 8),
        setup::node(
          (0, 7),
          setup::node((0, 6), setup::node((0, 5), Stack::Empty)),
        ),
      ),
      tail: setup::node(
        (0, 1),
        setup::node(
          (0, 2),
          setup::node((0, 3), setup::node((0, 4), Stack::Empty)),
        ),
      ),
    };
    let op = Queue::find_r(&queue, |item| matches!(item, (0, _)));
    // the last on both filled is just the same as the last on tail.
    // the last on tail filled is the last on tail stack, then it is (0, 8)
    assert_eq!(op, Some(&(0, 8)))
  }
}

#[cfg(test)]
mod map {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::map(&queue, |item| item + 3);
    let expected = Queue {
      head: Stack::Empty,
      tail: Stack::Empty,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::map(&queue, |item| item + 3);
    let expected = Queue {
      head: Stack::Empty,
      tail: setup::node(
        3,
        setup::node(4, setup::node(5, setup::node(6, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::map(&queue, |item| item + 3);
    let expected = Queue {
      head: setup::node(
        10,
        setup::node(9, setup::node(8, setup::node(7, Stack::Empty))),
      ),
      tail: setup::node(
        3,
        setup::node(4, setup::node(5, setup::node(6, Stack::Empty))),
      ),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod filter {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::filter(&queue, |_| true);
    let expected = Queue {
      head: Stack::Empty,
      tail: Stack::Empty,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::filter(&queue, |item| item % 2 == 0);
    let expected = Queue {
      head: Stack::Empty,
      tail: setup::node(0, setup::node(2, Stack::Empty)),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::filter(&queue, |item| item % 2 == 0);
    let expected = Queue {
      head: setup::node(6, setup::node(4, Stack::Empty)),
      tail: setup::node(0, setup::node(2, Stack::Empty)),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod reduce {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = Queue::reduce(&queue, |item, acc| acc + item, 0);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = Queue::reduce(&queue, |item, acc| acc + item, 0);
    assert_eq!(op, 6)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = Queue::reduce(
      &queue,
      |item, acc| {
        println!("Acc: {acc}, Item: {item}");
        acc + item
      },
      0,
    );
    assert_eq!(op, 28)
  }
}
