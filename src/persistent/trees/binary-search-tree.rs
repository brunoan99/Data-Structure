#[derive(Clone, PartialEq, Debug)]
pub enum TreeNode<T> {
  Empty,
  Node {
    value: T,
    left: Box<TreeNode<T>>,
    right: Box<TreeNode<T>>,
  },
}

// Invariants:
//    - the left_node value is always <= node value
//    - the right_node value is always > node value
#[derive(Clone, PartialEq, Debug)]
pub struct BinarySearchTree<T> {
  root: TreeNode<T>,
}

mod private {
  use super::*;

  pub fn insert_node_aux<T>(node: &TreeNode<T>, item: T) -> TreeNode<T>
  where
    T: Clone + PartialOrd,
  {
    match node {
      TreeNode::Empty => TreeNode::Node {
        value: item,
        left: Box::new(TreeNode::Empty),
        right: Box::new(TreeNode::Empty),
      },
      TreeNode::Node { value, left, right } => {
        if &item > value {
          TreeNode::Node {
            value: value.clone(),
            left: left.clone(),
            right: Box::new(insert_node_aux(right, item)),
          }
        } else {
          TreeNode::Node {
            value: value.clone(),
            left: Box::new(insert_node_aux(left, item)),
            right: right.clone(),
          }
        }
      }
    }
  }

  fn in_order_sucessor<T>(node: &TreeNode<T>, acc: T) -> T
  where
    T: Clone,
  {
    match node {
      TreeNode::Empty => acc,
      TreeNode::Node {
        value,
        left,
        right: _,
      } => match *left.clone() {
        TreeNode::Empty => value.clone(),
        TreeNode::Node { .. } => in_order_sucessor(left, value.clone()),
      },
    }
  }

  pub fn remove_node_aux<T>(node: &TreeNode<T>, item: T) -> TreeNode<T>
  where
    T: Clone + PartialOrd,
  {
    match node {
      TreeNode::Empty => TreeNode::Empty,
      TreeNode::Node { value, left, right } => {
        match (value.clone() == item, *left.clone(), *right.clone()) {
          (false, _, _) => {
            if &item > value {
              TreeNode::Node {
                value: value.clone(),
                left: left.clone(),
                right: Box::new(remove_node_aux(right, item)),
              }
            } else {
              TreeNode::Node {
                value: value.clone(),
                left: Box::new(remove_node_aux(left, item)),
                right: right.clone(),
              }
            }
          }
          (true, TreeNode::Empty, TreeNode::Empty) => TreeNode::Empty,
          (true, TreeNode::Empty, _) => *right.clone(),
          (true, _, TreeNode::Empty) => *left.clone(),
          (true, _, _) => {
            let rightmost = in_order_sucessor(right, item);
            TreeNode::Node {
              value: rightmost.clone(),
              left: left.to_owned(),
              right: Box::new(remove_node_aux(right, rightmost.clone())),
            }
          }
        }
      }
    }
  }

  pub fn search_node_aux<T>(node: &TreeNode<T>, item: T) -> Option<T>
  where
    T: PartialEq + PartialOrd,
  {
    let _ = item;
    match node {
      TreeNode::Empty => None,
      TreeNode::Node { value, left, right } => {
        if value == &item {
          Some(item)
        } else if value > &item {
          search_node_aux(right, item)
        } else {
          search_node_aux(left, item)
        }
      }
    }
  }

  pub fn len_node_aux<T>(node: &TreeNode<T>, acc: i32) -> i32 {
    match node {
      TreeNode::Empty => acc,
      TreeNode::Node {
        value: _,
        left,
        right,
      } => len_node_aux(right, len_node_aux(left, acc + 1)),
    }
  }

  pub fn height_node_aux<T>(node: &TreeNode<T>) -> i32 {
    match node {
      TreeNode::Empty => 0,
      TreeNode::Node {
        value: _,
        left,
        right,
      } => std::cmp::max(height_node_aux(left), height_node_aux(right)) + 1,
    }
  }

  pub fn any_node_aux<T>(node: &TreeNode<T>, f: fn(&T) -> bool) -> bool {
    match node {
      TreeNode::Empty => false,
      TreeNode::Node { value, left, right } => {
        if f(value) {
          true
        } else {
          any_node_aux(left, f) || any_node_aux(right, f)
        }
      }
    }
  }

  pub fn all_node_aux<T>(node: &TreeNode<T>, f: fn(&T) -> bool) -> bool {
    match node {
      TreeNode::Empty => true,
      TreeNode::Node { value, left, right } => {
        if f(value) {
          all_node_aux(left, f) && all_node_aux(right, f)
        } else {
          false
        }
      }
    }
  }

  pub fn find_node_aux<'a, T>(node: &'a TreeNode<T>, f: fn(&T) -> bool) -> Option<&'a T> {
    match node {
      TreeNode::Empty => None,
      TreeNode::Node { value, left, right } => {
        if f(value) {
          Some(value)
        } else {
          match find_node_aux(left, f) {
            Some(value) => Some(value),
            None => match find_node_aux(right, f) {
              Some(value) => Some(value),
              None => None,
            },
          }
        }
      }
    }
  }

  pub fn map_node_aux<T, U>(node: &TreeNode<T>, f: fn(&T) -> U) -> TreeNode<U> {
    match node {
      TreeNode::Empty => TreeNode::Empty,
      TreeNode::Node { value, left, right } => TreeNode::Node {
        value: f(value),
        left: Box::new(map_node_aux(left, f)),
        right: Box::new(map_node_aux(right, f)),
      },
    }
  }

  pub fn reduce_node_aux<T, U>(node: &TreeNode<T>, f: fn(&T, U) -> U, acc: U) -> U {
    match node {
      TreeNode::Empty => acc,
      TreeNode::Node { value, left, right } => {
        reduce_node_aux(right, f, f(value, reduce_node_aux(left, f, acc)))
      }
    }
  }
}

impl<T> BinarySearchTree<T>
where
  T: PartialEq + PartialOrd + Clone + Copy,
{
  pub fn new() -> Self {
    Self {
      root: TreeNode::Empty,
    }
  }

  pub fn is_empty(tree: &Self) -> bool {
    matches!(tree.root, TreeNode::Empty)
  }

  pub fn insert(tree: &Self, item: T) -> Self {
    Self {
      root: private::insert_node_aux(&tree.root, item),
    }
  }

  pub fn remove(tree: &Self, item: T) -> Self {
    Self {
      root: private::remove_node_aux(&tree.root, item),
    }
  }

  pub fn search(tree: &Self, item: T) -> Option<T> {
    private::search_node_aux(&tree.root, item)
  }

  pub fn len(tree: &Self) -> i32 {
    private::len_node_aux(&tree.root, 0)
  }

  pub fn height(tree: &Self) -> i32 {
    private::height_node_aux(&tree.root)
  }

  pub fn any(tree: &Self, f: fn(&T) -> bool) -> bool {
    private::any_node_aux(&tree.root, f)
  }

  pub fn all(tree: &Self, f: fn(&T) -> bool) -> bool {
    private::all_node_aux(&tree.root, f)
  }

  pub fn find(tree: &Self, f: fn(&T) -> bool) -> Option<&T> {
    private::find_node_aux(&tree.root, f)
  }

  pub fn map<U>(tree: &Self, f: fn(&T) -> U) -> BinarySearchTree<U> {
    BinarySearchTree::<U> {
      root: private::map_node_aux(&tree.root, f),
    }
  }

  pub fn reduce<U>(tree: &Self, f: fn(&T, U) -> U, acc: U) -> U {
    private::reduce_node_aux(&tree.root, f, acc)
  }
}

#[cfg(test)]
#[path = "./binary-search-tree_test.rs"]
mod test;
