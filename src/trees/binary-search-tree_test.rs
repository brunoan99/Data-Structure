use super::*;

mod setup {
  use super::*;

  pub type BinaryTreeT = BinarySearchTree<i32>;

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
    let op = BinarySearchTree::<i32>::new();
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
    let op = BinarySearchTree::is_empty(&tree);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::is_empty(&tree);
    assert_eq!(op, false)
  }
}

#[cfg(test)]
mod insert {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinarySearchTree::insert(&tree, 0);
    let expected = BinarySearchTree {
      root: setup::node(0, TreeNode::Empty, TreeNode::Empty),
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::insert(&tree, 7);
    let expected = BinarySearchTree {
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
    let op = BinarySearchTree::remove(&tree, 0);
    let expected = BinarySearchTree {
      root: TreeNode::Empty,
    };
    assert_eq!(op, expected)
  }

  #[test]
  fn to_filled_no_children() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::remove(&tree, 6);
    let expected = BinarySearchTree {
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
    let tree = BinarySearchTree {
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
    let op = BinarySearchTree::remove(&tree, 5);
    let expected = BinarySearchTree {
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
    let tree = BinarySearchTree {
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
    let op = BinarySearchTree::remove(&tree, 6);
    let expected = BinarySearchTree {
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
    let op = BinarySearchTree::remove(&tree, 5);
    let expected = BinarySearchTree {
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
    let op = BinarySearchTree::len(&tree);
    assert_eq!(op, 0);
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::len(&tree);
    assert_eq!(op, 7);
  }
}

#[cfg(test)]
mod height {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinarySearchTree::height(&tree);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::height(&tree);
    assert_eq!(op, 3)
  }
}

#[cfg(test)]
mod any {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinarySearchTree::any(&tree, |_| true);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_without_true_return() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::any(&tree, |item| item < &0);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::any(&tree, |item| item == &0);
    assert_eq!(op, true)
  }
}

#[cfg(test)]
mod all {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinarySearchTree::all(&tree, |_| false);
    assert_eq!(op, true)
  }

  #[test]
  fn to_filled_with_false_return() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::all(&tree, |item| item < &6);
    assert_eq!(op, false)
  }

  #[test]
  fn to_filled_with_only_true() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::all(&tree, |item| item > &-1);
    assert_eq!(op, true)
  }
}

#[cfg(test)]
mod find {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinarySearchTree::find(&tree, |_| true);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_without_match() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::find(&tree, |_| false);
    assert_eq!(op, None)
  }

  #[test]
  fn to_filled_with_match() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::find(&tree, |item| item % 4 == 0);
    assert_eq!(op, Some(&0))
  }

  #[test]
  fn to_filled_should_match_in_order() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::find(&tree, |item| item % 2 == 0 && item > &1);
    assert_eq!(op, Some(&2))
  }
}

#[cfg(test)]
mod map {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinarySearchTree::map(&tree, |item| item + 5);
    assert_eq!(op, tree)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::map(&tree, |item| item + 5);
    let expected = BinarySearchTree {
      root: setup::node(
        8,
        setup::node(
          6,
          setup::node(5, TreeNode::Empty, TreeNode::Empty),
          setup::node(7, TreeNode::Empty, TreeNode::Empty),
        ),
        setup::node(
          10,
          setup::node(9, TreeNode::Empty, TreeNode::Empty),
          setup::node(11, TreeNode::Empty, TreeNode::Empty),
        ),
      ),
    };
    assert_eq!(op, expected)
  }
}

#[cfg(test)]
mod reduce {
  use super::*;

  #[test]
  fn to_empty() {
    let tree = setup::binary_tree_empty();
    let op = BinarySearchTree::reduce(&tree, |item, acc| item + acc, 0);
    assert_eq!(op, 0)
  }

  #[test]
  fn to_filled() {
    let tree = setup::binary_tree_filled();
    let op = BinarySearchTree::reduce(&tree, |item, acc| item + acc, 0);
    assert_eq!(op, 21)
  }
}
