use crate::utils::Coords;
use array2d::Array2D;

pub fn access_grid<T>(grid: &Array2D<T>, coords: Coords) -> &T {
    grid.get(coords.y, coords.x).unwrap()
}

pub fn all_coords<T>(grid: &Array2D<T>) -> impl Iterator<Item = Coords> {
    let columns = grid.num_columns();
    let rows = grid.num_rows();
    (0..rows)
        .map(move |row| {
            std::iter::repeat(row)
                .take(columns)
                .enumerate()
                .map(|(row, column)| Coords { y: row, x: column })
        })
        .flatten()
}

pub type MaybeNextCell<'a, T> = Option<(&'a T, Coords)>;

pub fn go_left<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (y, x) = coords.into();
    match x.overflowing_sub(1) {
        (_, true) => None,
        (new_column, false) => Some((grid.get(y, new_column)?, Coords { x: new_column, y })),
    }
}
pub fn go_right<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (y, x) = coords.into();
    let new_column = x + 1;
    if new_column < grid.num_columns() {
        Some((grid.get(y, new_column)?, Coords { x: new_column, y }))
    } else {
        None
    }
}
pub fn go_up<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (y, x) = coords.into();
    match y.overflowing_sub(1) {
        (_, true) => None,
        (new_row, false) => Some((grid.get(new_row, x)?, Coords { x, y: new_row })),
    }
}
pub fn go_down<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (y, x) = coords.into();
    let new_row = y + 1;
    if new_row < grid.num_rows() {
        Some((grid.get(new_row, x)?, Coords { x, y: new_row }))
    } else {
        None
    }
}
pub fn go_left_up<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (_, moved) = go_left(grid, coords)?;
    go_up(grid, moved)
}
pub fn go_right_up<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (_, moved) = go_right(grid, coords)?;
    go_up(grid, moved)
}
pub fn go_left_down<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (_, moved) = go_left(grid, coords)?;
    go_down(grid, moved)
}
pub fn go_right_down<T>(grid: &Array2D<T>, coords: Coords) -> MaybeNextCell<T> {
    let (_, moved) = go_right(grid, coords)?;
    go_down(grid, moved)
}
