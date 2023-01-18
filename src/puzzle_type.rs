use core::fmt;

//順次追加
#[derive(Debug)]
pub enum PuzzleType {
    SimpleLoop,
}

//型定義
#[derive(Debug)]
pub struct Puzzle {
    name: PuzzleType,
    width: usize,
    pub height: usize,
    board: Vec<Vec<i32>>,
    is_cleared: bool,
}

// コンストラクタとアクセッサ
impl Puzzle {
    //コンストラクタ
    pub fn new(name: PuzzleType, width: usize, height: usize) -> Puzzle {
        Puzzle {
            name,
            width,
            height,
            board: vec![vec![0; width * 2 + 1]; height * 2 + 1],
            is_cleared: false,
        }
    }
    //アクセッサ
    pub fn get_name(&self) -> &PuzzleType {
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
