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
  marker: PhantomData<T>,
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

mod private {
  use super::*;

  pub fn new_link<T>(value: T, next: Link<T>) -> Link<T> {
    unsafe {
      Some(NonNull::new_unchecked(Box::into_raw(Box::new(ListNode {
        value,
        next,
      }))))
    }
  }

  pub fn get_next<T>(node: Link<T>) -> Link<T> {
    unsafe {
      if let Some(inner_node) = node {
        (*inner_node.as_ptr()).next
      } else {
        None
      }
    }
  }
}

impl<T: PartialEq + Copy + Clone> LinkedList<T> {
  pub fn new() -> Self {
    Self {
      root: None,
      marker: PhantomData,
    }
  }

  pub fn is_empty(&self) -> bool {
    matches!(self.root, None)
  }

  pub fn insert_at_beginning(&mut self, item: T) {
    let new_root = private::new_link(item, self.root);
    self.root = new_root;
  }

  pub fn insert_at_end(&mut self, item: T) {
    unsafe {
      let new_node = private::new_link(item, None);
      let mut ptr = self.root;
      while let Some(in_next) = private::get_next(ptr) {
        ptr = Some(in_next);
      }
      match ptr {
        None => {
          self.root = new_node;
        }
        Some(inner) => {
          (*inner.as_ptr()).next = new_node;
        }
      }
    }
  }

  pub fn insert_before(&mut self, item: T, before: T) -> Result<(), InsertError> {
    unsafe {
      let mut ptr = self.root;
      let mut preptr: Link<T> = None;
      while let Some(inner) = ptr {
        let inner_ptr = inner.as_ptr();
        if (*inner_ptr).value == before {
          let new_node = private::new_link(item, ptr);
          match preptr {
            None => self.root = new_node,
            Some(inner_preptr) => (*inner_preptr.as_ptr()).next = new_node,
          }
          return Ok(());
        }
        preptr = ptr;
        ptr = (*inner_ptr).next;
      }
      return Err(InsertError::BeforeItemNotFound);
    }
  }

  pub fn insert_after(&mut self, item: T, after: T) -> Result<(), InsertError> {
    unsafe {
      let mut ptr = self.root.and_then(|node| (*node.as_ptr()).next);
      let mut preptr = self.root;
      while let Some(inner_preptr) = preptr {
        let inner_ptr = inner_preptr.as_ptr();
        if (*inner_ptr).value == after {
          (*inner_ptr).next = private::new_link(item, ptr);
          return Ok(());
        }
        preptr = ptr;
        ptr = ptr.and_then(|node| (*node.as_ptr()).next);
      }
      return Err(InsertError::AfterItemNotFound);
    }
  }

  pub fn remove_at_beginning(&mut self) -> Result<(), RemoveError> {
    unsafe {
      if let None = self.root {
        return Err(RemoveError::EmptyList);
      }
      self.root = (*self.root.unwrap().as_ptr()).next;
      Ok(())
    }
  }

  pub fn remove_at_end(&mut self) -> Result<(), RemoveError> {
    unsafe {
      if let None = self.root {
        return Err(RemoveError::EmptyList);
      }
      let mut ptr = self.root;
      let mut preptr: Link<T> = None;
      while let Some(inner_next) = private::get_next(ptr) {
        preptr = ptr;
        ptr = Some(inner_next);
      }
      (*preptr.unwrap().as_ptr()).next = None;
      Ok(())
    }
  }

  pub fn remove_item(&mut self, item: T) -> Result<(), RemoveError> {
    unsafe {
      if let None = self.root {
        return Err(RemoveError::EmptyList);
      }
      let mut ptr = self.root;
      let mut preptr: Link<T> = None;
      while let Some(inner) = ptr {
        let in_ptr = inner.as_ptr();
        if (*in_ptr).value == item {
          match preptr {
            None => self.root = private::get_next(ptr),
            Some(_) => (*preptr.unwrap().as_ptr()).next = private::get_next(ptr),
          }
          return Ok(());
        }
        preptr = ptr;
        ptr = (*in_ptr).next;
      }
      return Err(RemoveError::ItemNotFound);
    }
  }

  pub fn len(&self) -> i32 {
    let mut count = 0;
    let mut ptr = self.root;
    while let Some(inner) = ptr {
      count += 1;
      ptr = unsafe { (*inner.as_ptr()).next };
    }
    count
  }

  pub fn search(&mut self, item: T) -> Link<T> {
    unsafe {
      let mut ptr = self.root;
      while let Some(inner) = ptr {
        let in_ptr = inner.as_ptr();
        if (*in_ptr).value == item {
          return ptr;
        }
        ptr = (*in_ptr).next;
      }
      return None;
    }
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
