use crate::Puzzle;

pub fn simple_loop_solver(mut puzzle: Puzzle) -> Puzzle {
    puzzle.is_finished = true;
    return puzzle;
}
