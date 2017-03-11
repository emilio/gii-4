#![feature(link_args)]

#[allow(dead_code)]
#[cfg(not(test))]
fn static_assert() {
    unsafe { ::std::mem::transmute::<u32, usize>(1u32) };
}

/// A problem-solving strategy for the n-queens problem.
pub trait NQueensStrategy: Sized {
    /// Extra parameters that may be given to the challenge to configure the
    /// solution.
    type Config;

    /// Creates a new solvable instance of this challenge.
    fn new(dimension: usize, config: Self::Config) -> Self;

    /// Solves the challenge for returning a vector with `n` positions,
    /// indicated as a vector into an array of dimension * dimension.
    ///
    /// Each step will call step_callback, if appropriate, with the current
    /// state of the board.
    fn solve<F>(&mut self, step_callback: F) -> Option<Vec<usize>>
        where F: FnMut(&[usize]);
}

pub mod hill_climbing {
    use super::*;

    /// The id of a queen, which is effectively an index.
    pub type QueenId = usize;

    /// A hill-climbing solution to the n-queens challenge
    pub struct HillClimbing(usize);

    impl HillClimbing {
        fn dimension(&self) -> usize {
            self.0
        }

        /// Returns true if a queen positioned at `one` could be hit by a queen
        /// positioned at `other`.
        fn positions_are_reachable(&self, one: usize, other: usize) -> bool {
            let (x1, y1) = (one % self.dimension(), one / self.dimension());
            let (x2, y2) = (other % self.dimension(), other / self.dimension());

            if x1 == x2 || y1 == y2 {
                return true; // aligned
            }

            let x_difference = (x1 as isize - x2 as isize).abs();
            let y_difference = (y1 as isize - y2 as isize).abs();

            if x_difference == y_difference {
                return true; // diagonal
            }

            false
        }

        fn queen_can_be_positioned_at(&self, pos: usize, queen_positions: &[usize]) -> bool {
            for existing_position in queen_positions {
                if pos == *existing_position || self.positions_are_reachable(pos, *existing_position) {
                    return false;
                }
            }

            true
        }

        /// Tries to position the next queen in a greedy way. Gets an immutable
        /// view of the current positions, and returns the position the next
        /// queen is at, or an error if it can't be positioned.
        fn position_next_queen_starting_from(&self,
                                             initial: usize,
                                             queen_positions: &[usize])
                                             -> Result<usize, ()> {
            for pos in initial..(self.dimension() * self.dimension()) {
                if self.queen_can_be_positioned_at(pos, queen_positions) {
                    return Ok(pos)
                }
            }

            Err(())
        }
    }

    impl NQueensStrategy for HillClimbing {
        /// No configuration needed.
        type Config = ();

        fn new(dimensions: usize, _: ()) -> Self {
            HillClimbing(dimensions)
        }

        fn solve<F>(&mut self, mut step_callback: F) -> Option<Vec<usize>>
            where F: FnMut(&[usize]),
        {
            if self.dimension() == 0 {
                return Some(vec![]);
            }

            let mut positions = vec![];

            let mut start_search_at = 0;
            while positions.len() != self.dimension() {
                match self.position_next_queen_starting_from(start_search_at, &positions) {
                    Ok(pos) => {
                        positions.push(pos);
                        step_callback(&positions);
                        start_search_at = 0;
                    }
                    Err(()) => {
                        match positions.pop() {
                            Some(last_queen_position) => {
                                start_search_at = last_queen_position + 1;
                            }
                            // No solution.
                            None => return None,
                        }
                    }
                }
            }

            return Some(positions);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const DIM: usize = 8;
        fn pos(x: usize, y: usize) -> usize {
            x + y * DIM
        }

        #[test]
        fn are_reachable_test() {
            let challenge = HillClimbing::new(DIM, ());

            assert!(challenge.positions_are_reachable(pos(0, 0), pos(0, 0)));
            assert!(challenge.positions_are_reachable(pos(0, 1), pos(0, 0)));
            assert!(challenge.positions_are_reachable(pos(1, 0), pos(0, 0)));
            assert!(challenge.positions_are_reachable(pos(1, 1), pos(5, 5)));
            assert!(challenge.positions_are_reachable(pos(3, 2), pos(2, 3)));
        }

        #[test]
        fn finds_eight_queens_solution() {
            let mut challenge = HillClimbing::new(DIM, ());
            assert!(challenge.solve(|_| {}).is_some());
        }
    }
}

pub type JSCallback = extern "C" fn(positions: *const usize, len: usize);

pub fn solve<T: NQueensStrategy>(n: usize,
                                 result_storage: *mut u32,
                                 callback: Option<JSCallback>,
                                 config: T::Config)
                                 -> usize {
    use std::{mem, slice};

    let mut challenge = T::new(n, config);
    let result = challenge.solve(|step| {
        if let Some(cb) = callback {
            cb(step.as_ptr(), step.len());
        }
    });

    assert_eq!(mem::size_of::<usize>(), mem::size_of::<u32>());

    let result = match result {
        Some(result) => result,
        None => return 0,
    };

    let mut storage = unsafe { slice::from_raw_parts_mut(result_storage, n) };
    for (i, pos) in result.into_iter().enumerate() {
        storage[i] = pos as u32;
    }

    return 1;
}

#[cfg(not(test))]
#[link_args = "-s EXPORTED_FUNCTIONS=['_solve_n_queens_hill_climbing'] -s RESERVED_FUNCTION_POINTERS=20"]
extern {}

#[no_mangle]
pub fn solve_n_queens_hill_climbing(n: usize,
                                    result_storage: *mut u32,
                                    cb: Option<JSCallback>)
                                    -> usize {
    solve::<hill_climbing::HillClimbing>(n, result_storage, cb, ())
}

fn main() { /* Intentionally empty */ }
