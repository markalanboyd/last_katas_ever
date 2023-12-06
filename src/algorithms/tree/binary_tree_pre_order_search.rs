enum BinaryNode<T> {
    Leaf(T),
    Internal {
        value: T,
        left: Box<BinaryNode<T>>,
        right: Box<BinaryNode<T>>,
    },
}

fn walk<T>(curr: Option<&BinaryNode<T>>, path: &mut Vec<T>)
where
    T: Copy + Clone,
{
    if let Some(node) = curr {
        match node {
            BinaryNode::Leaf(val) => path.push(*val),
            BinaryNode::Internal { value, left, right } => {
                path.push(*value);
                walk(left.as_ref().map(|node| &**node), path); // dereference Box
                walk(right.as_ref().map(|node| &**node), path); // dereference Box
            }
        }
    }
}

pub fn binary_tree_pre_order_search<T>(head: &BinaryNode<T>) -> Vec<T>
where
    T: Copy + Clone,
{
    let mut path = Vec::new();
    walk(Some(head), &mut path);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_pre_order_search() {
        // Create a simple binary tree
        //       1
        //      / \
        //     2   3
        let tree = BinaryNode::Internal {
            value: 1,
            left: Box::new(BinaryNode::Leaf(2)),
            right: Box::new(BinaryNode::Leaf(3)),
        };

        // Perform a pre-order search
        let result = binary_tree_pre_order_search(&tree);

        // Define the expected output (pre-order: root, left, right)
        let expected = vec![1, 2, 3];

        // Assert that the result matches the expected output
        assert_eq!(result, expected);
    }
}
