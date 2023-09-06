use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Groups elements of a collection based on a key function and counts them.
pub fn group_by_count<'a, T: 'a, U: Eq + Hash>(
    objects: impl IntoIterator<Item = &'a T>,
    key: impl Fn(&'a T) -> U,
) -> HashMap<U, usize> {
    let mut result: HashMap<U, usize> = HashMap::new();
    for object in objects.into_iter() {
        let key = key(object);
        if let Some(amount) = result.get_mut(&key) {
            *amount = amount.saturating_add(1usize);
        } else {
            result.insert(key, 1usize);
        }
    }
    result
}

/// Groups elements of a collection based on a key function.
pub fn group_by<'a, T, U: Eq + Hash>(
    objects: impl IntoIterator<Item = &'a T>,
    key: impl Fn(&'a T) -> U,
) -> HashMap<U, Vec<&'a T>> {
    let mut result: HashMap<U, Vec<&'a T>> = HashMap::new();
    for object in objects.into_iter() {
        let key = key(object);
        if let Some(vec) = result.get_mut(&key) {
            vec.push(object);
        } else {
            result.insert(key, vec![object]);
        }
    }
    result
}

/// Counts unique elements in an iterable.
pub fn count_unique<T: Eq + Hash>(objects: impl IntoIterator<Item = T>) -> usize {
    objects.into_iter().collect::<HashSet<_>>().len()
}

/// Compares two objects applying a key function which returns a reference.
fn compare_reference_keys<T, R>(a: &T, b: &T, key: &mut impl FnMut(&T) -> &R) -> std::cmp::Ordering
where
    for<'a> &'a R: Ord,
{
    let ak = key(a);
    let bk = key(b);
    ak.cmp(&bk)
}

/// Sorts a vector by a key function that returns a reference.
/// Useful when sorting strings.
/// See https://stackoverflow.com/a/47126516
pub fn sort_by_reference_key<T, R>(v: &mut [T], mut key: impl FnMut(&T) -> &R)
where
    for<'a> &'a R: Ord,
{
    v.sort_by(|a, b| compare_reference_keys(a, b, &mut key));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::sort_by_reference_key;
    use crate::util::iter::{count_unique, group_by, group_by_count};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Animal {
        name: String,
    }

    impl Animal {
        fn new(name: impl Into<String>) -> Self {
            Self { name: name.into() }
        }
    }

    #[test]
    fn test_group_by_correctly_groups_up_numbers_by_mod_4() {
        let key = |n| n % 4;
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let grouped_up = group_by(values.iter(), key);

        assert_eq!(
            vec![
                (0, vec![4, 8, 12, 16]),
                (1, vec![1, 5, 9, 13]),
                (2, vec![2, 6, 10, 14]),
                (3, vec![3, 7, 11, 15])
            ]
            .into_iter()
            .collect::<HashSet<_>>(),
            grouped_up
                .into_iter()
                .map(|(i, v)| (i, v.into_iter().map(|&n| n).collect()))
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_group_by_count_correctly_counts_numbers_grouped_by_mod_4() {
        let key = |n| n % 4;
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let grouped_up = group_by_count(values.iter(), key);
        assert_eq!(
            vec![(0, 4), (1, 5), (2, 4), (3, 4)]
                .into_iter()
                .collect::<HashSet<_>>(),
            grouped_up.into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_count_unique_counts_numbers_correctly() {
        let values = [1, 1, 2, 2, 2, 2, 2, 3, 4, 6, 10];

        assert_eq!(6, count_unique(values));
    }

    #[test]
    fn test_sort_by_reference_key_works_with_string_keys() {
        let mut animals = vec![
            Animal::new("ccc"),
            Animal::new("ddd"),
            Animal::new("aaa"),
            Animal::new("eee"),
            Animal::new("bbb"),
        ];
        sort_by_reference_key(&mut animals, |animal| &animal.name);
        assert_eq!(
            animals,
            vec![
                Animal::new("aaa"),
                Animal::new("bbb"),
                Animal::new("ccc"),
                Animal::new("ddd"),
                Animal::new("eee")
            ]
        );
    }
}
