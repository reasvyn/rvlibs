//! Set implementation for unique numeric values.
//!
//! This module provides a `Set` structure that maintains a collection
//! of unique values of any type that implements the `Numeric` trait.

use crate::num::Numeric;

/// A set of unique numeric values.
/// This structure stores components of any type `T` that implements `Numeric`.
/// It ensures that all elements in the set are unique based on their `PartialEq` implementation.
#[derive(Debug, Clone, Default)]
pub struct Set<T: Numeric> {
    elements: Vec<T>,
}

impl<T: Numeric> Set<T> {
    /// Creates a new empty set.
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Inserts a value into the set if it's not already present.
    /// Duplicates are silently ignored. Only the first insertion of a value is retained.
    pub fn insert(&mut self, value: T) {
        if !self.contains(&value) {
            self.elements.push(value);
        }
    }

    /// Removes a value from the set.
    /// Returns silently if the value is not in the set.
    pub fn remove(&mut self, value: &T) {
        if let Some(pos) = self.elements.iter().position(|x| x == value) {
            self.elements.swap_remove(pos);
        }
    }

    /// Returns `true` if the set contains the specified value.
    pub fn contains(&self, value: &T) -> bool {
        self.elements.contains(value)
    }

    /// Returns an iterator over the elements in the set.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    /// Returns the union of two sets as a new set.
    /// The result contains all elements that appear in either set.
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for &val in other.iter() {
            result.insert(val);
        }
        result
    }

    /// Returns the intersection of two sets as a new set.
    /// The result contains only elements that appear in both sets.
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for &val in self.iter() {
            if other.contains(&val) {
                result.insert(val);
            }
        }
        result
    }

    /// Returns the difference of two sets as a new set (self - other).
    /// The result contains elements that are in `self` but not in `other`.
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for &val in self.iter() {
            if !other.contains(&val) {
                result.insert(val);
            }
        }
        result
    }
}

impl<T: Numeric> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Set::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

// ---- Number Set Classification ----

/// Classification of standard mathematical number sets.
///
/// The variants follow the standard mathematical hierarchy:
/// ℕ ⊂ 𝕎 ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ.
/// Each variant corresponds to at least one concrete type in the library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberSet {
    /// Natural numbers (ℕ): 1, 2, 3, ...
    Natural,
    /// Whole numbers (𝕎): 0, 1, 2, 3, ...
    Whole,
    /// Integers (ℤ): ..., -2, -1, 0, 1, 2, ...
    Integer,
    /// Rational numbers (ℚ): a/b where a,b ∈ ℤ, b ≠ 0
    Rational,
    /// Real numbers (ℝ): all numbers on the number line
    Real,
    /// Complex numbers (ℂ): a + bi where a,b ∈ ℝ
    Complex,
}

impl NumberSet {
    /// Returns `true` if a concrete type belongs to this number set.
    ///
    /// A type belongs to a set if its `number_set()` is a subset of this set.
    /// For example, `u32` belongs to `Whole`, `Integer`, `Rational`, `Real`, and `Complex`,
    /// but NOT to `Natural` (since `0` is not in ℕ).
    ///
    /// # Example
    /// ```
    /// use rvmath::{NumberSet, NumberKind};
    /// assert!(NumberSet::Whole.contains::<u32>());
    /// assert!(!NumberSet::Natural.contains::<u32>());
    /// assert!(NumberSet::Real.contains::<f64>());
    /// ```
    pub fn contains<T: NumberKind>(&self) -> bool {
        T::number_set().is_subset_of(*self)
    }

    /// Returns `true` if `self` is a subset of (or equal to) `other` in the mathematical
    /// set hierarchy.
    ///
    /// The inclusion chain is: ℕ ⊂ 𝕎 ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ
    pub fn is_subset_of(self, other: NumberSet) -> bool {
        if self == other {
            return true;
        }
        matches!(
            (self, other),
            (NumberSet::Natural, NumberSet::Whole)
                | (NumberSet::Natural, NumberSet::Integer)
                | (NumberSet::Natural, NumberSet::Rational)
                | (NumberSet::Natural, NumberSet::Real)
                | (NumberSet::Natural, NumberSet::Complex)
                | (NumberSet::Whole, NumberSet::Integer)
                | (NumberSet::Whole, NumberSet::Rational)
                | (NumberSet::Whole, NumberSet::Real)
                | (NumberSet::Whole, NumberSet::Complex)
                | (NumberSet::Integer, NumberSet::Rational)
                | (NumberSet::Integer, NumberSet::Real)
                | (NumberSet::Integer, NumberSet::Complex)
                | (NumberSet::Rational, NumberSet::Real)
                | (NumberSet::Rational, NumberSet::Complex)
                | (NumberSet::Real, NumberSet::Complex),
        )
    }
}

/// Metadata about which mathematical number set a type belongs to.
pub trait NumberKind {
    /// Returns the mathematical number set this type belongs to.
    fn number_set() -> NumberSet;
    /// Returns true if the type can represent negative values.
    fn is_signed() -> bool;
    /// Returns true if the type only represents integer values.
    fn is_integer_valued() -> bool;
    /// Returns true if the type is a floating-point type.
    fn is_float() -> bool { false }
}

// ---- Natural (unsigned integers) ----

macro_rules! impl_whole {
    ($($t:ty),*) => {
        $(
            impl NumberKind for $t {
                fn number_set() -> NumberSet { NumberSet::Whole }
                fn is_signed() -> bool { false }
                fn is_integer_valued() -> bool { true }
                fn is_float() -> bool { false }
            }
        )*
    };
}

impl_whole!(u8, u16, u32, u64, usize);

// ---- Integer (signed integers) ----

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl NumberKind for $t {
                fn number_set() -> NumberSet { NumberSet::Integer }
                fn is_signed() -> bool { true }
                fn is_integer_valued() -> bool { true }
                fn is_float() -> bool { false }
            }
        )*
    };
}

impl_integer!(i8, i16, i32, i64, isize);

// ---- Real (floating-point) ----

macro_rules! impl_real {
    ($($t:ty),*) => {
        $(
            impl NumberKind for $t {
                fn number_set() -> NumberSet { NumberSet::Real }
                fn is_signed() -> bool { true }
                fn is_integer_valued() -> bool { false }
                fn is_float() -> bool { true }
            }
        )*
    };
}

impl_real!(f32, f64);

// ---- Num delegation ----

impl<T: Numeric + NumberKind> NumberKind for crate::num::Num<T> {
    fn number_set() -> NumberSet { T::number_set() }
    fn is_signed() -> bool { T::is_signed() }
    fn is_integer_valued() -> bool { T::is_integer_valued() }
    fn is_float() -> bool { T::is_float() }
}
