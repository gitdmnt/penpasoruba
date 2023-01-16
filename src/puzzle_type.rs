use core::fmt;

//順次追加
#[derive(Debug)]
pub enum PuzzleType {
    SimpleLoop,
}

//この辺全部pubにしてるのなんかキモくない？　なんかキモいと思う　アクセッサ的なやつでどうにかすればいいのかな
#[derive(Debug)]
pub struct Puzzle {
    pub name: PuzzleType,
    pub height: usize,
    pub width: usize,
    pub board: Vec<Vec<i32>>,
    pub is_finished: bool,
    pub is_cleared: bool,
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
