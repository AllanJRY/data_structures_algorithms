#![allow(dead_code)]

/// Bubble sort is a simple algorithm to sort an array, but it is not a right
/// choice when working with larges datas, it as a known time complexity of O(n^2).
/// But it still a convenient way of sorting due to its simplicity of implementation.
pub fn bubble_sort<T: PartialOrd>(mut vec: Vec<T>) -> Vec<T> {
    if vec.is_empty() {
        return vec;
    }

    for i in 0..vec.len() {
        // We reduce the len by one, because we look at j + 1.
        // We also can reduce the len by i, because at every outer loop iteration,
        // all greater elements will be sorted and moved at te end of the vec,
        // so no need to check them again.
        for j in 0..vec.len() - 1 - i {
            // to be concise, vec is accessed by index but a better way would be
            // to access it by the method .get(j) which perform a check on the
            // presence of the value.
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                // The swap method swap by manipulating the pointers of the targeted
                // values but here is the logic if we do not use swap method :
                // let tmp = vec.remove(j);
                // vec.insert(j + 1, tmp);
                // Read the doc of remove and insert to see the operations they
                // imply.
            }
        }
    }

    vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bubble_sort_success() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            bubble_sort(vec![1, 6, 9, 4, 3, 2, 7, 8, 5])
        )
    }

    #[test]
    fn bubble_sort_already_sorted() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            bubble_sort(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        )
    }

    #[test]
    fn bubble_sort_empty() {
        let empty: Vec<i32> = Vec::new();
        let empty_sorted = bubble_sort(empty);
        assert!(empty_sorted.is_empty());
    }
}
