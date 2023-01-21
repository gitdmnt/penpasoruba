use crate::puzzle_types::Puzzle;

pub fn simple_loop_solver(mut puzzle: Puzzle) -> Puzzle {
    //行ってはならない壁を全部ふさぐ
    puzzle = wall_fusaguer(puzzle);

    //出入り口が二枚しかないところを全部リストアップ

    //リストアップしたところを埋める

    puzzle
}

//壁を全部ふさぐ関数
fn wall_fusaguer(mut puzzle: Puzzle) -> Puzzle {
    //単純に壁があるところ
    for y in 0..puzzle.get_height() {
        for x in 0..puzzle.get_width() {}
    }
    //ふさぐと小ループになってしまうところ
    puzzle
}

//その壁がブロックによって通れないかどうかを判定する関数
fn kabe_hanteier(puzzle: &Puzzle, x: &usize, y: &usize) -> bool {
    let heiki = false;
    heiki
}

//その壁を通った時、小ループが発生するかどうかを判定する関数
fn shouloop_hanteier(puzzle: &Puzzle, x: &usize, y: &usize) -> bool {
    let heiki = false;
    heiki
}
