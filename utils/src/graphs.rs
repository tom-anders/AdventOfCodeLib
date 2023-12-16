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

pub enum NextState<I> {
    Terminal(usize),
    Next(I),
}

pub trait DfsGraph {
    type State: Clone + Debug + Eq + Hash;
    /// Return None for terminal state
    fn next_states<'a, 'b: 'a>(
        &'a self,
        current: &'b Self::State,
        current_best: usize,
        depth: usize,
    ) -> NextState<impl IntoIterator<Item = Self::State> + 'a>;
}

pub fn dfs<T>(graph: &impl DfsGraph<State = T>, start: impl Into<T>) -> Option<Cost> {
    let mut current_best = 0;
    dfs_impl(graph, &start.into(), &mut current_best, 0)
}

fn dfs_impl<T>(
    graph: &impl DfsGraph<State = T>,
    current: &T,
    current_best: &mut Cost,
    depth: usize,
) -> Option<usize> {
    return match graph.next_states(current, *current_best, depth) {
        NextState::Terminal(score) => {
            *current_best = score.max(*current_best);
            Some(*current_best)
        }
        NextState::Next(next_states) => next_states
            .into_iter()
            .flat_map(|next_state| dfs_impl(graph, &next_state, current_best, depth + 1))
            .max(),
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        grid::{self, Grid},
        math::Vec2D,
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
                self[to].abs_diff(self[from])
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
                    .filter(move |pos| self[*pos] != '#')
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

    #[test]
    fn test_dfs() {
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        enum State {
            A,
            B,
            C,
            D,
            E,
            F,
            G,
        }

        struct StateGraph;
        impl DfsGraph for StateGraph {
            type State = State;

            fn next_states<'a, 'b: 'a>(
                &'a self,
                current: &'b Self::State,
                _current_best: usize,
                _depth: usize,
            ) -> NextState<impl IntoIterator<Item = Self::State> + 'a> {
                use NextState::*;
                match current {
                    State::A => Next([State::B, State::C]),
                    State::B => Terminal(10),
                    State::C => Next([State::D, State::E]),
                    State::D => Next([State::F, State::G]),
                    State::E => Terminal(11),
                    State::F => Terminal(12),
                    State::G => Terminal(13),
                }
            }
        }

        assert_eq!(dfs(&StateGraph {}, State::A), Some(13)); // A -> C -> H -> 13
    }
}
