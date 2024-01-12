#![allow(dead_code)]
use std::usize;

/// Based on an interview of ThePrimeng.
/// It was given this statement:
///
/// _Given two crystal balls that wil break if dropped from high enough distance,
/// determine the exact spot in which it will break in the most optimized way_
///
/// So to find the correct spot faster, linear search wasn't ideal and binary
/// search neither, be cause of the fact that we are scanning boolean and
/// there is a lot of duplicate values (\[false false, true, true\]),
/// back in pevious sport as to be made frequently.
/// So he decide kind of mix between linear abd binary search
/// were he jump but not to half the array, but square root of the length of
/// the array. If he found the value he is looking for he go back a bit and do
/// a linear search on juste the amount of the jump it does before.
/// So the time complexity of this algorithm is O(Sqrt(n)).
///
/// # Arguments
///
/// * `breaks` - the state of the crystal balls during time.
pub fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    let jmp_amount = (breaks.len() as f32).sqrt().floor() as usize;

    let mut i = jmp_amount;
    while i < breaks.len() {
        if breaks[i] {
            break;
        }

        i += jmp_amount;
    }

    i -= jmp_amount;
    for _ in 0..jmp_amount {
        if i >= breaks.len() {
            break;
        }

        if breaks[i] {
            return Some(i);
        }

        i += 1;
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_crystal_balls_break_found() {
        assert_eq!(
            Some(7),
            two_crystal_balls(&[false, false, false, false, false, false, false, true, true])
        );
    }

    #[test]
    fn two_crystal_balls_break_no_break() {
        assert_eq!(
            None,
            two_crystal_balls(&[false, false, false, false, false, false, false])
        );
    }
}
