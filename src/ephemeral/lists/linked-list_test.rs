use super::*;

mod setup {
  use super::*;

  pub type LinkedListT = LinkedList<i32>;

  pub fn node<T>(value: T, next: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
    Some(Box::new(ListNode { value, next: next }))
  }

  pub fn linked_list_empty() -> LinkedListT {
    LinkedListT { root: None }
  }

  pub fn linked_list_filled() -> LinkedListT {
    LinkedListT {
      root: node(0, node(1, node(2, node(3, None)))),
    }
  }
}

#[cfg(test)]
mod new {
  use super::*;

  #[test]
  fn single_case() {
    let op = LinkedList::<i32>::new();
    let expected = setup::linked_list_empty();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod is_empty {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let op = LinkedList::is_empty(&list);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let op = LinkedList::is_empty(&list);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod insert_at_beginning {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    list.insert_at_beginning(0);
    let expected = LinkedList {
      root: setup::node(0, None),
    };
    assert_eq!(list, expected)
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    let expected = LinkedList {
      root: setup::node(
        -1,
        setup::node(0, setup::node(1, setup::node(2, setup::node(3, None)))),
      ),
    };
    list.insert_at_beginning(-1);
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod insert_at_end {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    list.insert_at_end(0);
    let expected = LinkedList {
      root: setup::node(0, None),
    };
    assert_eq!(list, expected)
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    let expected = LinkedList {
      root: setup::node(
        0,
        setup::node(1, setup::node(2, setup::node(3, setup::node(-1, None)))),
      ),
    };
    list.insert_at_end(-1);
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod insert_before {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    let op = list.insert_before(0, 4);
    let expected = Err(InsertError::BeforeItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_without_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.insert_before(6, 5);
    let expected = Err(InsertError::BeforeItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.insert_before(4, 2);
    assert_eq!(op, Ok(()));
    let expected = LinkedList {
      root: setup::node(
        0,
        setup::node(1, setup::node(4, setup::node(2, setup::node(3, None)))),
      ),
    };
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod insert_after {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    let op = list.insert_after(0, 4);
    let expected = Err(InsertError::AfterItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_without_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.insert_after(6, 5);
    let expected = Err(InsertError::AfterItemNotFound);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.insert_after(4, 1);
    assert_eq!(op, Ok(()));
    let expected = LinkedList {
      root: setup::node(
        0,
        setup::node(1, setup::node(4, setup::node(2, setup::node(3, None)))),
      ),
    };
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod remove_at_beginning {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    let op = list.remove_at_beginning();
    assert_eq!(op, Err(RemoveError::EmptyList))
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    let op = list.remove_at_beginning();
    assert_eq!(op, Ok(()));
    let expected = LinkedList {
      root: setup::node(1, setup::node(2, setup::node(3, None))),
    };
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod remove_at_end {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    let op = list.remove_at_end();
    assert_eq!(op, Err(RemoveError::EmptyList))
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    let op = list.remove_at_end();
    assert_eq!(op, Ok(()));
    let expected = LinkedList {
      root: setup::node(0, setup::node(1, setup::node(2, None))),
    };
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod remove_item {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    let op = list.remove_item(1);
    assert_eq!(op, Err(RemoveError::EmptyList))
  }

  #[test]
  fn to_filled_without_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.remove_item(4);
    assert_eq!(op, Err(RemoveError::ItemNotFound))
  }

  #[test]
  fn to_filled_with_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.remove_item(2);
    assert_eq!(op, Ok(()));
    let expected = LinkedList {
      root: setup::node(0, setup::node(1, setup::node(3, None))),
    };
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod len {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let len = list.len();
    assert_eq!(len, 0)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let len = list.len();
    assert_eq!(len, 4)
  }
}

#[cfg(test)]
mod rev {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    list.rev();
    let expected = LinkedList { root: None };
    assert_eq!(list, expected)
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    list.rev();
    let expected = LinkedList {
      root: setup::node(3, setup::node(2, setup::node(1, setup::node(0, None)))),
    };
    assert_eq!(list, expected)
  }
}

#[cfg(test)]
mod concat {
  use super::*;

  #[test]
  fn to_both_empty() {
    let mut l1 = setup::linked_list_empty();
    let l2 = setup::linked_list_empty();
    l1.concat(l2);
    let expect = LinkedList { root: None };
    assert_eq!(l1, expect)
  }

  #[test]
  fn to_first_filled_second_empty() {
    let mut l1 = setup::linked_list_filled();
    let l2 = setup::linked_list_empty();
    l1.concat(l2);
    let expect = LinkedList {
      root: setup::node(0, setup::node(1, setup::node(2, setup::node(3, None)))),
    };
    assert_eq!(l1, expect)
  }

  #[test]
  fn to_first_empty_second_filled() {
    let mut l1 = setup::linked_list_empty();
    let l2 = setup::linked_list_filled();
    l1.concat(l2);
    let expect = LinkedList {
      root: setup::node(0, setup::node(1, setup::node(2, setup::node(3, None)))),
    };
    assert_eq!(l1, expect)
  }

  #[test]
  fn to_both_filled() {
    let mut l1 = LinkedList {
      root: setup::node(0, setup::node(1, setup::node(2, setup::node(3, None)))),
    };
    let l2 = LinkedList {
      root: setup::node(4, setup::node(5, setup::node(6, setup::node(7, None)))),
    };
    l1.concat(l2);
    let expect = LinkedList {
      root: setup::node(
        0,
        setup::node(
          1,
          setup::node(
            2,
            setup::node(
              3,
              setup::node(4, setup::node(5, setup::node(6, setup::node(7, None)))),
            ),
          ),
        ),
      ),
    };
    assert_eq!(l1, expect)
  }
}
