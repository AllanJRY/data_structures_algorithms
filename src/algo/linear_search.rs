#![allow(dead_code)]

/// Loop through an array of comparable datas to find the value given in the
/// arguments of the function. The return value could have been the index or a
/// boolean but because it is for learning purpose only on it will not be used
/// in the context of an application, the value is directly return.
/// The linear search is one of the simple algorithm, but it's time complexity
/// is O(n), which make it not really suitable for large arrays.
///
/// # Arguments
///
/// * `haystack` - The array in which to fo find the value.
/// * `needle` - The value to find.
///
/// # Examples
///
/// ```
/// let result = linear_search(&[12, 45, 3, 2, 7, 68, 493, 90], 3);
/// assert_eq!(Some(&3), result);
/// ```
#[allow(clippy::manual_find)]
pub fn linear_search<T: PartialEq>(haystack: &[T], needle: T) -> Option<&T> {
    for val in haystack {
        if *val == needle {
            return Some(val);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_search_i32() {
        assert_eq!(Some(&3), linear_search(&[12, 45, 3, 2, 7, 68, 493, 90], 3));
    }

    #[test]
    fn test_linear_search_i32_not_found() {
        assert_eq!(None, linear_search(&[12, 45, 3, 2, 7, 68, 493, 90], 77));
    }

    #[test]
    fn test_linear_search_string() {
        assert_eq!(
            Some(&"Ellie"),
            linear_search(&["Emile", "Julie", "Ellie", "Joel", "Frank"], "Ellie")
        );
    }

    #[test]
    fn test_linear_search_string_not_found() {
        assert_eq!(
            None,
            linear_search(&["Emile", "Julie", "Ellie", "Joel", "Frank"], "Allan")
        );
    }
}
