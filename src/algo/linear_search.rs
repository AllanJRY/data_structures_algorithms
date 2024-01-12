#![allow(dead_code)]

// TODO: add documentation

#[allow(clippy::manual_find)]
pub fn linear_search<T: PartialEq>(haystack: &[T], needle: T) -> Option<&T> {
    for val in haystack {
        if *val == needle {
            return Some(val);
        }
    }

    None
}

pub fn linear_search_alt<T: PartialEq>(haystack: &[T], needle: T) -> Option<&T> {
    haystack.iter().find(|&val| *val == needle)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_search_i32() {
        assert_eq!(Some(&3), linear_search(&[12, 45, 3, 2, 7, 68, 493, 90], 3));
    }

    #[test]
    fn test_linear_search_string() {
        assert_eq!(
            Some(&"Ellie"),
            linear_search(&["Emile", "Julie", "Ellie", "Joel", "Frank"], "Ellie")
        );
    }

    #[test]
    fn test_linear_search_alt_i32() {
        assert_eq!(
            Some(&3),
            linear_search_alt(&[12, 45, 3, 2, 7, 68, 493, 90], 3)
        );
    }

    #[test]
    fn test_linear_search_alt_string() {
        assert_eq!(
            Some(&"Ellie"),
            linear_search_alt(&["Emile", "Julie", "Ellie", "Joel", "Frank"], "Ellie")
        );
    }
}
