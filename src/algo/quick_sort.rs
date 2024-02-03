// FIXME: attempt to substract with overflow
fn quick_sort<T: Ord>(array: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let pivotIdx = partition(array, low, high);
    quick_sort(array, low, pivotIdx - 1);
    quick_sort(array, pivotIdx + 1, high);
}

fn partition<T: Ord>(array: &mut [T], low: usize, high: usize) -> usize {
    let mut i = low - 1;

    for idx in low..high {
        if array[idx] <= array[high] {
            i += 1;
            array.swap(i, idx);
        }
    }

    i += 1;
    array.swap(i, high);
    i
}

fn sort<T: Ord>(array: &mut [T]) {
    quick_sort(array, 0, array.len() - 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut array = vec![10, 5, 2, 3, 1, 8, 9, 7, 6, 4];
        sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
