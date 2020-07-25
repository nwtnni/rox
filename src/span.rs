use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct T {
    pub lo: Loc,
    pub hi: Loc,
}

impl fmt::Display for T {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}-{}", self.lo, self.hi)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Loc {
    pub idx: usize,
    pub row: usize,
    pub col: usize,
}

impl Loc {
    pub fn next_row(&self, idx: usize) -> Self {
        Loc {
            idx,
            row: self.row + 1,
            col: 1,
        }
    }

    pub fn next_col(&self, idx: usize) -> Self {
        Loc {
            idx,
            row: self.row,
            col: self.col + 1,
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}:{}", self.row, self.col)
    }
}
