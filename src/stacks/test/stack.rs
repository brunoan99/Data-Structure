use super::*;

type StackT = Stack<i32>;

#[test]
fn test_is_empty() {
  let stack = StackT::Empty;
  let op = StackT::empty(&stack);
  assert_eq!(op, true)
}

#[test]
fn test_isnt_empty() {
  let stack = StackT::Node(0, Box::new(StackT::Empty));
  let op = StackT::empty(&stack);
  assert_eq!(op, false)
}

#[test]
fn test_push_to_empty() {
  let stack = StackT::Empty;
  let new_stack = StackT::push(&stack, 0);
  let expected = StackT::Node(0, Box::new(Stack::Empty));
  assert_eq!(new_stack, expected);
}

#[test]
fn test_push_to_non_empty() {
  let stack = StackT::Node(0, Box::new(StackT::Empty));
  let new_stack = StackT::push(&stack, 1);
  let expected = StackT::Node(1, Box::new(Stack::Node(0, Box::new(Stack::Empty))));
  assert_eq!(new_stack, expected)
}

#[test]
fn test_pop_to_empty() {
  let stack = StackT::Empty;
  let op = StackT::pop(&stack);
  assert_eq!(op, None);
}

#[test]
fn test_pop_to_non_empty() {
  let stack = StackT::Node(0, Box::new(StackT::Empty));
  let op = StackT::pop(&stack);
  let expected = Some((0, StackT::Empty));
  assert_eq!(op, expected);
}

#[test]
fn test_pop_until_empty() {
  let s0 = Stack::Node(
    3,
    Box::new(Stack::Node(
      2,
      Box::new(Stack::Node(
        1,
        Box::new(Stack::Node(0, Box::new(Stack::Empty))),
      )),
    )),
  );

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
fn test_peek_to_empty() {
  let stack = StackT::Empty;
  let op = StackT::peek(&stack);
  assert_eq!(op, None);
}

#[test]
fn test_peek_to_non_empty() {
  let stack = StackT::Node(0, Box::new(StackT::Empty));
  let op = StackT::peek(&stack);
  let expected: Option<&i32> = Some(&0);
  assert_eq!(op, expected);
}

#[test]
fn test_find_to_empty() {
  let stack = StackT::Empty;
  let op = StackT::find(&stack, |item: &i32| -> bool { item == &0 });
  assert_eq!(op, None)
}

#[test]
fn test_find_to_non_empty() {
  let stack = StackT::Node(0, Box::new(StackT::Empty));
  let op = StackT::find(&stack, |item: &i32| -> bool { item == &0 });
  assert_eq!(op, Some(&0))
}

#[test]
fn test_find_first_to_non_empty() {
  let stack = Stack::Node(
    (0, 1),
    Box::new(Stack::Node((0, 2), Box::new(Stack::Empty))),
  );
  let op = Stack::find(&stack, |item: &(i32, i32)| -> bool {
    match item {
      (0, _) => true,
      _ => false,
    }
  });
  assert_eq!(op, Some(&(0, 1)))
}

#[test]
fn test_map_to_empty() {
  let stack = StackT::Empty;
  let op = StackT::map(&stack, |item| -> String { item.clone().to_string() });
  assert_eq!(op, Stack::<String>::Empty);
}

#[test]
fn test_map_to_non_empty() {
  let stack = StackT::Node(
    0,
    Box::new(StackT::Node(
      1,
      Box::new(Stack::Node(2, Box::new(Stack::Empty))),
    )),
  );
  let op = StackT::map(&stack, |item| -> String { item.clone().to_string() });
  let expected = Stack::Node(
    String::from("0"),
    Box::new(Stack::Node(
      String::from("1"),
      Box::new(Stack::Node(String::from("2"), Box::new(Stack::Empty))),
    )),
  );
  assert_eq!(op, expected)
}

#[test]
fn test_filter_to_empty() {
  let stack = StackT::Empty;
  let op = StackT::filter(&stack, |item| -> bool { item >= &1 });
  assert_eq!(op, StackT::Empty)
}

#[test]
fn test_filter_to_non_empty() {
  let stack = StackT::Node(
    0,
    Box::new(StackT::Node(
      1,
      Box::new(Stack::Node(2, Box::new(Stack::Empty))),
    )),
  );
  let op = StackT::filter(&stack, |item| -> bool { item > &1 });
  let expected = StackT::Node(2, Box::new(Stack::Empty));
  assert_eq!(op, expected);
}
