use core::fmt;

//順次追加
#[derive(Debug)]
pub enum PuzzleName {
    SimpleLoop,
}

//型定義
#[derive(Debug)]
pub struct Puzzle {
    name: PuzzleName,
    width: usize,
    height: usize,
    board: Vec<Vec<i32>>,
    is_cleared: bool,
}

// コンストラクタとアクセッサ
impl Puzzle {
    //コンストラクタ
    pub fn new(name: PuzzleName, width: usize, height: usize) -> Puzzle {
        Puzzle {
            name,
            width,
            height,
            board: vec![vec![0; width * 2 + 1]; height * 2 + 1],
            is_cleared: false,
        }
    }
    //アクセッサ
    pub fn get_name(&self) -> &PuzzleName {
        &self.name
    }

    pub fn get_width(&self) -> &usize {
        &self.width
    }

    pub fn get_height(&self) -> &usize {
        &self.height
    }

    pub fn is_cleared(&self) -> &bool {
        &self.is_cleared
    }
}

//ゲームの終了を判定するメソッドとか
impl Puzzle {
    pub fn is_finished(&self) -> bool {
        //todo
        //パターンマッチでsolversのis_finishedに飛ばそう
        true
    }
}

//盤面をいい感じに表示すべきだと思う
impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..2 * self.height + 1 {
            for x in 0..2 * self.width + 1 {
                match (x % 2, y % 2) {
                    (1, 1) => s += self.fmt_face(x, y),
                    (0, 1) => s += self.fmt_edge(x, y, false),
                    (1, 0) => s += self.fmt_edge(x, y, true),
                    (0, 0) => s += self.fmt_vertex(x, y),
                    _ => (),
                }
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

//盤面表示関連
impl Puzzle {
    fn fmt_face(&self, x: usize, y: usize) -> &str {
        match self.board[y][x] {
            0 => " ",
            1 => "x",
            _ => ".",
        }
    }
    fn fmt_edge(&self, x: usize, y: usize, is_vertical: bool) -> &str {
        const BLOCKED: &str = "x";
        const ERROR: &str = ".";
        if is_vertical {
            match self.board[y][x] {
                0 => "─",
                1 => BLOCKED,
                _ => ERROR,
            }
        } else {
            match self.board[y][x] {
                0 => "│",
                1 => BLOCKED,
                _ => ERROR,
            }
        }
    }
    fn fmt_vertex(&self, x: usize, y: usize) -> &str {
        match self.board[y][x] {
            0 => "┼",
            1 => "x",
            _ => ".",
        }
    }
}
