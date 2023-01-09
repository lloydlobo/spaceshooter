// use crate::prelude::*;

/// Enum with two possible values: `True` or `False` which represent the
/// [`logical`] values `true` and `false` respectively.
///
/// [`logical`]: https://en.wikipedia.org/wiki/Boolean_algebra
/// [See also])https://docs.rs/boolean/0.3.0/src/boolean/lib.rs.html#227-234)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Updated {
    False,
    True,
}

impl Default for Updated {
    fn default() -> Self {
        Self::False
    }
}

impl Updated {
    /// Creates a `Updated` based on the given primitive `bool`.
    /// If the value is `true`, it will return `Updated::True` and
    /// if the value is`false` will return `Updated::False`
    pub fn new(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }

    pub fn is_true(self) -> bool {
        self == Self::True
    }
    pub fn is_false(self) -> bool {
        self == Self::False
    }
}

impl From<bool> for Updated {
    fn from(item: bool) -> Self {
        if item {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<Updated> for bool {
    fn from(item: Updated) -> Self {
        match item {
            Updated::True => true,
            Updated::False => false,
        }
    }
}

impl From<&Updated> for bool {
    fn from(item: &Updated) -> Self {
        match item {
            Updated::True => true,
            Updated::False => false,
        }
    }
}

impl From<&mut Updated> for bool {
    fn from(item: &mut Updated) -> Self {
        match item {
            Updated::True => true,
            Updated::False => false,
        }
    }
}
