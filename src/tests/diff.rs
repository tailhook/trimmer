//! This crate provides utilities around [least common subsequences][wiki]. From a least common
//! subsequences table, you can also calculate diffs (see `LcsTable::diff`).
//!
//! Usage of this crate is centered around `LcsTable`, so most interesting documentation can be
//! found there.
//!
//! [wiki]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem

use std::fmt::{Write, Debug};
use std::cmp;


#[derive(Debug)]
pub struct LcsTable<'a, T: 'a> {
    lengths: Vec<Vec<i64>>,

    a: &'a [T],
    b: &'a [T]
}

#[derive(Debug, PartialEq)]
pub enum DiffComponent<T> {
    Insertion(T),
    Unchanged(T, T),
    Deletion(T)
}

/// Finding longest common subsequences ("LCS") between two sequences requires constructing a *n x
/// m* table (where the two sequences are of lengths *n* and *m*). This is expensive to construct
/// and there's a lot of stuff you can calculate using it, so `LcsTable` holds onto this data.
impl<'a, T> LcsTable<'a, T> where T: PartialEq {
    /// Constructs a LcsTable for matching between two sequences `a` and `b`.
    pub fn new(a: &'a [T], b: &'a [T]) -> LcsTable<'a, T> {
        let mut lengths = vec![vec![0; b.len() + 1]; a.len() + 1];

        for i in 0..a.len() {
            for j in 0..b.len() {
                lengths[i + 1][j + 1] = if a[i] == b[j] {
                    1 + lengths[i][j]
                } else {
                    cmp::max(lengths[i + 1][j], lengths[i][j + 1])
                }
            }
        }

        LcsTable { lengths: lengths, a: a, b: b }
    }

    /// Computes a diff from `a` to `b`.
    ///
    /// # Example
    ///
    /// ```
    /// use lcs::{DiffComponent, LcsTable};
    ///
    /// let a: Vec<_> = "axb".chars().collect();
    /// let b: Vec<_> = "abc".chars().collect();
    ///
    /// let table = LcsTable::new(&a, &b);
    /// let diff = table.diff();
    /// assert_eq!(diff, vec![
    ///     DiffComponent::Unchanged(&'a', &'a'),
    ///     DiffComponent::Deletion(&'x'),
    ///     DiffComponent::Unchanged(&'b', &'b'),
    ///     DiffComponent::Insertion(&'c')
    /// ]);
    /// ```
    pub fn diff(&self) -> Vec<DiffComponent<&T>> {
        self.compute_diff(self.a.len(), self.b.len())
    }

    fn compute_diff(&self, i: usize, j: usize) -> Vec<DiffComponent<&T>> {
        if i == 0 && j == 0 {
            return vec![];
        }

        enum DiffType {
            Insertion,
            Unchanged,
            Deletion
        }

        let diff_type = if i == 0 {
            DiffType::Insertion
        } else if j == 0 {
            DiffType::Deletion
        } else if self.a[i - 1] == self.b[j - 1] {
            DiffType::Unchanged
        } else if self.lengths[i][j - 1] > self.lengths[i - 1][j] {
            DiffType::Insertion
        } else {
            DiffType::Deletion
        };

        let (to_add, mut rest_diff) = match diff_type {
            DiffType::Insertion => {
                (DiffComponent::Insertion(&self.b[j - 1]),
                    self.compute_diff(i, j - 1))
            },

            DiffType::Unchanged => {
                (DiffComponent::Unchanged(&self.a[i - 1], &self.b[j - 1]),
                    self.compute_diff(i - 1, j - 1))
            },

            DiffType::Deletion => {
                (DiffComponent::Deletion(&self.a[i - 1]),
                    self.compute_diff(i - 1, j))
            }
        };

        rest_diff.push(to_add);
        rest_diff
    }
}


pub fn assert_eq<T: PartialEq + Debug>(x: Vec<T>, y: Vec<T>) {
    use self::DiffComponent::{Insertion, Unchanged, Deletion};

    if x != y {
        let mut buf = String::with_capacity(1024);
        for item in LcsTable::new(&x, &y).diff() {
            match item {
                Deletion(val) => {
                    writeln!(&mut buf, "  - {:?}", val).unwrap();
                }
                Unchanged(val, _) => {
                    writeln!(&mut buf, "    {:?}", val).unwrap();
                }
                Insertion(val) => {
                    writeln!(&mut buf, "  + {:?}", val).unwrap();
                }
            }
        }
        panic!("Sequences differ:\n{}", buf);
    }
}

#[test]
fn test_lcs_table() {
    // Example taken from:
    //
    // https://en.wikipedia.org/wiki/Longest_common_subsequence_problem#Worked_example

    let a: Vec<_> = "gac".chars().collect();
    let b: Vec<_> = "agcat".chars().collect();

    let actual_lengths = LcsTable::new(&a, &b).lengths;
    let expected_lengths = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 1],
        vec![0, 1, 1, 1, 2, 2],
        vec![0, 1, 1, 2, 2, 2]
    ];

    assert_eq!(expected_lengths, actual_lengths);
}

#[test]
fn test_diff() {
    use self::DiffComponent::*;

    let a: Vec<_> = "axb".chars().collect();
    let b: Vec<_> = "abc".chars().collect();

    let table = LcsTable::new(&a, &b);
    let diff = table.diff();
    assert_eq!(diff, vec![
        Unchanged(&'a', &'a'),
        Deletion(&'x'),
        Unchanged(&'b', &'b'),
        Insertion(&'c')
    ]);
}
