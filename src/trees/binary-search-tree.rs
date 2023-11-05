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
pub struct BinaryTree<T> {
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
            left: left.to_owned(),
            right: Box::new(insert_node_aux(&*right.clone(), item)),
          }
        } else {
          TreeNode::Node {
            value: value.clone(),
            left: Box::new(insert_node_aux(&*left.clone(), item)),
            right: right.clone().to_owned(),
          }
        }
      }
    }
  }

  pub fn in_order_sucessor<T>(node: &TreeNode<T>, acc: T) -> T
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
        TreeNode::Node { .. } => in_order_sucessor(&*left.clone(), value.clone()),
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
                left: left.to_owned(),
                right: Box::new(remove_node_aux(&*right.clone(), item)),
              }
            } else {
              TreeNode::Node {
                value: value.clone(),
                left: Box::new(remove_node_aux(&*left.clone(), item)),
                right: right.to_owned(),
              }
            }
          }
          (true, TreeNode::Empty, TreeNode::Empty) => TreeNode::Empty,
          (true, TreeNode::Empty, _) => *right.clone(),
          (true, _, TreeNode::Empty) => *left.clone(),
          (true, _, _) => {
            let rightmost = in_order_sucessor(&*right.clone(), item);
            TreeNode::Node {
              value: rightmost.clone(),
              left: left.to_owned(),
              right: Box::new(remove_node_aux(&*right.clone(), rightmost.clone())),
            }
          }
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
}

impl<T> BinaryTree<T>
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

  pub fn len(tree: &Self) -> i32 {
    private::len_node_aux(&tree.root, 0)
  }

  pub fn height(tree: &Self) -> i32 {
    private::height_node_aux(&tree.root)
  }
}

#[cfg(test)]
#[path = "./binary-search-tree_test.rs"]
mod test;
