#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct T {
    pub lo: Loc,
    pub hi: Loc,
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
