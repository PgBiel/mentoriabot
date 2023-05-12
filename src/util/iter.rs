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
        let key = key(&object);
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
        let key = key(&object);
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::util::iter::{count_unique, group_by, group_by_count};

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
}
