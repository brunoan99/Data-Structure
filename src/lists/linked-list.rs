#[derive(Clone, PartialEq, Debug)]
pub enum ListNode<T> {
  Empty,
  Node { value: T, next: Box<ListNode<T>> },
}

#[derive(Clone, PartialEq, Debug)]
pub struct LinkedList<T> {
  root: ListNode<T>,
}

mod private {
  use super::*;

  pub fn insert_node_aux<T>(node: &ListNode<T>, item: T) -> ListNode<T>
  where
    T: Clone,
  {
    match node {
      ListNode::Empty => ListNode::Node {
        value: item,
        next: Box::new(ListNode::Empty),
      },
      ListNode::Node { value, next } => ListNode::Node {
        value: value.clone(),
        next: Box::new(insert_node_aux(next, item)),
      },
    }
  }

  pub fn remove_node_aux<T>(node: &ListNode<T>, item: T) -> ListNode<T>
  where
    T: PartialEq + Clone,
  {
    match node {
      ListNode::Empty => ListNode::Empty,
      ListNode::Node { value, next } => {
        if value.clone() == item {
          *next.to_owned()
        } else {
          ListNode::Node {
            value: value.clone(),
            next: Box::new(remove_node_aux(next, item)),
          }
        }
      }
    }
  }

  pub fn len_aux<T>(node: &ListNode<T>, acc: i32) -> i32 {
    match node {
      ListNode::Empty => acc,
      ListNode::Node { value: _, next } => len_aux(&*next, acc + 1),
    }
  }

  pub fn rev_node_aux<T>(node: &ListNode<T>, acc: ListNode<T>) -> ListNode<T>
  where
    T: Clone,
  {
    match node {
      ListNode::Empty => acc,
      ListNode::Node { value, next } => insert_node_aux(&rev_node_aux(next, acc), value.clone()),
    }
  }

  pub fn concat_nodes_aux<T>(n1: &ListNode<T>, n2: &ListNode<T>, acc: ListNode<T>) -> ListNode<T>
  where
    T: Clone,
  {
    match (n1, n2) {
      (ListNode::Empty, ListNode::Empty) => acc,
      (_, ListNode::Node { value, next }) => {
        concat_nodes_aux(n1, next, insert_node_aux(&acc, value.clone()))
      }
      (ListNode::Node { value, next }, _) => {
        concat_nodes_aux(next, n2, insert_node_aux(&acc, value.clone()))
      }
    }
  }

  pub fn split_node_aux<T>(
    node: &ListNode<T>,
    f: fn(&T) -> bool,
    acc1: ListNode<T>,
    acc2: ListNode<T>,
  ) -> (ListNode<T>, ListNode<T>)
  where
    T: Clone,
  {
    match node {
      ListNode::Empty => (acc1, acc2),
      ListNode::Node { value, next } => {
        if f(value) {
          split_node_aux(next, f, acc1, insert_node_aux(&acc2, value.clone()))
        } else {
          split_node_aux(next, f, insert_node_aux(&acc1, value.clone()), acc2)
        }
      }
    }
  }

  pub fn any_node_aux<T>(node: &ListNode<T>, f: fn(&T) -> bool) -> bool {
    match node {
      ListNode::Empty => false,
      ListNode::Node { value, next } => {
        if f(value) {
          true
        } else {
          any_node_aux(next, f)
        }
      }
    }
  }

  pub fn all_node_aux<T>(node: &ListNode<T>, f: fn(&T) -> bool) -> bool {
    match node {
      ListNode::Empty => true,
      ListNode::Node { value, next } => {
        if f(value) {
          all_node_aux(next, f)
        } else {
          false
        }
      }
    }
  }

  pub fn find_node_aux<T>(node: &ListNode<T>, f: fn(&T) -> bool) -> Option<&T> {
    match node {
      ListNode::Empty => None,
      ListNode::Node { value, next } => {
        if f(value) {
          Some(value)
        } else {
          find_node_aux(next, f)
        }
      }
    }
  }

  pub fn find_r_node_aux<'a, T>(
    node: &'a ListNode<T>,
    f: fn(&T) -> bool,
    acc: Option<&'a T>,
  ) -> Option<&'a T> {
    match node {
      ListNode::Empty => acc,
      ListNode::Node { value, next } => {
        if f(value) {
          find_r_node_aux(next, f, Some(value))
        } else {
          find_r_node_aux(next, f, acc)
        }
      }
    }
  }

  pub fn map_node_aux<T, U>(node: &ListNode<T>, f: fn(&T) -> U) -> ListNode<U> {
    match node {
      ListNode::Empty => ListNode::<U>::Empty,
      ListNode::Node { value, next } => ListNode::<U>::Node {
        value: f(value),
        next: Box::new(map_node_aux(next, f)),
      },
    }
  }

  pub fn filter_node_aux<T>(node: &ListNode<T>, f: fn(&T) -> bool, acc: ListNode<T>) -> ListNode<T>
  where
    T: Clone,
  {
    match node {
      ListNode::Empty => acc,
      ListNode::Node { value, next } => {
        if f(value) {
          filter_node_aux(next, f, insert_node_aux(&acc, value.clone()))
        } else {
          filter_node_aux(next, f, acc)
        }
      }
    }
  }

  pub fn reduce_node_aux<T, U>(node: &ListNode<T>, f: fn(&T, U) -> U, acc: U) -> U {
    match node {
      ListNode::Empty => acc,
      ListNode::Node { value, next } => reduce_node_aux(next, f, f(value, acc)),
    }
  }
}

impl<T> LinkedList<T>
where
  T: PartialEq + Clone + Copy,
{
  pub fn new() -> Self {
    Self {
      root: ListNode::Empty,
    }
  }

  pub fn is_empty(list: &Self) -> bool {
    matches!(
      list,
      Self {
        root: ListNode::Empty
      }
    )
  }

  pub fn insert(list: &Self, item: T) -> Self {
    Self {
      root: private::insert_node_aux(&list.root, item),
    }
  }

  pub fn remove(list: &Self, item: T) -> Self {
    Self {
      root: private::remove_node_aux(&list.root, item),
    }
  }

  pub fn len(list: &Self) -> i32 {
    private::len_aux(&list.root, 0)
  }

  pub fn rev(list: &Self) -> Self {
    Self {
      root: private::rev_node_aux(&list.root, ListNode::Empty),
    }
  }

  pub fn concat(l1: &Self, l2: &Self) -> Self {
    Self {
      root: private::concat_nodes_aux(&l1.root, &l2.root, ListNode::Empty),
    }
  }

  pub fn split(list: &Self, f: fn(&T) -> bool) -> (Self, Self) {
    let (n1, n2) = private::split_node_aux(&list.root, f, ListNode::Empty, ListNode::Empty);
    (Self { root: n1 }, Self { root: n2 })
  }

  pub fn any(list: &Self, f: fn(&T) -> bool) -> bool {
    private::any_node_aux(&list.root, f)
  }

  pub fn all(list: &Self, f: fn(&T) -> bool) -> bool {
    private::all_node_aux(&list.root, f)
  }

  pub fn find(list: &Self, f: fn(&T) -> bool) -> Option<&T> {
    private::find_node_aux(&list.root, f)
  }

  pub fn find_r(list: &Self, f: fn(&T) -> bool) -> Option<&T> {
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
      root: private::filter_node_aux(&list.root, f, ListNode::Empty),
    }
  }

  pub fn reduce<U>(list: &Self, f: fn(&T, U) -> U, acc: U) -> U {
    private::reduce_node_aux(&list.root, f, acc)
  }
}

#[cfg(test)]
#[path = "./linked-list_test.rs"]
mod test;
