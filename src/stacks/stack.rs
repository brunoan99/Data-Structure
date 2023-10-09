#[derive(Clone, PartialEq, Debug)]
pub enum Stack<T> {
  Empty,
  Node(T, Box<Stack<T>>),
}

impl<T> Stack<T>
where
  T: Clone + PartialEq,
{
  pub fn empty(stack: &Self) -> bool {
    matches!(stack, Self::Empty)
  }

  #[must_use]
  pub fn push(stack: &Self, item: T) -> Self {
    Self::Node(item, Box::new(stack.clone()))
  }

  #[must_use]
  pub fn pop(stack: &Self) -> Option<(T, Self)> {
    match stack {
      Self::Empty => None,
      Self::Node(value, stack) => Some((value.to_owned(), *stack.clone())),
    }
  }

  pub fn peek(stack: &Self) -> Option<&T> {
    match stack {
      Self::Node(value, _) => Some(value),
      Self::Empty => None,
    }
  }

  pub fn find(stack: &Self, f: fn(&T) -> bool) -> Option<&T> {
    match stack {
      Self::Empty => None,
      Self::Node(value, stack_remaining) => match f(value) {
        true => Some(value),
        false => Self::find(stack_remaining, f),
      },
    }
  }

  pub fn map<U>(stack: &Self, f: fn(&T) -> U) -> Stack<U>
  where
    U: Clone + PartialEq,
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
      Self::Node(value, stack_remaining) => match f(value) {
        true => Self::Node(value.clone(), Box::new(Self::filter(stack_remaining, f))),
        false => Self::filter(stack_remaining, f),
      },
    }
  }

  pub fn any(stack: &Self, f: fn(&T) -> bool) -> bool {
    match stack {
      Self::Empty => false,
      Self::Node(value, stack_remaining) => match f(value) {
        true => true,
        false => Self::any(&stack_remaining, f),
      },
    }
  }

  pub fn all(stack: &Self, f: fn(&T) -> bool) -> bool {
    match stack {
      Self::Empty => true,
      Self::Node(value, stack_remaining) => match f(value) {
        true => Self::all(&stack_remaining, f),
        false => false,
      },
    }
  }
}

#[cfg(test)]
#[path = "./test/stack.rs"]
mod test;
