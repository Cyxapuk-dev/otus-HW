fn main() {}
pub fn merge_sort(input: &[u64]) -> Vec<u64> {
    if input.len() <= 1 {
        return input.to_vec();
    }

    let mid = input.len() / 2;
    let left = merge_sort(&input[..mid]);
    let right = merge_sort(&input[mid..]);

    merge(&left, &right)
}

fn merge(left: &[u64], right: &[u64]) -> Vec<u64> {
    let mut l_iter = left.iter().peekable();
    let mut r_iter = right.iter().peekable();

    std::iter::from_fn(move || match (l_iter.peek(), r_iter.peek()) {
        (Some(&&l_val), Some(&&r_val)) => {
            if l_val <= r_val {
                l_iter.next().copied()
            } else {
                r_iter.next().copied()
            }
        }
        (Some(_), None) => l_iter.next().copied(),
        (None, Some(_)) => r_iter.next().copied(),
        (None, None) => None,
    })
    .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input: Vec<u64> = vec![];
        let sorted = merge_sort(&input);
        assert_eq!(sorted, vec![]);
    }

    #[test]
    fn test_single() {
        let input = vec![42];
        let sorted = merge_sort(&input);
        assert_eq!(sorted, vec![42]);
    }

    #[test]
    fn test_sorted() {
        let input = vec![1, 2, 3, 4, 5];
        let sorted = merge_sort(&input);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse() {
        let input = vec![5, 4, 3, 2, 1];
        let sorted = merge_sort(&input);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random() {
        let input = vec![4, 1, 3, 2, 5];
        let sorted = merge_sort(&input);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let input = vec![2, 3, 2, 1, 3, 1];
        let sorted = merge_sort(&input);
        assert_eq!(sorted, vec![1, 1, 2, 2, 3, 3]);
    }
}
