//! Collection and iterator utilities.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Chunk a vector into smaller vectors of specified size.
pub fn chunk<T: Clone>(items: Vec<T>, size: usize) -> Vec<Vec<T>> {
    items.chunks(size).map(|chunk| chunk.to_vec()).collect()
}

/// Get unique items from a vector.
pub fn unique<T: Clone + Hash + Eq>(items: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    items
        .into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

/// Group items by a key function.
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_default().push(item);
    }
    groups
}

/// Partition items into two vectors based on predicate.
pub fn partition<T, F>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    let mut pass = Vec::new();
    let mut fail = Vec::new();
    for item in items {
        if predicate(&item) {
            pass.push(item);
        } else {
            fail.push(item);
        }
    }
    (pass, fail)
}

/// Flatten nested vectors.
pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

/// Zip two vectors into pairs.
pub fn zip<T, U>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    a.into_iter().zip(b).collect()
}

/// Find first matching item.
pub fn find<T, F>(items: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().find(|item| predicate(item))
}

/// Find first matching item and return its index.
pub fn find_index<T, F>(items: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    items.iter().position(|item| predicate(item))
}

/// Check if any item matches predicate.
pub fn any<T, F>(items: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    items.iter().any(predicate)
}

/// Check if all items match predicate.
pub fn all<T, F>(items: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    items.iter().all(predicate)
}

/// Sum numeric values.
pub fn sum<T, N, F>(items: &[T], value_fn: F) -> N
where
    N: std::iter::Sum,
    F: Fn(&T) -> N,
{
    items.iter().map(value_fn).sum()
}

/// Count items matching predicate.
pub fn count<T, F>(items: &[T], predicate: F) -> usize
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|item| predicate(item)).count()
}

/// Get first n items.
pub fn take<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().take(n).cloned().collect()
}

/// Skip first n items.
pub fn skip<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().skip(n).cloned().collect()
}

/// Reverse a vector.
pub fn reverse<T: Clone>(items: &[T]) -> Vec<T> {
    items.iter().rev().cloned().collect()
}

/// Interleave two vectors.
pub fn interleave<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();
    loop {
        match (a_iter.next(), b_iter.next()) {
            (Some(x), Some(y)) => {
                result.push(x);
                result.push(y);
            }
            (Some(x), None) => result.push(x),
            (None, Some(y)) => result.push(y),
            (None, None) => break,
        }
    }
    result
}

/// Create a frequency map.
pub fn frequencies<T: Clone + Hash + Eq>(items: &[T]) -> HashMap<T, usize> {
    let mut freq = HashMap::new();
    for item in items {
        *freq.entry(item.clone()).or_insert(0) += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk() {
        let items = vec![1, 2, 3, 4, 5];
        let chunks = chunk(items, 2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_unique() {
        let items = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(unique(items), vec![1, 2, 3]);
    }
}
