use penpasoruba::puzzle_type;
use penpasoruba::solvers;

fn main() {
    //パズル指定
    let name = puzzle_type::PuzzleType::SimpleLoop;

    //盤面作成/入力
    let width: usize = 10;
    let height: usize = 10;

    //構造体のインスタンスを作成
    let puzzle = puzzle_type::Puzzle::new(name, width, height);

    //遊ぶよ
    play(puzzle);
}

//遊ぶやつ
fn play(mut puzzle: puzzle_type::Puzzle) {
    while !puzzle.is_finished() {
        show_board(&puzzle);
        puzzle = solve(puzzle);
    }
    //解き終わったらメッセージと盤面表示したほうがいい
    show_board(&puzzle);
}

fn show_board(puzzle: &puzzle_type::Puzzle) {
    println!("{:}", puzzle);
}

//これ(パズルの種類によるソルバーの振り分け)もっと簡単に書きたいけど、どっかで結局パターンマッチしなきゃいけないんだろうな　とりあえずここでやる
fn solve(mut puzzle: puzzle_type::Puzzle) -> puzzle_type::Puzzle {
    match puzzle.get_name() {
        puzzle_type::PuzzleType::SimpleLoop => {
            puzzle = solvers::simple_loop::simple_loop_solver(puzzle)
        }
    }
    return puzzle;
}
