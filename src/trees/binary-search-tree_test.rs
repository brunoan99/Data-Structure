use super::*;

mod setup {
  use super::*;

  pub type BinaryTreeT = BinaryTree<i32>;

  pub fn node<T>(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
    TreeNode::Node {
      value: value,
      left: Box::new(left),
      right: Box::new(right),
    }
  }

  pub fn binary_tree_empty() -> BinaryTreeT {
    BinaryTreeT {
      root: TreeNode::Empty,
    }
  }

  pub fn binary_tree_filled() -> BinaryTreeT {
    /*
           3
         /   \
        1     5
       / \   / \
      0   2 4   6
    */
    BinaryTreeT {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(
          5,
          setup::node(4, TreeNode::Empty, TreeNode::Empty),
          setup::node(6, TreeNode::Empty, TreeNode::Empty),
        ),
      ),
    }
  }
}

#[cfg(test)]
mod new {
  use super::*;

  #[test]
  fn single_case() {
    let op = BinaryTree::<i32>::new();
    let expected = setup::binary_tree_empty();
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod is_empty {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinaryTree::is_empty(&tree);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinaryTree::is_empty(&tree);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod insert {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinaryTree::insert(&tree, 0);
    let expected = BinaryTree {
      root: setup::node(0, TreeNode::Empty, TreeNode::Empty),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinaryTree::insert(&tree, 7);
    let expected = BinaryTree {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(
          5,
          setup::node(4, TreeNode::Empty, TreeNode::Empty),
          setup::node(
            6,
            TreeNode::Empty,
            setup::node(7, TreeNode::Empty, TreeNode::Empty),
          ),
        ),
      ),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod remove {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinaryTree::remove(&tree, 0);
    let expected = BinaryTree {
      root: TreeNode::Empty,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_no_children() {
    let tree = setup::binary_tree_filled();
    let op = BinaryTree::remove(&tree, 6);
    let expected = BinaryTree {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(
          5,
          setup::node(4, TreeNode::Empty, TreeNode::Empty),
          TreeNode::Empty,
        ),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_children_on_left() {
    let tree = BinaryTree {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(
          5,
          setup::node(4, TreeNode::Empty, TreeNode::Empty),
          TreeNode::Empty,
        ),
      ),
    };
    let op = BinaryTree::remove(&tree, 5);
    let expected = BinaryTree {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(4, TreeNode::Empty, TreeNode::Empty),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_children_on_right() {
    let tree = BinaryTree {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(
          5,
          TreeNode::Empty,
          setup::node(6, TreeNode::Empty, TreeNode::Empty),
        ),
      ),
    };
    let op = BinaryTree::remove(&tree, 6);
    let expected = BinaryTree {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(5, TreeNode::Empty, TreeNode::Empty),
      ),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_children_on_both() {
    let tree = setup::binary_tree_filled();
    let op = BinaryTree::remove(&tree, 5);
    let expected = BinaryTree {
      root: setup::node(
        3,
        setup::node(
          1,
          setup::node(0, TreeNode::Empty, TreeNode::Empty),
          setup::node(2, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(
          6,
          setup::node(4, TreeNode::Empty, TreeNode::Empty),
          TreeNode::Empty,
        ),
      ),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod len {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinaryTree::len(&tree);
    assert_eq!(op, 0);
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinaryTree::len(&tree);
    assert_eq!(op, 7);
  }
}

#[cfg(test)]
mod height {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinaryTree::height(&tree);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinaryTree::height(&tree);
    assert_eq!(op, 3)
  }
}
