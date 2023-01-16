use core::fmt;

mod solvers;

#[derive(Debug)]
enum PuzzleType {
    SimpleLoop,
}
#[derive(Debug)]
pub struct Puzzle {
    name: PuzzleType,
    height: usize,
    width: usize,
    board: Vec<Vec<i32>>,
    is_finished: bool,
    is_cleared: bool,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                match self.board[i][j] {
                    0 => {
                        write!(f, " ").unwrap();
                    }
                    1 => {
                        write!(f, "x").unwrap();
                    }
                    _ => {
                        write!(f, "{}", self.board[i][j]).unwrap();
                    }
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

fn main() {
    //パズル指定
    let name = PuzzleType::SimpleLoop;

    //盤面作成/入力
    let height: usize = 10;
    let width: usize = 10;

    //構造体のインスタンスを作成
    let puzzle = Puzzle {
        name,
        height: height,
        width: width,
        board: vec![vec![0; width * 2 + 1]; height * 2 + 1],
        is_finished: false,
        is_cleared: false,
    };

    //遊ぶよ
    play(puzzle);
}

fn play(mut puzzle: Puzzle) {
    while !puzzle.is_finished {
        showBoard(&puzzle);
        puzzle = solve(puzzle);
    }
}

fn showBoard(puzzle: &Puzzle) {
    println!("{:}", puzzle);
}

fn solve(mut puzzle: Puzzle) -> Puzzle {
    match puzzle.name {
        PuzzleType::SimpleLoop => puzzle = solvers::simple_loop_solver(puzzle),
        _ => println!("何のゲームやるよ??"),
    }
    return puzzle;
}
