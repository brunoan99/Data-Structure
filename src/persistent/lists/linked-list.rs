type Link<T> = Option<Box<ListNode<T>>>;

#[derive(Clone, PartialEq, Debug)]
pub struct ListNode<T> {
  value: T,
  next: Link<T>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LinkedList<T> {
  root: Link<T>,
}

#[derive(PartialEq, Debug)]
pub enum InsertError {
  BeforeItemNotFound,
  AfterItemNotFound,
}

#[derive(PartialEq, Debug)]
pub enum RemoveError {
  EmptyList,
  ItemNotFound,
}

mod private {
  use super::*;

  pub fn insert_at_beginning_node_aux<T>(node: &Link<T>, item: T) -> Link<T>
  where
    T: Clone,
  {
    Some(Box::new(ListNode {
      value: item,
      next: node.clone(),
    }))
  }

  pub fn insert_at_end_node_aux<T>(node: &Link<T>, item: T) -> Link<T>
  where
    T: Copy,
  {
    match node {
      None => Some(Box::new(ListNode {
        value: item,
        next: None,
      })),
      Some(node) => Some(Box::new(ListNode {
        value: node.value,
        next: insert_at_end_node_aux(&node.next, item),
      })),
    }
  }

  pub fn insert_before_node_aux<T>(
    node: &Link<T>,
    item: T,
    before: &T,
  ) -> Result<Link<T>, InsertError>
  where
    T: PartialEq + Copy,
  {
    match node {
      Some(inner_node) => {
        if &inner_node.value == before {
          Ok(Some(Box::new(ListNode {
            value: item,
            next: Some(Box::new(ListNode {
              value: inner_node.value,
              next: inner_node.next.clone(),
            })),
          })))
        } else {
          match insert_before_node_aux(&inner_node.next, item, before) {
            Ok(link) => Ok(Some(Box::new(ListNode {
              value: inner_node.value,
              next: link,
            }))),
            Err(err) => Err(err),
          }
        }
      }
      None => Err(InsertError::BeforeItemNotFound),
    }
  }

  pub fn insert_after_node_aux<T>(
    node: &Link<T>,
    item: T,
    after: &T,
  ) -> Result<Link<T>, InsertError>
  where
    T: PartialEq + Copy,
  {
    match node {
      Some(inner_node) => {
        if &inner_node.value == after {
          Ok(Some(Box::new(ListNode {
            value: inner_node.value,
            next: Some(Box::new(ListNode {
              value: item,
              next: inner_node.next.clone(),
            })),
          })))
        } else {
          match insert_after_node_aux(&inner_node.next, item, after) {
            Ok(link) => Ok(Some(Box::new(ListNode {
              value: inner_node.value,
              next: link,
            }))),
            Err(err) => Err(err),
          }
        }
      }
      None => Err(InsertError::AfterItemNotFound),
    }
  }

  pub fn remove_item_node_aux<T>(node: &Link<T>, item: T) -> Result<Link<T>, RemoveError>
  where
    T: PartialEq + Copy,
  {
    match node {
      None => Err(RemoveError::ItemNotFound),
      Some(inner_node) => {
        if inner_node.value == item {
          Ok(inner_node.next.clone())
        } else {
          match remove_item_node_aux(&inner_node.next, item) {
            Ok(link) => Ok(Some(Box::new(ListNode {
              value: inner_node.value,
              next: link,
            }))),
            Err(err) => Err(err),
          }
        }
      }
    }
  }

  pub fn remove_at_beginning_node_aux<T>(node: &Link<T>) -> Result<Link<T>, RemoveError>
  where
    T: PartialEq + Clone,
  {
    match node {
      None => Err(RemoveError::EmptyList),
      Some(inner_node) => Ok(inner_node.next.clone()),
    }
  }

  pub fn remove_at_end_node_aux<T>(node: &Link<T>) -> Link<T>
  where
    T: PartialEq + Copy,
  {
    match node {
      None => None,
      Some(inner_node) => match inner_node.next {
        Some(_) => Some(Box::new(ListNode {
          value: inner_node.value,
          next: remove_at_end_node_aux(&inner_node.next),
        })),
        None => None,
      },
    }
  }

  pub fn len_aux<T>(node: &Link<T>, acc: i32) -> i32 {
    match node {
      None => acc,
      Some(inner_node) => len_aux(&inner_node.next, acc + 1),
    }
  }

  pub fn rev_node_aux<T>(node: &Link<T>, acc: Link<T>) -> Link<T>
  where
    T: Copy,
  {
    match node {
      None => acc,
      Some(inner_node) => {
        insert_at_end_node_aux(&rev_node_aux(&inner_node.next, acc), inner_node.value)
      }
    }
  }

  pub fn concat_nodes_aux<T>(n1: &Link<T>, n2: &Link<T>, acc: Link<T>) -> Link<T>
  where
    T: Copy,
  {
    match (n1, n2) {
      (None, None) => acc,
      (_, Some(inner_node)) => concat_nodes_aux(
        n1,
        &inner_node.next,
        insert_at_end_node_aux(&acc, inner_node.value),
      ),
      (Some(inner_node), _) => concat_nodes_aux(
        &inner_node.next,
        n2,
        insert_at_end_node_aux(&acc, inner_node.value),
      ),
    }
  }

  pub fn split_node_aux<T>(
    node: &Link<T>,
    f: fn(&T) -> bool,
    acc1: Link<T>,
    acc2: Link<T>,
  ) -> (Link<T>, Link<T>)
  where
    T: Copy,
  {
    match node {
      None => (acc1, acc2),
      Some(inner_node) => {
        if f(&inner_node.value) {
          split_node_aux(
            &inner_node.next,
            f,
            acc1,
            insert_at_end_node_aux(&acc2, inner_node.value),
          )
        } else {
          split_node_aux(
            &inner_node.next,
            f,
            insert_at_end_node_aux(&acc1, inner_node.value),
            acc2,
          )
        }
      }
    }
  }

  pub fn any_node_aux<T>(node: &Link<T>, f: fn(&T) -> bool) -> bool {
    match node {
      None => false,
      Some(inner_node) => {
        if f(&inner_node.value) {
          true
        } else {
          any_node_aux(&inner_node.next, f)
        }
      }
    }
  }

  pub fn all_node_aux<T>(node: &Link<T>, f: fn(&T) -> bool) -> bool {
    match node {
      None => true,
      Some(inner_node) => {
        if f(&inner_node.value) {
          all_node_aux(&inner_node.next, f)
        } else {
          false
        }
      }
    }
  }

  pub fn find_node_aux<T>(node: &Link<T>, f: fn(&T) -> bool) -> Option<T>
  where
    T: Copy,
  {
    match node {
      None => None,
      Some(inner_node) => {
        if f(&inner_node.value) {
          Some(inner_node.value)
        } else {
          find_node_aux(&inner_node.next, f)
        }
      }
    }
  }

  pub fn find_r_node_aux<T>(node: &Link<T>, f: fn(&T) -> bool, acc: Option<T>) -> Option<T>
  where
    T: Copy,
  {
    match node {
      None => acc,
      Some(inner_node) => {
        if f(&inner_node.value) {
          find_r_node_aux(&inner_node.next, f, Some(inner_node.value))
        } else {
          find_r_node_aux(&inner_node.next, f, acc)
        }
      }
    }
  }

  pub fn map_node_aux<T, U>(node: &Link<T>, f: fn(&T) -> U) -> Link<U> {
    match node {
      None => None,
      Some(inner_node) => Some(Box::new(ListNode {
        value: f(&inner_node.value),
        next: map_node_aux(&inner_node.next, f),
      })),
    }
  }

  pub fn filter_node_aux<T>(node: &Link<T>, f: fn(&T) -> bool, acc: Link<T>) -> Link<T>
  where
    T: Clone + Copy,
  {
    match node {
      None => acc,
      Some(inner_node) => {
        if f(&inner_node.value) {
          filter_node_aux(
            &inner_node.next,
            f,
            insert_at_end_node_aux(&acc, inner_node.value),
          )
        } else {
          filter_node_aux(&inner_node.next, f, acc)
        }
      }
    }
  }

  pub fn reduce_node_aux<T, U>(node: &Link<T>, f: fn(&T, U) -> U, acc: U) -> U {
    match node {
      None => acc,
      Some(inner_node) => reduce_node_aux(&inner_node.next, f, f(&inner_node.value, acc)),
    }
  }
}

impl<T> LinkedList<T>
where
  T: PartialEq + Clone + Copy,
{
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn is_empty(list: &Self) -> bool {
    matches!(list.root, None)
  }

  pub fn insert_at_beginning(list: &Self, item: T) -> Self {
    Self {
      root: private::insert_at_beginning_node_aux(&list.root, item),
    }
  }

  pub fn insert_at_end(list: &Self, item: T) -> Self {
    Self {
      root: private::insert_at_end_node_aux(&list.root, item),
    }
  }

  pub fn insert_before(list: &Self, item: T, before: &T) -> Result<Self, InsertError> {
    match private::insert_before_node_aux(&list.root, item, before) {
      Ok(link) => Ok(Self { root: link }),
      Err(err) => Err(err),
    }
  }

  pub fn insert_after(list: &Self, item: T, after: &T) -> Result<Self, InsertError> {
    match private::insert_after_node_aux(&list.root, item, after) {
      Ok(link) => Ok(Self { root: link }),
      Err(err) => Err(err),
    }
  }

  pub fn remove_at_beginning(list: &Self) -> Result<Self, RemoveError> {
    match private::remove_at_beginning_node_aux(&list.root) {
      Ok(link) => Ok(Self { root: link }),
      Err(err) => Err(err),
    }
  }

  pub fn remove_at_end(list: &Self) -> Result<Self, RemoveError> {
    if let None = list.root {
      return Err(RemoveError::EmptyList);
    }
    Ok(Self {
      root: private::remove_at_end_node_aux(&list.root),
    })
  }

  pub fn remove_item(list: &Self, item: T) -> Result<Self, RemoveError> {
    if let None = list.root {
      return Err(RemoveError::EmptyList);
    }
    match private::remove_item_node_aux(&list.root, item) {
      Ok(link) => Ok(Self { root: link }),
      Err(err) => Err(err),
    }
  }

  pub fn len(list: &Self) -> i32 {
    private::len_aux(&list.root, 0)
  }

  pub fn rev(list: &Self) -> Self {
    Self {
      root: private::rev_node_aux(&list.root, None),
    }
  }

  pub fn concat(l1: &Self, l2: &Self) -> Self {
    Self {
      root: private::concat_nodes_aux(&l1.root, &l2.root, None),
    }
  }

  pub fn split(list: &Self, f: fn(&T) -> bool) -> (Self, Self) {
    let (n1, n2) = private::split_node_aux(&list.root, f, None, None);
    (Self { root: n1 }, Self { root: n2 })
  }

  pub fn any(list: &Self, f: fn(&T) -> bool) -> bool {
    private::any_node_aux(&list.root, f)
  }

  pub fn all(list: &Self, f: fn(&T) -> bool) -> bool {
    private::all_node_aux(&list.root, f)
  }

  pub fn find(list: &Self, f: fn(&T) -> bool) -> Option<T> {
    private::find_node_aux(&list.root, f)
  }

  pub fn find_r(list: &Self, f: fn(&T) -> bool) -> Option<T> {
    private::find_r_node_aux(&list.root, f, None)
  }

  pub fn map<U>(list: &Self, f: fn(&T) -> U) -> LinkedList<U>
  where
    U: Clone + PartialEq + Copy,
  {
    LinkedList::<U> {
      root: private::map_node_aux(&list.root, f),
    }
  }

  pub fn filter(list: &Self, f: fn(&T) -> bool) -> Self {
    Self {
      root: private::filter_node_aux(&list.root, f, None),
    }
  }

  pub fn reduce<U>(list: &Self, f: fn(&T, U) -> U, acc: U) -> U {
    private::reduce_node_aux(&list.root, f, acc)
  }
}

#[cfg(test)]
#[path = "./linked-list_test.rs"]
mod test;
