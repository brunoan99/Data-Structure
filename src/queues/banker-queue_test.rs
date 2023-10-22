use super::*;

mod setup {
  use super::*;

  pub type BankerQueueT = BankerQueue<i32>;

  pub fn node<T>(head: T, tail: Stack<T>) -> Stack<T> {
    Stack::Node(head, Box::new(tail))
  }

  pub fn queue_empty_on_both() -> BankerQueueT {
    BankerQueueT {
      head: Stack::<i32>::Empty,
      len_head: 0,
      tail: Stack::<i32>::Empty,
      len_tail: 0,
    }
  }

  pub fn queue_filled_on_tail() -> BankerQueueT {
    BankerQueueT {
      head: Stack::<i32>::Empty,
      len_head: 0,
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
      len_tail: 4,
    }
  }

  pub fn queue_filled_on_both() -> BankerQueueT {
    BankerQueueT {
      head: node(7, node(6, node(5, node(4, Stack::Empty)))),
      len_head: 4,
      tail: node(0, node(1, node(2, node(3, Stack::Empty)))),
      len_tail: 4,
    }
  }
}

#[cfg(test)]
mod new {
  use super::*;

  #[test]
  fn single_case() {
    let op = BankerQueue::<i32>::new();
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod queue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let op = BankerQueue::queue(&Stack::Empty, 0, &Stack::Empty, 0);
    let expected = setup::queue_empty_on_both();
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_head() {
    let op = BankerQueue::queue(
      &setup::node(
        3,
        setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
      ),
      4,
      &Stack::Empty,
      0,
    );
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let op = BankerQueue::queue(
      &Stack::Empty,
      0,
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      4,
    );
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let op = BankerQueue::queue(
      &setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      4,
      &setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      4,
    );
    let expected = BankerQueue {
      head: setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      len_head: 4,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod is_empty {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::is_empty(&queue);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::is_empty(&queue);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::is_empty(&queue);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod enqueue {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::enqueue(&queue, 0);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(0, Stack::Empty),
      len_tail: 1,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::enqueue(&queue, 4);
    let expected = BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: queue.tail,
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_unballanced() {
    let queue = BankerQueue {
      head: setup::node(5, setup::node(4, Stack::Empty)),
      len_head: 2,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    let op = BankerQueue::enqueue(&queue, 6);
    let expected = BankerQueue {
      head: setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      len_head: 3,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_balanced() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::enqueue(&queue, 8);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        0,
        setup::node(
          1,
          setup::node(
            2,
            setup::node(
              3,
              setup::node(
                4,
                setup::node(
                  5,
                  setup::node(6, setup::node(7, setup::node(8, Stack::Empty))),
                ),
              ),
            ),
          ),
        ),
      ),
      len_tail: 9,
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
    let op = BankerQueue::dequeue(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::dequeue(&queue);
    let expected = Some((
      0,
      BankerQueue {
        head: Stack::Empty,
        len_head: 0,
        tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
        len_tail: 3,
      },
    ));
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_unballanced() {
    let queue = BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    let op = BankerQueue::dequeue(&queue);
    let expected = Some((
      0,
      BankerQueue {
        head: setup::node(4, Stack::Empty),
        len_head: 1,
        tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
        len_tail: 3,
      },
    ));
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_balanced() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::dequeue(&queue);
    let expected = Some((
      0,
      BankerQueue {
        head: Stack::Empty,
        len_head: 0,
        tail: setup::node(
          1,
          setup::node(
            2,
            setup::node(
              3,
              setup::node(
                4,
                setup::node(5, setup::node(6, setup::node(7, Stack::Empty))),
              ),
            ),
          ),
        ),
        len_tail: 7,
      },
    ));
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod drop {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::drop(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::drop(&queue);
    let expected = Some(BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      len_tail: 3,
    });
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_unballanced() {
    let queue = BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    let op = BankerQueue::drop(&queue);
    let expected = Some(BankerQueue {
      head: setup::node(4, Stack::Empty),
      len_head: 1,
      tail: setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      len_tail: 3,
    });
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both_balanced() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::drop(&queue);
    let expected = Some(BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        1,
        setup::node(
          2,
          setup::node(
            3,
            setup::node(
              4,
              setup::node(5, setup::node(6, setup::node(7, Stack::Empty))),
            ),
          ),
        ),
      ),
      len_tail: 7,
    });
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod head {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::head(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::head(&queue);
    assert_eq!(op, Some(0))
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::head(&queue);
    assert_eq!(op, Some(0))
  }
}

#[cfg(test)]
mod daeh {

  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::daeh(&queue);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::daeh(&queue);
    assert_eq!(op, Some(3))
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::daeh(&queue);
    assert_eq!(op, Some(7))
  }
}

#[cfg(test)]
mod len {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::len(&queue);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::len(&queue);
    assert_eq!(op, 4)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::len(&queue);
    assert_eq!(op, 8)
  }
}

#[cfg(test)]
mod rev {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::rev(&queue);
    let expected = queue;
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::rev(&queue);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        3,
        setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::rev(&queue);
    let expected = BankerQueue {
      head: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_head: 4,
      tail: setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod concat {
  use super::*;

  #[test]
  fn to_both_empty() {
    let q1 = setup::queue_empty_on_both();
    let q2 = setup::queue_empty_on_both();
    let op = BankerQueue::concat(&q1, &q2);
    let expected = q1;
    assert_eq!(op, expected);
  }

  #[test]
  fn to_first_filled_on_tail_second_empty() {
    let q1 = setup::queue_filled_on_tail();
    let q2 = setup::queue_empty_on_both();
    let op = BankerQueue::concat(&q1, &q2);
    let expected = q1;
    assert_eq!(op, expected);
  }

  #[test]
  fn to_first_empty_second_filled_on_tail() {
    let q1 = setup::queue_empty_on_both();
    let q2 = setup::queue_filled_on_tail();
    let op = BankerQueue::concat(&q1, &q2);
    let expected = q2;
    assert_eq!(op, expected);
  }

  #[test]
  fn to_filled_on_both() {
    let q1 = BankerQueue {
      head: setup::node(3, setup::node(2, Stack::Empty)),
      len_head: 2,
      tail: setup::node(0, setup::node(1, Stack::Empty)),
      len_tail: 2,
    };
    let q2 = BankerQueue {
      head: setup::node(7, setup::node(6, Stack::Empty)),
      len_head: 2,
      tail: setup::node(4, setup::node(5, Stack::Empty)),
      len_tail: 2,
    };
    let op = BankerQueue::concat(&q1, &q2);
    let expected = BankerQueue {
      head: setup::node(
        7,
        setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
      ),
      len_head: 4,
      tail: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod split {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::split(&queue, |item| item > &1);
    let expected = (queue.clone(), queue.clone());
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::split(&queue, |item| item > &1);
    let expected = (
      BankerQueue {
        head: Stack::Empty,
        len_head: 0,
        tail: setup::node(0, setup::node(1, Stack::Empty)),
        len_tail: 2,
      },
      BankerQueue {
        head: Stack::Empty,
        len_head: 0,
        tail: setup::node(2, setup::node(3, Stack::Empty)),
        len_tail: 2,
      },
    );
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_only_falsy() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::split(&queue, |_| false);
    let expected = (queue.clone(), setup::queue_empty_on_both());
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_only_truthy() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::split(&queue, |_| true);
    let expected = (setup::queue_empty_on_both(), queue.clone());
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::split(&queue, |item| item % 2 == 0);
    let expected = (
      BankerQueue {
        head: setup::node(7, setup::node(5, Stack::Empty)),
        len_head: 2,
        tail: setup::node(1, setup::node(3, Stack::Empty)),
        len_tail: 2,
      },
      BankerQueue {
        head: setup::node(6, setup::node(4, Stack::Empty)),
        len_head: 2,
        tail: setup::node(0, setup::node(2, Stack::Empty)),
        len_tail: 2,
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
    let op = BankerQueue::any(&queue, |_| true);
    assert_eq!(op, false)
  }

  #[test]
  fn to_empty_with_only_false_return() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::any(&queue, |_| false);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_tail_with_only_true_return() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::any(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail_with_only_false_return() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::any(&queue, |_| false);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both_with_only_true_return() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::any(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_both_without_false_return() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::any(&queue, |_| false);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod all {
  use super::*;

  #[test]
  fn to_empty_with_only_true_return() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::all(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_empty_with_only_false_return() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::all(&queue, |_| false);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail_with_only_true_return() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::all(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_tail_with_only_false_return() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::all(&queue, |_| false);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_on_both_with_only_true_return() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::all(&queue, |_| true);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_on_both_with_only_false_return() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::all(&queue, |_| false);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod find {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::find(&queue, |_| true);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail_without_match() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::find(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail_with_match() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::find(&queue, |item| item % 2 == 0);
    assert_eq!(op, Some(&0))
  }

  #[test]
  fn to_filled_on_both_without_match() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::find(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_both_with_match() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::find(&queue, |item| item % 2 == 1);
    assert_eq!(op, Some(&1))
  }
}

#[cfg(test)]
mod find_r {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::find_r(&queue, |_| true);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail_without_match() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::find_r(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_tail_with_match() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::find_r(&queue, |item| item % 2 == 0);
    assert_eq!(op, Some(&2))
  }

  #[test]
  fn to_filled_on_both_without_match() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::find_r(&queue, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_on_both_with_match() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::find_r(&queue, |item| item % 2 == 1);
    assert_eq!(op, Some(&7))
  }
}

#[cfg(test)]
mod map {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::map(&queue, |item| item * 2);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: Stack::Empty,
      len_tail: 0,
    };
    assert_eq!(op, expected);
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::map(&queue, |item| item * 2);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(
        0,
        setup::node(2, setup::node(4, setup::node(6, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected);
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::map(&queue, |item| item * 2);
    let expected = BankerQueue {
      head: setup::node(
        14,
        setup::node(12, setup::node(10, setup::node(8, Stack::Empty))),
      ),
      len_head: 4,
      tail: setup::node(
        0,
        setup::node(2, setup::node(4, setup::node(6, Stack::Empty))),
      ),
      len_tail: 4,
    };
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod filter {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::filter(&queue, |item| item % 2 == 0);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: Stack::Empty,
      len_tail: 0,
    };
    assert_eq!(op, expected);
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::filter(&queue, |item| item % 2 == 0);
    let expected = BankerQueue {
      head: Stack::Empty,
      len_head: 0,
      tail: setup::node(0, setup::node(2, Stack::Empty)),
      len_tail: 2,
    };
    assert_eq!(op, expected);
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::filter(&queue, |item| item % 2 == 1);
    let expected = BankerQueue {
      head: setup::node(7, setup::node(5, Stack::Empty)),
      len_head: 2,
      tail: setup::node(1, setup::node(3, Stack::Empty)),
      len_tail: 2,
    };
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod reduce {
  use super::*;

  #[test]
  fn to_empty_on_both() {
    let queue = setup::queue_empty_on_both();
    let op = BankerQueue::reduce(&queue, |item, acc| acc + item, 0);
    assert_eq!(op, 0);
  }

  #[test]
  fn to_filled_on_tail() {
    let queue = setup::queue_filled_on_tail();
    let op = BankerQueue::reduce(&queue, |item, acc| acc + item, 0);
    assert_eq!(op, 6);
  }

  #[test]
  fn to_filled_on_both() {
    let queue = setup::queue_filled_on_both();
    let op = BankerQueue::reduce(&queue, |item, acc| acc + item, 10);
    assert_eq!(op, 38);
  }
}
