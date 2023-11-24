use crate::{graphs, Vec2D};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T, Iter: IntoIterator<Item = Vec<T>>> From<Iter> for Grid<T>
{
    fn from(iter: Iter) -> Self {
        let grid = Grid { data: iter.into_iter().collect() };
        assert!(grid.data.iter().all(|row| row.len() == grid.data[0].len()));
        grid
    }
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        assert!(data.iter().all(|row| row.len() == data[0].len()));
        Grid { data }
    }

    pub fn get(&self, pos: impl Into<Vec2D>) -> &T {
        let pos = pos.into();
        &self.data[pos.y as usize][pos.x as usize]
    }

    pub fn contains(&self, pos: &Vec2D) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.num_cols() as i64 && pos.y < self.num_rows() as i64
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn num_cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Vec2D> + '_ {
        // This very nice macro creates a cartesian product.
        itertools::iproduct!(0..self.num_cols(), 0..self.num_rows()).map(Vec2D::from)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec2D, &T)> + '_ {
        self.coordinates().map(move |pos| (pos, self.get(pos)))
    }

    pub fn orthogonal_neighbors<'a, 'b: 'a>(&'a self, pos: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        pos.orthogonal_neighbors().filter(move |neighbor| self.contains(neighbor))
    }

    pub fn diagonal_neighbors<'a, 'b: 'a>(&'a self, pos: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        pos.diagonal_neighbors().filter(move |neighbor| self.contains(neighbor))
    }

    pub fn all_neighbors<'a, 'b: 'a>(&'a self, pos: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        pos.all_neighbors().filter(move |neighbor| self.contains(neighbor))
    }
}

pub trait UnweightedGrid {
    fn neighbors<'a, 'b: 'a>(&'a self, node: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a;
}

pub trait WeightedGrid {
    fn neighbors<'a, 'b: 'a>(&'a self, node: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a;

    fn cost(&self, from: Vec2D, to: Vec2D) -> graphs::Cost;
}

impl<T> graphs::WeightedGraph for Grid<T>
where
    Grid<T>: WeightedGrid,
{
    type Node = Vec2D;

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Node, graphs::Cost)> + 'a {
        assert!(self.contains(node));
        WeightedGrid::neighbors(self, node)
            .map(move |neighbor| (neighbor, self.cost(*node, neighbor)))
    }

    fn nodes(&self) -> impl Iterator<Item = Self::Node> {
        self.coordinates()
    }
}

impl<T> graphs::UnweightedGraph for Grid<T>
where
    Grid<T>: UnweightedGrid
{
    type Node = Vec2D;

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = Self::Node> + 'a {
        assert!(self.contains(node));
        UnweightedGrid::neighbors(self, node)
    }
}
