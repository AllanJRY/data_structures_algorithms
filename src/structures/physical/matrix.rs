#![allow(dead_code)]

use std::fmt::{Display, Write};

/// For a matrix to be considered a diagonal matrix, all his values except those
/// on his diagonal should be null/empty/zero.
/// This is a diagonal:    This is not:
/// [ 7 0 0 0 0 ]          [ 7 2 0 0 0 ]
/// [ 0 3 0 0 0 ]          [ 0 3 0 0 9 ]
/// [ 0 0 5 0 0 ]          [ 0 0 5 0 0 ]
/// [ 0 0 0 7 0 ]          [ 0 9 0 7 0 ]
/// [ 0 0 0 0 3 ]          [ 0 4 1 0 3 ]
/// We can translate this caracteristic by saying that **M[i,j] = 0 IF i != j**
/// In code, using a two dimensional array is not efficient, it is taking a lot
/// of space that will not be used. So we may repredent a diagonal matix using
/// a one dimensional array with the values : [ 7 3 5 7 3 ]. then we can access
/// a value with i and j, but they have to be the same values. Example:
/// at coordinate (1, 1), because we consider coordinate starting from 1 we get
/// back 7.
pub struct DiagonalMatrix {
    dim: usize,
    diag: Vec<Option<i32>>,
}

impl DiagonalMatrix {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            diag: vec![None; dim],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<i32> {
        if x != y {
            return None;
        }
        *self.diag.get(x - 1).unwrap()
    }

    pub fn set(&mut self, x: usize, y: usize, val: i32) {
        if x == y {
            self.diag.get_mut(x - 1).unwrap().replace(val);
        }
    }
}

impl Display for DiagonalMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 1..=self.dim {
            for j in 1..=self.dim {
                if i == j {
                    if let Some(val) = self.get(i, j) {
                        write!(f, "{}", val).unwrap();
                    } else {
                        write!(f, "0").unwrap();
                    }
                } else {
                    write!(f, "0").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
