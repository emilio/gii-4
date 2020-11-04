use crate::input::{Input, NodeId, NodeMap};
use std::cmp;
use std::collections::BinaryHeap;

/// A state in our heap (that is, node + our cost so far + the k value from the
/// exercise).
///
/// TODO: In order to make it a proper A* we need an heuristic function...
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct State {
    pub node: NodeId,
    pub cost: f32,
    pub k_so_far: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Goal {
    pub node: NodeId,
}

#[derive(Default, Debug)]
pub struct Solution {
    pub path: Vec<State>,
}

impl PartialOrd for State{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(
            self.cost
                .partial_cmp(&other.cost)
                .expect("No NaN and other shenanigans here!")
                // These two are mostly to make PartialOrd match PartialEq.
                .then_with(|| self.node.cmp(&other.node))
                .then_with(|| self.k_so_far.partial_cmp(&other.k_so_far).unwrap())
        )
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
pub struct Solver<'a> {
    initial_state: State,
    input: &'a Input,
    goal: Goal,
    // We use Reverse here to do a min-heap (and thus extract the elements with
    // less cost first) rather than a regular binary heap, see:
    // https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap
    open: BinaryHeap<cmp::Reverse<State>>,
    // A map from a node to the best path so far, along with the cost of
    // actually visiting it.
    came_from: NodeMap<State>,
}

impl<'a> Solver<'a> {
    pub fn new(initial_state: State, input: &'a Input, goal: Goal) -> Self {
        Self {
            initial_state,
            input,
            goal,
            open: BinaryHeap::new(),
            came_from: NodeMap::default(),
        }
    }

    pub fn solve(mut self) -> Option<Solution> {
        /// We can't use k as a cost really, because it doesn't tell us
        /// anything, so we use a fixed cost for each step of the way.
        ///
        /// This kinda sucks and makes this not really an A* algorithm I guess,
        /// since we don't really have an estimation function to get from a node
        /// to the goal... So we end up really just doing some sort of fancy
        /// breadth-first-search, really.
        const FIXED_COST: f32 = 1.0;

        self.open.push(cmp::Reverse(self.initial_state));

        // This is part of the problem statement, see quotes on the uses below.
        fn acceptable(k: f32) -> bool {
            k > 0.09
        }

        while let Some(cmp::Reverse(current)) = self.open.pop() {
            // El Votante_Convencido tiene una CargaIdeológicaConvencido
            // significativa (mayor que 0.09) [...]
            if !acceptable(current.k_so_far) {
                continue;
            }

            if current.node == self.goal.node {
                let mut path = vec![];
                path.push(current);

                let mut current = current.node;
                while let Some(state) = self.came_from.get(current) {
                    path.push(state.clone());
                    if state.node == self.initial_state.node {
                        break;
                    }
                    current = state.node;
                }

                path.reverse();
                return Some(Solution { path });
            }

            for edge in self.input.edges_from(current.node) {
                // We have no distance information, unfortunately.
                let cost = current.cost + FIXED_COST;
                let k = current.k_so_far * edge.k;

                // Votante_A_Convencer tiene una CargaIdeológicaAConvencer
                // resultante significativa (mayor 0.09) [...]
                if !acceptable(k) {
                    continue;
                }

                // If it worse than the existing best path, we also bail.
                if let Some(state) = self.came_from.get(edge.to) {
                    if state.cost <= cost {
                        // This is what really cuts the recursion and prevents
                        // us from eternally ping-ponging between different
                        // states.
                        continue;
                    }
                }

                self.came_from.insert(edge.to, current);

                let new_state = State {
                    node: edge.to,
                    cost,
                    k_so_far: k,
                };
                self.open.push(cmp::Reverse(new_state));
            }
        }

        None
    }
}
