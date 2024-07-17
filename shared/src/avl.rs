use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
struct Entry<K: Ord, V> {
    pub key: K,
    pub value: V,
    left: Tree<K, V>,
    right: Tree<K, V>,
    height: usize
}

type Tree<K, V> = Option<Box<Entry<K, V>>>;

#[derive(Debug, PartialEq, Clone)]
struct SortedMap<K: Ord, V> {
    root: Tree<K, V>,
    size: usize
}

impl<'a, K: 'a + Ord, V> SortedMap<K, V> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        let mut current = &mut self.root;
        let mut height = 0;

        while let Some(node) = current {
            height += 1;
            match node.key.cmp(&key) {
                Ordering::Less => current = &mut node.left,
                Ordering::Greater => current = &mut node.right,
                Ordering::Equal => return false
            }
        }

        self.size += 1;
        *current = Some(Box::new(Entry {
            key,
            value,
            left: None,
            right: None,
            height
        }));

        true
    }

    pub fn iter(&'a self) -> SortedMapIter<'a, K, V> {
        SortedMapIter {
            stack: Vec::new(),
            current: &self.root,
        }
    }
}

#[derive(Debug)]
struct SortedMapIter<'a, K: Ord, V> {
    stack: Vec<&'a Entry<K, V>>,
    current: &'a Tree<K, V>,
}

impl<'a, K: Ord, V> Iterator for SortedMapIter<'a, K, V> {
    type Item = &'a Entry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match *self.current {
                None => match self.stack.pop() {
                    None => None,
                    Some(ref parent) => {
                        self.current = &parent.right;
                        Some(&parent)
                    }
                },
                Some(ref node) => {
                    if node.left.is_some() {
                        self.stack.push(&node);
                        self.current = &node.left;
                        continue;
                    } else if node.right.is_some() {
                        self.current = &node.right;
                    } else {
                        self.current = &None;
                    }
                    Some(&node)
                }
            }
        }
    }
}

impl<'a, K: Ord, V> DoubleEndedIterator for SortedMapIter<'a, K, V>  {

    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            return match *self.current {
                None => match self.stack.pop() {
                    None => None,
                    Some(ref parent) => {
                        self.current = &parent.left;
                        Some(&parent)
                    }
                },
                Some(ref node) => {
                    if node.right.is_some() {
                        self.stack.push(&node);
                        self.current = &node.right;
                        continue;
                    } else if node.left.is_some() {
                        self.current = &node.left;
                    } else {
                        self.current = &None;
                    }
                    Some(&node)
                }
            }
        }
    }
}

impl<'a, K: Ord, V> IntoIterator for SortedMap<K, V> {
    type Item = &'a Entry<K, V>;
    type IntoIter = SortedMapIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        SortedMapIter {
            stack: Vec::new(),
            current: &self.root,
        }
    }
}

// impl<K: Ord, V> FromFn<V> for SortedMap<K, V> {
//     fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: (K, V)) -> Self {
//         let mut tree = Self::new();
//         for (key, value) in iter {
//             tree.insert(value)
//         }
//         tree
//     }
// }

#[cfg(test)]
mod tests {
    extern crate rstest;

    use super::*;

    #[test]
    fn should_insert() {
        let mut tree: SortedMap<u8, char> = SortedMap::new();

        assert_eq!(tree.len(), 0);

        tree.insert(1, 'a');
        tree.insert(2, 'b');
        tree.insert(3, 'c');

        assert_ne!(tree.root, None);
        assert_eq!(tree.len(), 3);

        let mut inorder = tree.iter().map(|x| x.value);
        assert_eq!(inorder.next(), Some('c'));
        assert_eq!(inorder.next(), Some('b'));
        assert_eq!(inorder.next(), Some('a'));
        assert_eq!(inorder.next(), None);

        let mut preorder = tree.iter().map(|x| x.value);
        assert_eq!(preorder.next_back(), Some('a'));
        assert_eq!(preorder.next_back(), Some('b'));
        assert_eq!(preorder.next_back(), Some('c'));
        assert_eq!(preorder.next_back(), None);
    }
}
