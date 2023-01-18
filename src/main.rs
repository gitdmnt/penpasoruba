use puzzle_type::Puzzle;

mod puzzle_type;
mod solvers;

fn main() {
    //パズル指定
    let name = puzzle_type::PuzzleType::SimpleLoop;

    //盤面作成/入力
    let width: usize = 10;
    let height: usize = 10;

    //構造体のインスタンスを作成
    let puzzle = Puzzle::new(name, width, height);

    //遊ぶよ
    play(puzzle);
}

fn play(mut puzzle: puzzle_type::Puzzle) {
    while !puzzle.is_finished() {
        show_board(&puzzle);
        puzzle = solve(puzzle);
    }
    show_board(&puzzle);
}

fn show_board(puzzle: &puzzle_type::Puzzle) {
    println!("{:}", puzzle);
}

fn solve(mut puzzle: puzzle_type::Puzzle) -> puzzle_type::Puzzle {
    match puzzle.get_name() {
        puzzle_type::PuzzleType::SimpleLoop => puzzle = solvers::simple_loop_solver(puzzle),
    }
    return puzzle;
}
