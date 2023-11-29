use itertools::Itertools;

use crate::{graphs, math::Vec2D};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T, InnerIter> FromIterator<InnerIter> for Grid<T>
where InnerIter: IntoIterator<Item = T>
{
    fn from_iter<I: IntoIterator<Item = InnerIter>>(iter: I) -> Self {
        iter.into()
    }
}

impl<T, InnerIter: IntoIterator<Item = T>, Iter: IntoIterator<Item = InnerIter>> From<Iter>
    for Grid<T>
{
    fn from(iter: Iter) -> Self {
        let grid = Grid {
            data: iter
                .into_iter()
                .map(|iter| iter.into_iter().collect_vec())
                .collect_vec(),
        };
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

    pub fn row(&self, row: usize) -> impl DoubleEndedIterator<Item = (Vec2D, &T)> + '_ {
        self.data[row]
            .iter()
            .enumerate()
            .map(move |(col, item)| ((col, row).into(), item))
    }

    pub fn rotate_row_left(&mut self, row: usize, mid: usize) {
        self.data[row].rotate_left(mid);
    }

    pub fn rotate_row_right(&mut self, row: usize, mid: usize) {
        self.data[row].rotate_right(mid);
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = (Vec2D, &T)>> + '_ {
        (0..self.num_rows()).map(move |row| self.row(row))
    }

    pub fn num_cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn col(&self, col: usize) -> ColIter<'_, T> {
        ColIter::new(self, col)
    }

    pub fn cols(&self) -> impl Iterator<Item = ColIter<'_, T>> + '_ {
        (0..self.num_cols()).map(move |col| self.col(col))
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Vec2D> + '_ {
        // This very nice macro creates a cartesian product.
        itertools::iproduct!(0..self.num_cols(), 0..self.num_rows()).map(Vec2D::from)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec2D, &T)> + '_ {
        self.coordinates().map(move |pos| (pos, self.get(pos)))
    }

    pub fn orthogonal_neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b Vec2D,
    ) -> impl Iterator<Item = Vec2D> + 'a {
        pos.orthogonal_neighbors()
            .filter(move |neighbor| self.contains(neighbor))
    }

    pub fn diagonal_neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b Vec2D,
    ) -> impl Iterator<Item = Vec2D> + 'a {
        pos.diagonal_neighbors()
            .filter(move |neighbor| self.contains(neighbor))
    }

    pub fn all_neighbors<'a, 'b: 'a>(&'a self, pos: &'b Vec2D) -> impl Iterator<Item = Vec2D> + 'a {
        pos.all_neighbors()
            .filter(move |neighbor| self.contains(neighbor))
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn with_value(val: T, num_rows: usize, num_cols: usize) -> Self {
        Grid {
            data: vec![vec![val; num_cols]; num_rows],
        }
    }

    fn rotate_col(&mut self, col: usize, mid: usize, up: bool) {
        let mut new_col = self.col(col).map(|(_, item)| item).cloned().collect_vec();
        if up {
            new_col.rotate_left(mid);
        } else {
            new_col.rotate_right(mid);
        }

        for (row, item) in new_col.into_iter().enumerate() {
            self.data[row][col] = item;
        }
    }

    pub fn rotate_col_up(&mut self, col: usize, mid: usize) {
        self.rotate_col(col, mid, true);
    }

    pub fn rotate_col_down(&mut self, col: usize, mid: usize) {
        self.rotate_col(col, mid, false);
    }
}

pub struct ColIter<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    row_back: usize,
    col: usize,
}

impl<'a, T> ColIter<'a, T> {
    fn new(grid: &'a Grid<T>, col: usize) -> Self {
        ColIter {
            grid,
            row: 0,
            row_back: grid.num_rows(),
            col,
        }
    }
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = (Vec2D, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.row_back {
            return None;
        }
        let pos = (self.col, self.row).into();
        let item = (pos, self.grid.get(pos));
        self.row += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.row_back - self.row;
        (len, Some(len))
    }
}

impl<'a, T> DoubleEndedIterator for ColIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.row_back == self.row {
            return None;
        }
        self.row_back -= 1;
        let pos = (self.col, self.row_back).into();
        let item = (pos, self.grid.get(pos));
        Some(item)
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
}

impl<T> graphs::UnweightedGraph for Grid<T>
where
    Grid<T>: UnweightedGrid,
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    #[test]
    fn iter_row_and_col() {
        let grid: Grid<_> = [[1, 2, 3], [4, 5, 6]].into();

        assert_eq!(
            grid.row(0).collect_vec(),
            vec![
                (Vec2D::new(0, 0), &1),
                (Vec2D::new(1, 0), &2),
                (Vec2D::new(2, 0), &3)
            ]
        );

        assert_eq!(
            grid.row(0).rev().collect_vec(),
            vec![
                (Vec2D::new(2, 0), &3),
                (Vec2D::new(1, 0), &2),
                (Vec2D::new(0, 0), &1)
            ]
        );

        assert_eq!(
            grid.row(1).collect_vec(),
            vec![
                (Vec2D::new(0, 1), &4),
                (Vec2D::new(1, 1), &5),
                (Vec2D::new(2, 1), &6)
            ]
        );

        assert_eq!(
            grid.rows().map(|row| row.collect_vec()).collect_vec(),
            vec![
                vec![
                    (Vec2D::new(0, 0), &1),
                    (Vec2D::new(1, 0), &2),
                    (Vec2D::new(2, 0), &3)
                ],
                vec![
                    (Vec2D::new(0, 1), &4),
                    (Vec2D::new(1, 1), &5),
                    (Vec2D::new(2, 1), &6)
                ],
            ]
        );

        assert_eq!(
            grid.col(0).collect_vec(),
            vec![(Vec2D::new(0, 0), &1), (Vec2D::new(0, 1), &4)]
        );

        assert_eq!(
            grid.col(0).rev().collect_vec(),
            vec![(Vec2D::new(0, 1), &4), (Vec2D::new(0, 0), &1),]
        );

        let mut col0_iter = grid.col(0);
        assert_eq!(col0_iter.size_hint(), (2, Some(2)));
        assert_eq!(col0_iter.next(), Some((Vec2D::new(0, 0), &1)));
        assert_eq!(col0_iter.size_hint(), (1, Some(1)));
        assert_eq!(col0_iter.next_back(), Some((Vec2D::new(0, 1), &4)));
        assert_eq!(col0_iter.next(), None);
        assert_eq!(col0_iter.next_back(), None);

        assert_eq!(
            grid.col(1).collect_vec(),
            vec![(Vec2D::new(1, 0), &2), (Vec2D::new(1, 1), &5)]
        );

        assert_eq!(
            grid.cols().map(|col| col.collect_vec()).collect_vec(),
            vec![
                vec![(Vec2D::new(0, 0), &1), (Vec2D::new(0, 1), &4)],
                vec![(Vec2D::new(1, 0), &2), (Vec2D::new(1, 1), &5)],
                vec![(Vec2D::new(2, 0), &3), (Vec2D::new(2, 1), &6)],
            ]
        );
    }

    #[test]
    fn rotate_rows_and_cols() {
        let mut grid: Grid<_> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].into();

        grid.rotate_row_left(0, 1);
        assert_eq!(grid.data, [[2, 3, 1], [4, 5, 6], [7, 8, 9]]);

        grid.rotate_row_right(0, 1);
        assert_eq!(grid.data, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        grid.rotate_row_left(1, 2);
        assert_eq!(grid.data, [[1, 2, 3], [6, 4, 5], [7, 8, 9]]);

        grid.rotate_row_right(1, 2);
        assert_eq!(grid.data, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        grid.rotate_col_up(0, 1);
        assert_eq!(grid.data, [[4, 2, 3], [7, 5, 6], [1, 8, 9]]);

        grid.rotate_col_down(0, 2);
        assert_eq!(grid.data, [[7, 2, 3], [1, 5, 6], [4, 8, 9]]);
    }
}
