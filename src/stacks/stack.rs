#[derive(Clone, PartialEq, Debug)]
pub enum Stack<T> {
  Empty,
  Node(T, Box<Stack<T>>),
}

mod private {
  pub use super::Stack;

  pub fn from_list<T>(v: Vec<T>, acc: Stack<T>) -> Stack<T>
  where
    T: PartialEq + Clone + Copy,
  {
    if v.is_empty() {
      acc
    } else {
      from_list(v[1..].to_vec(), Stack::push(&acc, v[0].clone()))
    }
  }

  pub fn to_list<T>(stack: &Stack<T>, acc: Vec<T>) -> Vec<T>
  where
    T: PartialEq + Clone + Copy,
  {
    match stack {
      Stack::Empty => acc,
      Stack::Node(value, stack_remaining) => {
        let mut new_acc = acc;
        new_acc.push(value.clone());
        to_list(&*stack_remaining.to_owned(), new_acc)
      }
    }
  }

  pub fn rev_aux<T>(stack: &Stack<T>, aux: Stack<T>) -> Stack<T>
  where
    T: PartialEq + Clone + Copy,
  {
    match stack {
      Stack::<T>::Empty => aux,
      Stack::Node(value, stack_remaining) => {
        rev_aux(stack_remaining, Stack::push(&aux, value.clone()))
      }
    }
  }

  pub fn keep_aux<'a, T>(stack: &'a Stack<T>, acc: Option<&'a T>) -> Option<&'a T> {
    match stack {
      Stack::Empty => acc,
      Stack::Node(value, stack_remaining) => keep_aux(&*stack_remaining, Some(value)),
    }
  }

  pub fn len_aux<T>(stack: &Stack<T>, acc: i32) -> i32 {
    match stack {
      Stack::Empty => acc,
      Stack::Node(_, stack_remaining) => len_aux(stack_remaining, acc + 1),
    }
  }

  pub fn concat_aux<T>(s1: &Stack<T>, s2: &Stack<T>, acc: Stack<T>) -> Stack<T>
  where
    T: PartialEq + Clone + Copy,
  {
    match (s1, s2) {
      (Stack::Empty, Stack::Empty) => acc,
      (Stack::Node(value, stack_remaining), _) => {
        concat_aux(stack_remaining, s2, Stack::push(&acc, value.clone()))
      }
      (_, Stack::Node(value, stack_remaining)) => {
        concat_aux(s1, stack_remaining, Stack::push(&acc, value.clone()))
      }
    }
  }

  pub fn split_aux<T>(
    stack: &Stack<T>,
    f: fn(&T) -> bool,
    acc1: Stack<T>,
    acc2: Stack<T>,
  ) -> (Stack<T>, Stack<T>)
  where
    T: PartialEq + Clone + Copy,
  {
    match stack {
      Stack::Empty => (Stack::rev(&acc1), Stack::rev(&acc2)),
      Stack::Node(value, stack_remaining) => {
        if f(value) {
          split_aux(stack_remaining, f, acc1, Stack::push(&acc2, value.clone()))
        } else {
          split_aux(stack_remaining, f, Stack::push(&acc1, value.clone()), acc2)
        }
      }
    }
  }

  pub fn find_r_aux<'a, T>(
    stack: &'a Stack<T>,
    f: fn(&T) -> bool,
    acc: Option<&'a T>,
  ) -> Option<&'a T>
  where
    T: PartialEq + Clone + Copy,
  {
    match stack {
      Stack::Empty => acc,
      Stack::Node(value, stack_remaining) => {
        if f(value) {
          find_r_aux(stack_remaining, f, Some(value))
        } else {
          find_r_aux(stack_remaining, f, acc)
        }
      }
    }
  }
}

impl<T> From<Vec<T>> for Stack<T>
where
  T: PartialEq + Clone + Copy,
{
  fn from(value: Vec<T>) -> Self {
    private::from_list(value, Stack::Empty)
  }
}

impl<T> From<Stack<T>> for Vec<T>
where
  T: PartialEq + Clone + Copy,
{
  fn from(value: Stack<T>) -> Self {
    private::to_list(&Stack::rev(&value), vec![])
  }
}

impl<T> Stack<T>
where
  T: PartialEq + Clone + Copy,
{
  pub fn new() -> Self {
    Self::Empty
  }

  pub fn empty(stack: &Self) -> bool {
    matches!(stack, Self::Empty)
  }

  pub fn push(stack: &Self, item: T) -> Self {
    Self::Node(item, Box::new(stack.clone()))
  }

  pub fn pop(stack: &Self) -> Option<(T, Self)> {
    match stack {
      Self::Empty => None,
      Self::Node(value, stack) => Some((value.to_owned(), *stack.clone())),
    }
  }

  pub fn drop(stack: &Self) -> Option<Self> {
    match stack {
      Self::Node(_, stack_remaining) => Some(*stack_remaining.clone()),
      Self::Empty => None,
    }
  }

  pub fn peek(stack: &Self) -> Option<&T> {
    match stack {
      Self::Node(value, _) => Some(value),
      Self::Empty => None,
    }
  }

  pub fn keep(stack: &Self) -> Option<&T> {
    private::keep_aux(stack, None)
  }

  pub fn len(stack: &Self) -> i32 {
    private::len_aux(stack, 0)
  }

  pub fn rev(stack: &Self) -> Self {
    private::rev_aux(stack, Stack::Empty)
  }

  pub fn find(stack: &Self, f: fn(&T) -> bool) -> Option<&T> {
    match stack {
      Self::Empty => None,
      Self::Node(value, stack_remaining) => {
        if f(value) {
          Some(value)
        } else {
          Self::find(stack_remaining, f)
        }
      }
    }
  }

  pub fn find_r(stack: &Self, f: fn(&T) -> bool) -> Option<&T> {
    private::find_r_aux(stack, f, None)
  }

  pub fn map<U>(stack: &Self, f: fn(&T) -> U) -> Stack<U>
  where
    U: PartialEq + Clone,
  {
    match stack {
      Self::Empty => Stack::<U>::Empty,
      Self::Node(value, stack_remaining) => {
        Stack::<U>::Node(f(value), Box::new(Self::map(stack_remaining, f)))
      }
    }
  }

  pub fn filter(stack: &Self, f: fn(&T) -> bool) -> Self {
    match stack {
      Self::Empty => Self::Empty,
      Self::Node(value, stack_remaining) => {
        if f(value) {
          Self::Node(value.clone(), Box::new(Self::filter(stack_remaining, f)))
        } else {
          Self::filter(stack_remaining, f)
        }
      }
    }
  }

  pub fn concat(s1: &Self, s2: &Self) -> Self {
    private::concat_aux(&Stack::rev(s1), &Stack::rev(&s2), Stack::Empty)
  }

  pub fn split(stack: &Self, f: fn(&T) -> bool) -> (Stack<T>, Stack<T>) {
    private::split_aux(stack, f, Stack::Empty, Stack::Empty)
  }

  pub fn any(stack: &Self, f: fn(&T) -> bool) -> bool {
    match stack {
      Self::Empty => false,
      Self::Node(value, stack_remaining) => {
        if f(value) {
          true
        } else {
          Self::any(stack_remaining, f)
        }
      }
    }
  }

  pub fn all(stack: &Self, f: fn(&T) -> bool) -> bool {
    match stack {
      Self::Empty => true,
      Self::Node(value, stack_remaining) => {
        if f(value) {
          Self::all(stack_remaining, f)
        } else {
          false
        }
      }
    }
  }
}

#[cfg(test)]
#[path = "./stack_test.rs"]
mod test;
