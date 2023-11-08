use super::*;

mod setup {
  use super::*;

  pub type StackT = Stack<i32>;

  pub fn node<T>(head: T, tail: Stack<T>) -> Stack<T> {
    Stack::Node(head, Box::new(tail))
  }

  pub fn stack_empty() -> StackT {
    Stack::Empty
  }

  pub fn stack_filled() -> StackT {
    node(
      3,
      setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
    )
  }
}

#[cfg(test)]
mod new {
  use super::*;

  #[test]
  fn new_return_empty() {
    let op = Stack::<i32>::new();
    let expected = setup::stack_empty();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod is_empty {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::is_empty(&stack);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::is_empty(&stack);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod push {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::push(&stack, 0);
    let expected = setup::node(0, stack);
    assert_eq!(op, expected);
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::push(&stack, 4);
    let expected = setup::node(4, stack);
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod pop {
  use super::*;

  #[test]
  fn pop_to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::pop(&stack);
    assert_eq!(op, None);
  }

  #[test]
  fn pop_to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::pop(&stack);
    let expected = Some((
      3,
      setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
    ));
    assert_eq!(op, expected);
  }

  #[test]
  fn pop_until_isnt_empty() {
    let s0 = setup::stack_filled();

    let op0 = Stack::pop(&s0);
    assert_eq!(op0.is_some(), true);
    let (v0, s1) = op0.unwrap();
    assert_eq!(v0, 3);

    let op1 = Stack::pop(&s1);
    assert_eq!(op1.is_some(), true);
    let (v1, s2) = op1.unwrap();
    assert_eq!(v1, 2);

    let op2 = Stack::pop(&s2);
    assert_eq!(op2.is_some(), true);
    let (v2, s3) = op2.unwrap();
    assert_eq!(v2, 1);

    let op3 = Stack::pop(&s3);
    assert_eq!(op3.is_some(), true);
    let (v3, s4) = op3.unwrap();
    assert_eq!(v3, 0);

    assert_eq!(s4, Stack::Empty)
  }
}

#[cfg(test)]
mod drop {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::drop(&stack);
    assert_eq!(op, None);
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::drop(&stack);
    let expected = Some(setup::node(2, setup::node(1, setup::node(0, Stack::Empty))));
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod peek {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::peek(&stack);
    assert_eq!(op, None);
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::peek(&stack);
    let expected = Some(&3);
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod keep {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::keep(&stack);
    assert_eq!(op, None);
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::keep(&stack);
    let expected = Some(&0);
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod len {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::len(&stack);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::len(&stack);
    assert_eq!(op, 4)
  }
}

#[cfg(test)]
mod rev {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::rev(&stack);
    assert_eq!(op, Stack::Empty);
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::rev(&stack);
    let expected = setup::node(
      0,
      setup::node(1, setup::node(2, setup::node(3, Stack::Empty))),
    );
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod concat {
  use super::*;

  #[test]
  fn to_both_empty() {
    let s1 = setup::stack_empty();
    let s2 = setup::stack_empty();
    let op = Stack::concat(&s1, &s2);
    assert_eq!(op, Stack::Empty)
  }

  #[test]
  fn s1_empty_s2_filled() {
    let s1 = setup::stack_empty();
    let s2 = setup::stack_filled();
    let op = Stack::concat(&s1, &s2);
    assert_eq!(op, s2)
  }

  #[test]
  fn s1_filled_s2_empty() {
    let s1 = setup::stack_filled();
    let s2 = setup::stack_empty();
    let op = Stack::concat(&s1, &s2);
    assert_eq!(op, s1);
  }

  #[test]
  fn both_filled() {
    let s1 = setup::node(
      7,
      setup::node(6, setup::node(5, setup::node(4, Stack::Empty))),
    );
    let s2 = setup::node(
      3,
      setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
    );
    let op = Stack::concat(&s1, &s2);
    let expected = setup::node(
      7,
      setup::node(
        6,
        setup::node(
          5,
          setup::node(
            4,
            setup::node(
              3,
              setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
            ),
          ),
        ),
      ),
    );
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod split {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::split(&stack, |item| item >= &1);
    assert_eq!(op, (Stack::Empty, Stack::Empty))
  }

  #[test]
  fn to_filled_with_only_false_returns() {
    let stack = setup::stack_filled();
    let op = Stack::split(&stack, |_| false);
    let expected = (setup::stack_filled(), Stack::Empty);
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_only_true_returns() {
    let stack = setup::stack_filled();
    let op = Stack::split(&stack, |_| true);
    let expected = (Stack::Empty, setup::stack_filled());
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_with_balanced_returns() {
    let stack = setup::stack_filled();
    let op = Stack::split(&stack, |item| item > &1);
    let expected = (
      setup::node(1, setup::node(0, Stack::Empty)),
      setup::node(3, setup::node(2, Stack::Empty)),
    );
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod any {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::any(&stack, |_| true);
    assert_eq!(op, false);
  }

  #[test]
  fn to_filled_whith_only_false_return() {
    let stack = setup::stack_filled();
    let op = Stack::any(&stack, |_| false);
    assert_eq!(op, false);
  }

  #[test]
  fn to_filled_with_a_true_return() {
    let stack = setup::stack_filled();
    let op = Stack::any(&stack, |item| item > &1);
    assert_eq!(op, true);
  }
}

#[cfg(test)]
mod all {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::all(&stack, |_| false);
    assert_eq!(op, true);
  }

  #[test]
  fn to_filled_with_a_false_return() {
    let stack = setup::stack_filled();
    let op = Stack::all(&stack, |item| item > &5);
    assert_eq!(op, false);
  }

  #[test]
  fn to_filled_with_only_true_return() {
    let stack = setup::stack_filled();
    let op = Stack::all(&stack, |item| item >= &0);
    assert_eq!(op, true);
  }
}

#[cfg(test)]
mod find {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::find(&stack, |item: &i32| item == &0);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_with_match() {
    let stack = setup::stack_filled();
    let op = Stack::find(&stack, |item: &i32| item == &2);
    assert_eq!(op, Some(&2))
  }

  #[test]
  fn to_filled_without_match() {
    let stack = setup::stack_filled();
    let op = Stack::find(&stack, |item: &i32| item == &-1);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_check_if_gets_first() {
    let stack = setup::node(
      (0, 1),
      setup::node((0, 2), setup::node((0, 3), Stack::Empty)),
    );
    let op = Stack::find(&stack, |item| matches!(item, (0, _)));
    assert_eq!(op, Some(&(0, 1)))
  }
}

#[cfg(test)]
mod find_r {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::find_r(&stack, |_| true);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_with_match() {
    let stack = setup::node(
      (0, 1),
      setup::node((0, 2), setup::node((0, 3), Stack::Empty)),
    );
    let op = Stack::find_r(&stack, |item| matches!(item, (0, _)));
    assert_eq!(op, Some(&(0, 3)))
  }

  #[test]
  fn to_filled_without_match() {
    let stack = setup::stack_filled();
    let op = Stack::find_r(&stack, |item: &i32| item == &-1);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_check_if_gets_last() {
    let stack = setup::node(
      (0, 1),
      setup::node((0, 2), setup::node((0, 3), Stack::Empty)),
    );
    let op = Stack::find_r(&stack, |item| matches!(item, (0, _)));
    assert_eq!(op, Some(&(0, 3)))
  }
}

#[cfg(test)]
mod map {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::map(&stack, |item| -> String { item.clone().to_string() });
    assert_eq!(op, Stack::<String>::Empty);
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::map(&stack, |item| -> String { item.clone().to_string() });
    let expected: Stack<String> = setup::node(
      "3".into(),
      setup::node(
        "2".into(),
        setup::node("1".into(), setup::node("0".into(), Stack::Empty)),
      ),
    );
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod filter {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::filter(&stack, |item| item >= &1);
    assert_eq!(op, Stack::Empty)
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::filter(&stack, |item| item == &2);
    let expected = setup::node(2, Stack::Empty);
    assert_eq!(op, expected);
  }
}

#[cfg(test)]
mod reduce {
  use super::*;

  #[test]
  fn to_empty() {
    let stack = setup::stack_empty();
    let op = Stack::reduce(&stack, |item, acc| acc + item, 0);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled() {
    let stack = setup::stack_filled();
    let op = Stack::reduce(&stack, |item, acc| acc + item, 0);
    assert_eq!(op, 6)
  }
}

#[cfg(test)]
mod from_vec {
  use super::*;

  #[test]
  fn from_empty() {
    let vec: Vec<i32> = vec![];
    let op = Stack::from(vec);
    let expected = Stack::Empty;
    assert_eq!(op, expected)
  }

  #[test]
  fn from_filled() {
    let vec = vec![0, 1, 2, 3];
    let op = Stack::from(vec);
    let expected = setup::node(
      3,
      setup::node(2, setup::node(1, setup::node(0, Stack::Empty))),
    );
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod to_vec {
  use super::*;

  #[test]
  fn from_empty_stack() {
    let stack = setup::stack_empty();
    let op = Vec::from(stack);
    let expected = vec![];
    assert_eq!(op, expected);
  }

  #[test]
  fn from_filled_stack() {
    let stack = setup::stack_filled();
    let op = Vec::from(stack);
    let expected = vec![0, 1, 2, 3];
    assert_eq!(op, expected);
  }
}
