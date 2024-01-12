use std::{ops::Deref, usize};

/// The binary search offers an effecient way to find a value in an ORDERED
/// array. The way it does this, it's by starting at the middle of the array,
/// check the value, if it's equal it returns the middle index, otherwise, if
/// the middle value is less: do the same process but splitting the left half
/// at the middle. Or on the right half if the value is greater. this process
/// is made until we found the value or there is no other entries to check.
/// This process offers a O(log(n)) time complexity, making it a great choice
/// for large arrays compared to a linear search.
/// But remember: the given array should always be ordered !
///
/// # Arguments
///
/// * `ordered_haystack` - The ordered array in which to find the value.
/// * `needle` - The value to find.
///
/// # Examples
///
/// ```
/// let result = binary_search(&[1, 12, 23, 56, 59, 63, 77, 95], 77);
/// assert_eq!(Some(MatchingIdx(6)), result);
/// ```
pub fn binary_search(ordered_haystack: &[i32], needle: i32) -> Option<MatchingIdx> {
    let mut lower_idx = 0;
    let mut higher_idx = ordered_haystack.len() as i32;

    while lower_idx < higher_idx {
        // We cannot do high / 2, because we need to handle the offset if it's not
        // the first search attempt
        let mid_idx = lower_idx + (higher_idx - lower_idx) / 2;
        let mid = ordered_haystack[mid_idx as usize];
        match mid.cmp(&needle) {
            std::cmp::Ordering::Less => lower_idx = mid_idx + 1,
            std::cmp::Ordering::Equal => return Some(MatchingIdx(mid_idx)),
            std::cmp::Ordering::Greater => higher_idx = mid_idx,
        }
    }

    None
}

/// This is modeling the result of the binary search, which is a matching index
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchingIdx(i32);

impl Deref for MatchingIdx {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search_i32_some() {
        assert_eq!(
            Some(MatchingIdx(6)),
            binary_search(&[1, 12, 23, 56, 59, 63, 77, 95], 77)
        );
    }

    #[test]
    fn test_binary_search_i32_none() {
        assert_eq!(None, binary_search(&[1, 12, 23, 56, 59, 63, 77, 95], 100));
    }

    #[test]
    fn test_binary_search_unordered_i32_none() {
        assert_eq!(None, binary_search(&[1, 23, 12, 59, 63, 56, 77, 95], 56));
    }
}
