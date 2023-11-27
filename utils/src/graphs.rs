use std::cmp::Reverse;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

use priority_queue::PriorityQueue;

pub type Cost = usize;

pub trait Node: Clone + Debug + Eq + Ord + Hash {}
impl<T> Node for T where T: Clone + Debug + Eq + Ord + Hash {}

pub trait UnweightedGraph {
    type Node: Node;
    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = Self::Node> + 'a;
}

pub fn bfs<T: Node>(
    graph: &impl UnweightedGraph<Node = T>,
    start: impl Into<T>,
    end: impl Into<T>,
) -> Option<usize> {
    let (start, end) = (start.into(), end.into());

    let mut visited: HashSet<T> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front((start, 0));

    loop {
        match queue.pop_front() {
            None => return None,
            Some((node, distance)) => {
                if node == end {
                    return Some(distance);
                }

                visited.insert(node.clone());

                queue.extend(graph.neighbors(&node).filter_map(|n| {
                    if visited.contains(&n) {
                        None
                    } else {
                        Some((n, distance + 1))
                    }
                }));
            }
        }
    }
}

pub trait WeightedGraph {
    type Node: Node;
    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Node, Cost)> + 'a;
}

pub fn dijkstra<T: Node>(
    graph: &impl WeightedGraph<Node = T>,
    start: impl Into<T>,
    end: impl Into<T>,
) -> Option<usize> {
    let (start, end) = (start.into(), end.into());

    let mut visited = HashSet::new();
    let mut queue = PriorityQueue::new();
    queue.push(start, std::cmp::Reverse(0));

    loop {
        match queue.pop() {
            None => return None,
            Some((node, Reverse(current_cost))) => {
                if node == end {
                    return Some(current_cost);
                }
                visited.insert(node.clone());

                for (neighbor, cost) in graph.neighbors(&node) {
                    if !visited.contains(&neighbor) && queue.get(&neighbor).is_none() {
                        queue.push(neighbor, Reverse(cost + current_cost));
                    } else if let Some(Reverse(previous_cost)) = queue.get_priority(&neighbor) {
                        let new_cost = current_cost + cost;
                        if new_cost < *previous_cost {
                            queue.change_priority(&neighbor, Reverse(new_cost));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        grid::{self, Grid},
        Vec2D,
    };

    use super::*;

    #[derive(Debug, Clone)]
    struct SimpleWeightedGraph {
        edges: HashMap<&'static str, Vec<(&'static str, usize)>>,
    }

    impl WeightedGraph for SimpleWeightedGraph {
        type Node = &'static str;

        fn neighbors<'a, 'b: 'a>(
            &'a self,
            node: &'b Self::Node,
        ) -> impl Iterator<Item = (Self::Node, Cost)> + 'a {
            self.edges[node].iter().copied()
        }
    }

    #[test]
    fn graph() {
        let mut edges = HashMap::new();
        edges.insert("A", vec![("B", 1), ("D", 2)]);
        edges.insert("B", vec![("C", 7)]);
        edges.insert("C", vec![("F", 1)]);
        edges.insert("D", vec![("C", 3), ("F", 10)]);
        edges.insert("F", vec![]);
        let graph = SimpleWeightedGraph { edges };

        assert_eq!(dijkstra(&graph, "A", "B"), Some(1));
        assert_eq!(dijkstra(&graph, "A", "D"), Some(2));
        assert_eq!(dijkstra(&graph, "A", "C"), Some(5));
        assert_eq!(dijkstra(&graph, "A", "F"), Some(6));
        assert_eq!(dijkstra(&graph, "A", "G"), None);
    }

    #[test]
    fn grid_diff() {
        impl grid::WeightedGrid for Grid<usize> {
            fn cost(&self, from: Vec2D, to: Vec2D) -> Cost {
                self.get(to).abs_diff(*self.get(from))
            }

            fn neighbors<'a, 'b: 'a>(
                &'a self,
                node: &'b Vec2D,
            ) -> impl Iterator<Item = Vec2D> + 'a {
                self.orthogonal_neighbors(node)
            }
        }

        let grid: Grid<usize> = vec![
            vec![0, 2, 9, 3, 1, 2],
            vec![1, 9, 1, 3, 3, 3],
            vec![4, 1, 1, 9, 9, 1],
            vec![9, 9, 9, 9, 9, 1],
        ]
        .into();

        assert_eq!(
            dijkstra(&grid, Vec2D::new(0, 0), Vec2D::new(5, 3)),
            Some(11)
        );
        assert_eq!(dijkstra(&grid, Vec2D::new(0, 0), Vec2D::new(-1, -1)), None);
        assert_eq!(dijkstra(&grid, Vec2D::new(0, 0), Vec2D::new(0, 0)), Some(0));
    }

    #[test]
    fn grid_bfs() {
        impl grid::UnweightedGrid for Grid<char> {
            fn neighbors<'a, 'b: 'a>(
                &'a self,
                node: &'b Vec2D,
            ) -> impl Iterator<Item = Vec2D> + 'a {
                self.all_neighbors(node)
                    .filter(move |pos| self.get(*pos) != &'#')
            }
        }

        let grid: Grid<char> = vec![
            vec!['.', '#', '.', '.', '.', '.'],
            vec!['.', '#', '.', '#', '#', '.'],
            vec!['.', '#', '.', '#', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.'],
        ]
        .into();

        assert_eq!(bfs(&grid, (0, 0), (5, 3)), Some(10));
        assert_eq!(bfs(&grid, (0, 0), (1, 1)), None);
        assert_eq!(bfs(&grid, (0, 0), (0, 0)), Some(0));
    }
}
