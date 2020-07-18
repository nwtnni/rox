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

