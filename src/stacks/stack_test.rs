use super::*;

type StackT = Stack<i32>;

fn node<T>(head: T, tail: Stack<T>) -> Stack<T> {
  Stack::Node(head, Box::new(tail))
}

fn stack_empty() -> StackT {
  StackT::Empty
}

fn stack_filled() -> StackT {
  node(3, node(2, node(1, node(0, Stack::Empty))))
}

#[test]
fn new_return_empty() {
  let op = Stack::<i32>::new();
  let expected = stack_empty();
  assert_eq!(op, expected)
}

#[test]
fn is_empty() {
  let stack = stack_empty();
  let op = StackT::empty(&stack);
  assert_eq!(op, true)
}

#[test]
fn isnt_empty() {
  let stack = stack_filled();
  let op = StackT::empty(&stack);
  assert_eq!(op, false)
}

#[test]
fn push_to_empty() {
  let stack = stack_empty();
  let op = Stack::push(&stack, 0);
  let expected = node(0, stack);
  assert_eq!(op, expected);
}

#[test]
fn push_to_filled() {
  let stack = stack_filled();
  let op = Stack::push(&stack, 4);
  let expected = node(4, stack);
  assert_eq!(op, expected)
}

#[test]
fn pop_to_empty() {
  let stack = stack_empty();
  let op = StackT::pop(&stack);
  assert_eq!(op, None);
}

#[test]
fn pop_to_filled() {
  let stack = stack_filled();
  let op = StackT::pop(&stack);
  let expected = Some((3, node(2, node(1, node(0, Stack::Empty)))));
  assert_eq!(op, expected);
}

#[test]
fn pop_until_empty() {
  let s0 = stack_filled();

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

#[test]
fn peek_to_empty() {
  let stack = stack_empty();
  let op = Stack::peek(&stack);
  assert_eq!(op, None);
}

#[test]
fn peek_to_filled() {
  let stack = stack_filled();
  let op = Stack::peek(&stack);
  let expected = Some(&3);
  assert_eq!(op, expected);
}

#[test]
fn len_to_empty() {
  let stack = stack_empty();
  let op = Stack::len(&stack);
  assert_eq!(op, 0)
}

#[test]
fn len_to_filled() {
  let stack = stack_filled();
  let op = Stack::len(&stack);
  assert_eq!(op, 4)
}

#[test]
fn rev_to_empty() {
  let stack = stack_empty();
  let op = Stack::rev(&stack);
  assert_eq!(op, Stack::Empty);
}

#[test]
fn rev_to_filled() {
  let stack = stack_filled();
  let op = Stack::rev(&stack);
  let expected = node(0, node(1, node(2, node(3, Stack::Empty))));
  assert_eq!(op, expected);
}

#[test]
fn find_to_empty() {
  let stack = stack_empty();
  let op = Stack::find(&stack, |item: &i32| -> bool { item == &0 });
  assert_eq!(op, None)
}

#[test]
fn find_to_filled_with_match() {
  let stack = stack_filled();
  let op = Stack::find(&stack, |item: &i32| -> bool { item == &2 });
  assert_eq!(op, Some(&2))
}

#[test]
fn find_to_filled_without_match() {
  let stack = stack_filled();
  let op = Stack::find(&stack, |item: &i32| -> bool { item == &-1 });
  assert_eq!(op, None)
}

#[test]
fn find_first_to_filled() {
  let stack = node((0, 1), node((0, 2), node((0, 3), Stack::Empty)));
  let op = Stack::find(&stack, |item| matches!(item, (0, _)));
  assert_eq!(op, Some(&(0, 1)))
}

#[test]
fn find_last_to_empty() {
  let stack = stack_empty();
  let op = Stack::find_r(&stack, |_| true);
  assert_eq!(op, None)
}

#[test]
fn find_last_to_filled_with_match() {
  let stack = node((0, 1), node((0, 2), node((0, 3), Stack::Empty)));
  let op = Stack::find_r(&stack, |item| matches!(item, (0, _)));
  assert_eq!(op, Some(&(0, 3)))
}

#[test]
fn find_last_to_filled_without_match() {
  let stack = stack_filled();
  let op = Stack::find_r(&stack, |item: &i32| -> bool { item == &-1 });
  assert_eq!(op, None)
}

#[test]
fn map_to_empty() {
  let stack = stack_empty();
  let op = StackT::map(&stack, |item| -> String { item.clone().to_string() });
  assert_eq!(op, Stack::<String>::Empty);
}

#[test]
fn map_to_filled() {
  let stack = stack_filled();
  let op = Stack::map(&stack, |item| -> String { item.clone().to_string() });
  let expected: Stack<String> = node(
    "3".into(),
    node("2".into(), node("1".into(), node("0".into(), Stack::Empty))),
  );
  assert_eq!(op, expected)
}

#[test]
fn filter_to_empty() {
  let stack = stack_empty();
  let op = Stack::filter(&stack, |item| -> bool { item >= &1 });
  assert_eq!(op, StackT::Empty)
}

#[test]
fn filter_to_filled() {
  let stack = stack_filled();
  let op = Stack::filter(&stack, |item| -> bool { item == &2 });
  let expected = node(2, Stack::Empty);
  assert_eq!(op, expected);
}

#[test]
fn concat_to_both_empty() {
  let s1 = stack_empty();
  let s2 = stack_empty();
  let op = Stack::concat(&s1, &s2);
  assert_eq!(op, Stack::Empty)
}

#[test]
fn concat_s1_empty_s2_filled() {
  let s1 = stack_empty();
  let s2 = stack_filled();
  let op = Stack::concat(&s1, &s2);
  assert_eq!(op, s2)
}

#[test]
fn concat_s1_filled_s2_empty() {
  let s1 = stack_filled();
  let s2 = stack_empty();
  let op = Stack::concat(&s1, &s2);
  assert_eq!(op, s1);
}

#[test]
fn concat_both_filled() {
  let s1 = node(3, node(2, node(1, node(0, Stack::Empty))));
  let s2 = node(7, node(6, node(5, node(4, Stack::Empty))));
  let op = Stack::concat(&s1, &s2);
  let expected = node(
    7,
    node(
      6,
      node(5, node(4, node(3, node(2, node(1, node(0, Stack::Empty)))))),
    ),
  );
  assert_eq!(op, expected);
}

#[test]
fn split_to_empty() {
  let stack = stack_empty();
  let op = Stack::split(&stack, |item| item >= &1);
  assert_eq!(op, (Stack::Empty, Stack::Empty))
}

#[test]
fn split_to_filled_with_only_false_returns() {
  let stack = stack_filled();
  let op = Stack::split(&stack, |_| false);
  let expected = (stack_filled(), Stack::Empty);
  assert_eq!(op, expected)
}

#[test]
fn split_to_filled_with_only_true_returns() {
  let stack = stack_filled();
  let op = Stack::split(&stack, |_| true);
  let expected = (Stack::Empty, stack_filled());
  assert_eq!(op, expected)
}

#[test]
fn split_to_filled_with_balanced_returns() {
  let stack = stack_filled();
  let op = Stack::split(&stack, |item| item > &1);
  let expected = (
    node(1, node(0, Stack::Empty)),
    node(3, node(2, Stack::Empty)),
  );
  assert_eq!(op, expected)
}

#[test]
fn any_to_empty() {
  let stack = stack_empty();
  let op = Stack::any(&stack, |_| -> bool { true });
  assert_eq!(op, false);
}

#[test]
fn any_to_filled_whith_only_false_return() {
  let stack = stack_filled();
  let op = Stack::any(&stack, |_| -> bool { false });
  assert_eq!(op, false);
}

#[test]
fn any_to_filled_with_a_true_return() {
  let stack = stack_filled();
  let op = Stack::any(&stack, |item| -> bool { item > &1 });
  assert_eq!(op, true);
}

#[test]
fn all_to_empty() {
  let stack = stack_empty();
  let op = Stack::all(&stack, |_| -> bool { false });
  assert_eq!(op, true);
}

#[test]
fn all_to_filled_with_a_false_return() {
  let stack = stack_filled();
  let op = Stack::all(&stack, |item| -> bool { item > &5 });
  assert_eq!(op, false);
}

#[test]
fn all_to_filled_with_only_true_return() {
  let stack = stack_filled();
  let op = Stack::all(&stack, |item| -> bool { item >= &0 });
  assert_eq!(op, true);
}

#[test]
fn from_empty_vec() {
  let vec: Vec<i32> = vec![];
  let op = Stack::from(vec);
  let expected = Stack::Empty;
  assert_eq!(op, expected)
}

#[test]
fn from_filled_vec() {
  let vec = vec![0, 1, 2, 3];
  let op = Stack::from(vec);
  let expected = node(3, node(2, node(1, node(0, Stack::Empty))));
  assert_eq!(op, expected)
}

#[test]
fn to_list_from_empty_stack() {
  let stack = stack_empty();
  let op = Vec::from(stack);
  let expected = vec![];
  assert_eq!(op, expected);
}

#[test]
fn to_list_from_filled_stack() {
  let stack = stack_filled();
  let op = Vec::from(stack);
  let expected = vec![0, 1, 2, 3];
  assert_eq!(op, expected);
}
