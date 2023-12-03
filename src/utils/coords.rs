use derive_more::Display;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Display, Copy, Hash)]
#[display(fmt = "({}, {})", y, x)]
pub struct Coords {
    pub y: usize,
    pub x: usize,
}

impl Coords {
    pub fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

impl From<Coords> for (usize, usize) {
    fn from(value: Coords) -> Self {
        (value.y, value.x)
    }
}
impl From<(usize, usize)> for Coords {
    fn from((y, x): (usize, usize)) -> Self {
        Self::new(y, x)
    }
}
