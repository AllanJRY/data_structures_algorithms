#![allow(dead_code)]
//! TODO: doc: Time complexity often O(n) and compared to a loop, rescursion
//! mays have a space complexity of O(n), due to the call stack.

/// It is call tail recursion because the recursive calls are at the end of
/// the function, and nothing happens after that. A tail recursion might be
/// easily convertible to a loop.
fn recursive_tail_push(n: u32, vals: &mut Vec<u32>) {
    if n > 0 {
        vals.push(n);
        recursive_tail_push(n - 1, vals);
    }
}

/// It is call head recursion because the recursive calls are the start of
/// the function, and nothing happens before that. Compared to tail recursion,
/// a head recursion may not be easily convertible to a loop, it may imply
/// some changes in the fonction body logic.
fn recursive_head_push(n: u32, vals: &mut Vec<u32>) {
    if n > 0 {
        recursive_head_push(n - 1, vals);
        vals.push(n);
    }
}

/// It is call tree recursion because a recursive call is made more than one
/// time. The big disadvantage in this exemple implementation, is that
/// the traversal time complexity is O(n^2) (2 because, 2 recursive calls happen).
/// For traversal, the space complexity, it is O(n), so no difference with tail
/// and head recursion.
fn recursive_tree_push(n: u32, vals: &mut Vec<u32>) {
    if n > 0 {
        vals.push(n);
        recursive_tree_push(n - 1, vals);
        recursive_tree_push(n - 1, vals);
    }
}

/// This is an indirect recursion, because during the call of a 'b' fonction,
/// this function will be called again. Here the time complexity is O(log(n))
/// due to function 'b' which divide n by 2.
fn recursive_indirect_a_push(n: u32, vals: &mut Vec<u32>) {
    if n > 0 {
        vals.push(n);
        // Here happens the indirect call, because the 'b' function is calling
        // this function in its logic.
        recursive_indirect_b_push(n - 1, vals);
    }
}

/// This is an indirect recursion, because during the call of a 'a' fonction,
/// this function will be called again. Here the time complexity is O(log(n))
/// due to this function which divide n by 2.
fn recursive_indirect_b_push(n: u32, vals: &mut Vec<u32>) {
    if n > 1 {
        vals.push(n);
        // Here happens the indirect call, because the 'a' function is calling
        // this function in its logic.
        recursive_indirect_a_push(n / 2, vals);
    }
}

/// This is a nested recursion, due to tha fact that in this function body
/// it is calling itself inside an another call of itself:
/// `nested_recursion(nested_recursion(_))`.
/// The time complexity is in the better case O(1) and in the worse case O(n),
/// same for space complexity.
fn nested_recursion(n: u32) -> u32 {
    if n > 100 {
        n - 10
    } else {
        // Here happens the nested recursion.
        nested_recursion(nested_recursion(n + 11))
    }
}

/// Sums the first n natural numbers using recursion. In term of time
/// complexity, it is O(n) same for space complexity.
/// Important note: a formula exists to do this and is better  O(1):
/// _(n * (n + 1)) / 2_ . But this function has been implemented for learning
/// purposes.
fn sum_of_first_natural_numbers(n: u32) -> u32 {
    if n > 0 {
        sum_of_first_natural_numbers(n - 1) + n
    } else {
        0
    }
}

// Calculate the factorial of n using recursion.
// Time complexity is O(n).
fn factorial_of_n(n: u32) -> u32 {
    if n > 1 {
        factorial_of_n(n - 1) * n
    } else {
        1
    }
}

/// Calculate the power of `n` using a given number of recursion.
/// Here the time complexity depends on the value of `times`, so it is O(n),
/// same for the space complexity.
/// Note that this is a naive implementation so it is not efficient. A better
/// logic using recursion could be :
/// - if `times` is even : `power_recursion(n * n, times / 2)`
/// - if `times` is odd : `power_recursion(n * n, (times - 1) / 2) * n`
///
/// This logic should divide the processing times and have a time complexity
/// of O(log(n)) because time is divide by 2.
fn power_recursion(n: u32, times: u32) -> u32 {
    if times == 0 {
        0
    } else if times > 1 {
        power_recursion(n, times - 1) * n
    } else {
        n
    }
}

/// Calculate the taylor series using recursion. this implementation has a time
/// complexity of O(n) and a space complexity of O(n).
fn taylor_series_recursion(n: u32, times: u32) -> f32 {
    if times == 0 {
        1.
    } else {
        let power = n.pow(times);
        let factorial = (1..=times).product::<u32>();
        taylor_series_recursion(n, times - 1) + power as f32 / factorial as f32
    }
}

/// Fibonacci implementation using recursion. This is an excessive recursion
/// because this implementation doesn't use memoization, so it is consuming
/// a lot so It's time complexity is O(2^n), with memoization it could became
/// just O(n).
fn fibonacci_series_recursion(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci_series_recursion(n - 1) + fibonacci_series_recursion(n - 2)
    }
}

/// Combination formula implemented using recursion. Recall: this formula
/// is used to know the number of way we can select with subset.
/// Exemple, with the set: `A B C D E F G`, we can have,`A`, `A B`, `A B C`
/// etc. The permutation are excluded, so `A B C` and `A C B` are the same
/// subset.
// TODO: better var names
fn combination_formula_recursion(n: u32, r: u32) -> u32 {
    if r == 0 || n == r {
        1
    } else {
        combination_formula_recursion(n - 1, r - 1) + combination_formula_recursion(n - 1, r)
    }
}

// TODO: implement Tower of Hanoi from Chap.76, it may involve some structure
// definition, so it will not be recursion only.

// TODO: Not exhaustive tests yet.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_recursive_tail_push() {
        let mut vals = Vec::with_capacity(5);
        recursive_tail_push(5, &mut vals);
        assert_eq!(vec![5, 4, 3, 2, 1], vals);
    }

    #[test]
    fn test_recursive_head_push() {
        let mut vals = Vec::with_capacity(5);
        recursive_head_push(5, &mut vals);
        assert_eq!(vec![1, 2, 3, 4, 5], vals);
    }

    #[test]
    fn test_recursive_tree_push() {
        let mut vals = Vec::with_capacity(7);
        recursive_tree_push(3, &mut vals);
        assert_eq!(vec![3, 2, 1, 1, 2, 1, 1], vals);
    }

    #[test]
    fn test_recursive_indirect_a_push() {
        let mut vals = Vec::with_capacity(7);
        recursive_indirect_a_push(20, &mut vals);
        assert_eq!(vec![20, 19, 9, 8, 4, 3, 1], vals);
    }

    #[test]
    fn test_recursive_indirect_b_push() {
        let mut vals = Vec::with_capacity(6);
        recursive_indirect_b_push(20, &mut vals);
        assert_eq!(vec![20, 10, 9, 4, 3, 1], vals);
    }

    #[test]
    fn test_nested_recursion() {
        assert_eq!(91, nested_recursion(95));
    }

    #[test]
    fn test_sum_of_first_natural_numbers() {
        assert_eq!(1 + 2 + 3 + 4 + 5, sum_of_first_natural_numbers(5));
    }

    #[test]
    fn test_factorial_of_n() {
        #![allow(clippy::identity_op)]
        assert_eq!(1 * 2 * 3 * 4 * 5, factorial_of_n(5));
    }

    #[test]
    fn test_power_recursion() {
        assert_eq!(2 * 2 * 2, power_recursion(2, 3));
    }

    #[test]
    fn test_power_recursion_0() {
        assert_eq!(0, power_recursion(2, 0));
    }

    #[test]
    fn test_power_recursion_1() {
        assert_eq!(2, power_recursion(2, 1));
    }

    #[test]
    fn test_taylor_series_recursion() {
        assert_eq!(20.079666, taylor_series_recursion(3, 10));
    }

    #[test]
    fn test_fibonacci_series_recursion() {
        assert_eq!(13, fibonacci_series_recursion(7));
    }

    #[test]
    fn test_combination_formula_recursion() {
        assert_eq!(10, combination_formula_recursion(5, 2))
    }
}
