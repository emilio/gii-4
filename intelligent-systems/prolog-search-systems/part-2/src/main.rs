#![allow(dead_code)]

use std::error::Error;

mod input;
mod solver;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("Need just one argument for the input file".into());
    }

    let input = input::Input::parse(&args[1])?;

    // input.dump();

    // TODO(emilio): Probably don't hard-code these (take them from the CLI
    // instead?).
    let initial_state = solver::State {
        node: input.node_by_name("vidal").unwrap(),
        cost: 0.0,
        k_so_far: 0.5,
    };

    let goal = solver::Goal {
        node: input.node_by_name("luis").unwrap(),
    };

    let solution = solver::Solver::new(initial_state, &input, goal).solve();

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
