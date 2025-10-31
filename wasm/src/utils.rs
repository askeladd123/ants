use anyhow::Result;

pub trait Coord {
    fn x() -> i32;
    fn y() -> i32;
}

pub struct Grid<T: Coord> {
    entries: Vec<T>,
}

impl<T: Coord> Grid<T> {
    pub fn insert(item: impl Coord) -> Result<()> {
        todo!()
    }
    pub fn update() -> Result<()> {
        todo!()
    }
}
