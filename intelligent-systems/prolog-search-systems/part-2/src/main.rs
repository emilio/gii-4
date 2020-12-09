#![allow(dead_code)]

use std::error::Error;
use std::sync::atomic::{Ordering, AtomicBool};

pub static DEBUG: AtomicBool = AtomicBool::new(false);

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if crate::DEBUG.load(std::sync::atomic::Ordering::Relaxed) {
            eprintln!($($arg)*);
        }
    }
}

mod input;
mod solver;

const USAGE: &str = "Usage: program <input-file> <initial-state> <final-state> <use-heuristic>";

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 5 {
        return Err(USAGE.into());
    }

    DEBUG.store(std::env::var_os("DEBUG").is_some(), Ordering::Relaxed);

    let input = input::Input::parse(&args[1])?;

    if DEBUG.load(Ordering::Relaxed) {
        input.dump();
    }

    let initial_state = solver::State {
        node: input.node_by_name(&args[2]).unwrap(),
        cost: 0.0,
        k_so_far: 0.5,
    };

    let goal = solver::Goal {
        node: input.node_by_name(&args[3]).unwrap(),
    };

    let use_heuristic = &args[4] != "0";

    let solution = solver::Solver::new(initial_state, &input, goal, use_heuristic).solve();
    let solution = match solution {
        Some(s) => s,
        None => return Err("No solution found".into()),
    };

    for (i, state) in solution.path.iter().enumerate() {
        println!(
            "{} - {} (cost = {}) (k = {})",
            i,
            input.node_name(state.node),
            state.cost,
            state.k_so_far,
        )
    }

    Ok(())
}
