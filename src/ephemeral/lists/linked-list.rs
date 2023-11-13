use std::{marker::PhantomData, ptr::NonNull};

type Link<T> = Option<NonNull<ListNode<T>>>;

#[derive(Clone, PartialEq, Debug)]
pub struct ListNode<T> {
  value: T,
  next: Link<T>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LinkedList<T> {
  root: Link<T>,
}

pub struct Iter<'a, T: 'a> {
  root: &'a Link<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if let None = self.root {
      return None;
    }
    self.root.map(|node| unsafe {
      let node = &*node.as_ptr();
      self.root = &node.next;
      &node.value
    })
  }
}

pub struct IterMut<'a, T: 'a> {
  root: Link<T>,
  marker: PhantomData<&'a ListNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<&'a mut T> {
    if let None = self.root {
      return None;
    }
    self.root.map(|node| unsafe {
      let node = &mut *node.as_ptr();
      self.root = node.next;
      &mut node.value
    })
  }
}

#[derive(Clone)]
pub struct IntoIter<T> {
  list: LinkedList<T>,
}

impl<T: Copy> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.list.root.map(|node| unsafe {
      let node = &mut *node.as_ptr();
      self.list.root = node.next;
      node.value
    })
  }
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

impl<T: PartialEq + Copy + Clone> LinkedList<T> {
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn is_empty(&self) -> bool {
    matches!(self.root, None)
  }

  pub fn insert_at_beginning(&mut self, item: T) {
    let new_root = NonNull::new(&mut ListNode {
      value: item,
      next: self.root,
    });
    self.root = new_root;
  }

  pub fn insert_at_end(&mut self, item: T) {
    let mut tmp_node = self.root;
    while let Some(inner_node) = tmp_node {
      unsafe {
        let next = (*inner_node.as_ptr()).next;
        if let None = next {
          let node = inner_node.as_ptr();
          (*node).next = NonNull::new(&mut ListNode {
            value: item,
            next: None,
          });
          break;
        }
        tmp_node = (*inner_node.as_ptr()).next;
      }
    }
  }

  pub fn insert_before(&mut self, item: T, before: T) -> Result<(), InsertError> {
    let mut tmp_node = self.root;
    while let Some(actual_node) = tmp_node {
      if (unsafe { (*actual_node.as_ptr()).value }) == before {
        tmp_node = NonNull::new(&mut ListNode {
          value: item,
          next: Some(actual_node),
        });
        return Ok(());
      }
    }
    return Err(InsertError::BeforeItemNotFound);
  }

  pub fn insert_after(&mut self, item: T, after: T) -> Result<(), InsertError> {
    let mut tmp_node = self.root;
    while let Some(actual_node) = tmp_node {
      let value = unsafe { (*actual_node.as_ptr()).value };
      if value == after {
        let next = unsafe { (*actual_node.as_ptr()).next };
        tmp_node = NonNull::new(&mut ListNode {
          value,
          next: NonNull::new(&mut ListNode { value: item, next }),
        });
        return Ok(());
      }
      tmp_node = unsafe { (*actual_node.as_ptr()).next }
    }
    return Err(InsertError::AfterItemNotFound);
  }

  pub fn remove_at_beginning(&mut self) -> Result<(), RemoveError> {
    let node = self.root;
    if let None = node {
      return Err(RemoveError::EmptyList);
    }
    self.root = unsafe { (*node.unwrap().as_ptr()).next };
    Ok(())
  }

  pub fn remove_at_end(&mut self) -> Result<(), RemoveError> {
    let node = self.root;
    if let None = node {
      return Err(RemoveError::EmptyList);
    }
    let mut tmp_node = node;
    let next = unsafe { (*node.unwrap().as_ptr()).next };
    while let Some(_) = next {
      tmp_node = unsafe { (*tmp_node.unwrap().as_ptr()).next };
    }
    tmp_node = None;
    Ok(())
  }

  pub fn remove_item(&mut self, item: T) -> Result<(), RemoveError> {
    let node = self.root;
    if let None = node {
      return Err(RemoveError::EmptyList);
    }
    let mut tmp_node = node;
    while let Some(actual_node) = tmp_node {
      let value = unsafe { (*actual_node.as_ptr()).value };
      if value == item {
        let next = unsafe { (*actual_node.as_ptr()).next };
        tmp_node = next;
        return Ok(());
      }
      tmp_node = unsafe { (*actual_node.as_ptr()).next };
    }
    return Err(RemoveError::ItemNotFound);
  }

  pub fn len(&self) -> i32 {
    let mut count = 0;
    let mut tmp_node = self.root;
    while let Some(inner_node) = tmp_node {
      count += 1;
      tmp_node = unsafe { (*inner_node.as_ptr()).next };
    }
    count
  }

  pub fn rev(&mut self) {
    let mut current = self.root;
    let mut prev: Link<T> = None;

    while let Some(current_inner) = current {
      let next = unsafe { (*current_inner.as_ptr()).next };
      unsafe {
        (*current_inner.as_ptr()).next = prev;
      }
      prev = Some(current_inner);
      current = next;
    }

    self.root = prev;
  }

  pub fn concat(&mut self, list: Self) {
    let mut tmp_node = &mut self.root;
    while let Some(inner_node) = tmp_node {
      tmp_node = unsafe { &mut (*inner_node.as_ptr()).next };
    }
    *tmp_node = list.root
  }
}

#[cfg(test)]
#[path = "./linked-list_test.rs"]
mod test;
