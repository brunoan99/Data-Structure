use super::*;

mod setup {
  use super::*;

  pub type LinkedListT = LinkedList<i32>;

  pub fn node<T>(value: T, next: Link<T>) -> Link<T> {
    unsafe {
      Some(NonNull::new_unchecked(Box::into_raw(Box::new(ListNode {
        value,
        next,
      }))))
    }
  }

  pub fn linked_list_empty() -> LinkedListT {
    LinkedListT {
      root: None,
      marker: PhantomData,
    }
  }

  pub fn linked_list_filled() -> LinkedListT {
    LinkedListT {
      root: node(0, node(1, node(2, node(3, None)))),
      marker: PhantomData,
    }
  }

  pub fn test_list<T: PartialEq + std::fmt::Debug>(list: LinkedList<T>, expected: Vec<T>) {
    let mut index = 0;
    let mut tmp_node = list.root;
    while let Some(node) = tmp_node {
      unsafe {
        dbg!(&(*node.as_ptr()).value);
        dbg!(&expected[index]);
        assert_eq!((*node.as_ptr()).value, expected[index]);
        index += 1;
        tmp_node = (*node.as_ptr()).next;
      }
    }
    assert_eq!(
      expected.len(),
      index,
      "List size isn't same as expected, list size: {}, expected size: {}",
      expected.len(),
      index
    );
    assert_eq!(
      tmp_node, None,
      "Last node isn't None as expected, last node: {:#?}",
      tmp_node,
    )
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
    let expected = vec![0];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    list.insert_at_beginning(-1);
    let expected = vec![-1, 0, 1, 2, 3];
    setup::test_list(list, expected)
  }
}

#[cfg(test)]
mod insert_at_end {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    list.insert_at_end(0);
    let expected = vec![0];
    setup::test_list(list, expected);
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    list.insert_at_end(4);
    let expected = vec![0, 1, 2, 3, 4];
    setup::test_list(list, expected)
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
    let expected = vec![0, 1, 4, 2, 3];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled_with_first_as_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.insert_before(-1, 0);
    assert_eq!(op, Ok(()));
    let expected = vec![-1, 0, 1, 2, 3];
    setup::test_list(list, expected)
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
    let expected = vec![0, 1, 4, 2, 3];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled_with_last_as_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.insert_after(4, 3);
    assert_eq!(op, Ok(()));
    let expected = vec![0, 1, 2, 3, 4];
    setup::test_list(list, expected)
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
    let expected = vec![1, 2, 3];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled_just_one_node() {
    let mut list = LinkedList {
      root: setup::node(1, None),
      marker: PhantomData,
    };
    let op = list.remove_at_beginning();
    assert_eq!(op, Ok(()));
    let expected = vec![];
    setup::test_list(list, expected)
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
    let expected = vec![0, 1, 2];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled_just_one_node() {
    let mut list = LinkedList {
      root: setup::node(1, None),
      marker: PhantomData,
    };
    let op = list.remove_at_beginning();
    assert_eq!(op, Ok(()));
    let expected = vec![];
    setup::test_list(list, expected)
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
    let expected = vec![0, 1, 3];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled_with_first_as_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.remove_item(0);
    assert_eq!(op, Ok(()));
    let expected = vec![1, 2, 3];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled_with_last_as_found_item() {
    let mut list = setup::linked_list_filled();
    let op = list.remove_item(3);
    assert_eq!(op, Ok(()));
    let expected = vec![0, 1, 2];
    setup::test_list(list, expected)
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
mod search {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let item = list.search(4);
    assert_eq!(item, None)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let item = list.search(2);
    let value = unsafe { (*item.unwrap().as_ptr()).value };
    assert_eq!(value, 2)
  }
}

#[cfg(test)]
mod rev {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    list.rev();
    let expected = vec![];
    setup::test_list(list, expected)
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    list.rev();
    let expected = vec![3, 2, 1, 0];
    setup::test_list(list, expected)
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
    let expect = vec![];
    setup::test_list(l1, expect)
  }

  #[test]
  fn to_first_filled_second_empty() {
    let mut l1 = setup::linked_list_filled();
    let l2 = setup::linked_list_empty();
    l1.concat(l2);
    let expect = vec![0, 1, 2, 3];
    setup::test_list(l1, expect)
  }

  #[test]
  fn to_first_empty_second_filled() {
    let mut l1 = setup::linked_list_empty();
    let l2 = setup::linked_list_filled();
    l1.concat(l2);
    let expect = vec![0, 1, 2, 3];
    setup::test_list(l1, expect)
  }

  #[test]
  fn to_both_filled() {
    let mut l1 = LinkedList {
      root: setup::node(0, setup::node(1, setup::node(2, setup::node(3, None)))),
      marker: PhantomData,
    };
    let l2 = LinkedList {
      root: setup::node(4, setup::node(5, setup::node(6, setup::node(7, None)))),
      marker: PhantomData,
    };
    l1.concat(l2);
    let expect = vec![0, 1, 2, 3, 4, 5, 6, 7];
    setup::test_list(l1, expect)
  }
}

#[cfg(test)]
mod iter {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let mut iter = list.iter();
    assert_eq!(iter.next(), None)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None)
  }
}

#[cfg(test)]
mod iter_mut {
  use super::*;

  #[test]
  fn to_empty() {
    let mut list = setup::linked_list_empty();
    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), None)
  }

  #[test]
  fn to_filled() {
    let mut list = setup::linked_list_filled();
    for element in list.iter_mut() {
      *element += 10;
    }
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&11));
    assert_eq!(iter.next(), Some(&12));
    assert_eq!(iter.next(), Some(&13));
    assert_eq!(iter.next(), None)
  }
}

#[cfg(test)]
mod into_inter {
  use super::*;

  #[test]
  fn to_empty() {
    let list = setup::linked_list_empty();
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), None)
  }

  #[test]
  fn to_filled() {
    let list = setup::linked_list_filled();
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None)
  }
}
