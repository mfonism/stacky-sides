#[derive(Debug)]
pub enum GameMessage {
    Board { state_str: String },
    Selection { row: u8, col: u8 },
    End { winner: u8 }, // 1 or 2 (or 0, in the case of a draw)
}

impl GameMessage {
    pub fn read(text: String) -> Result<Self, &'static str> {
        let text = text.to_lowercase();

        if text.starts_with("board") {
            return Ok(Self::Board { state_str: text });
        }

        let parts = text.split(" ").collect::<Vec<&str>>();
        if parts.len() == 0 {
            return Err("empty message!");
        }

        if parts[0] == "selection" && parts.len() == 3 {
            if let (Ok(row), Ok(col)) = (parts[1].parse(), parts[2].parse()) {
                return Ok(GameMessage::Selection { row, col });
            }
            return Err("could not parse selection message");
        }

        if parts[0] == "end" && parts.len() == 2 {
            if let Ok(winner) = parts[1].parse() {
                return Ok(GameMessage::End { winner });
            }
            return Err("could not parse game end message");
        }

        Err("could not parse message")
    }
}

pub fn is_winning_move(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> bool {
    true
}

pub fn seek_top(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let mut row = row;

    while row > 0 {
        if game_board[row - 1][col] != item {
            break;
        }

        row -= 1;
    }

    (row, col)
}

pub fn seek_bottom(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let mut row = row;

    while row < game_board.len() - 1 {
        if game_board[row + 1][col] != item {
            break;
        }

        row += 1;
    }

    (row, col)
}

pub fn seek_right(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let mut col = col;

    while col < game_board[row].len() - 1 {
        if game_board[row][col + 1] != item {
            break;
        }

        col += 1;
    }

    (row, col)
}

pub fn seek_left(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let mut col = col;

    while col > 0 {
        if game_board[row][col - 1] != item {
            break;
        }

        col -= 1;
    }

    (row, col)
}

pub fn seek_top_right(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let (mut row, mut col) = (row, col);

    while row > 0 && col < game_board[row].len() - 1 {
        if game_board[row - 1][col + 1] != item {
            break;
        }

        row -= 1;
        col += 1;
    }

    (row, col)
}

pub fn seek_top_left(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let (mut row, mut col) = (row, col);

    while row > 0 && col > 0 {
        if game_board[row - 1][col - 1] != item {
            break;
        }

        row -= 1;
        col -= 1;
    }

    (row, col)
}

pub fn seek_bottom_right(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let (mut row, mut col) = (row, col);

    while row < game_board.len() - 1 && col < game_board[row].len() - 1 {
        if game_board[row + 1][col + 1] != item {
            break;
        }

        row += 1;
        col += 1;
    }

    (row, col)
}

pub fn seek_bottom_left(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
    let item = game_board[row][col];
    let (mut row, mut col) = (row, col);

    while row < game_board.len() - 1 && col > 0 {
        if game_board[row + 1][col - 1] != item {
            break;
        }

        row += 1;
        col -= 1;
    }

    (row, col)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_board() -> Vec<Vec<u8>> {
        vec![
            vec![0, 0, 0, 0, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 1, 0, 0],
        ]
    }

    fn get_extra_board() -> Vec<Vec<u8>> {
        vec![
            vec![2, 0, 2, 0, 0],
            vec![0, 0, 2, 0, 0],
            vec![2, 0, 2, 2, 2],
            vec![0, 0, 0, 0, 0],
            vec![0, 2, 2, 0, 0],
        ]
    }

    #[test]
    fn top_right_end_of_chain() {
        let ref board = get_board();

        assert_eq!(seek_top_right(1, 1, board), (1, 1));
        assert_eq!(seek_top_right(3, 1, board), (0, 4));
        assert_eq!(seek_top_right(4, 2, board), (3, 3));
    }

    #[test]
    fn top_left_end_of_chain() {
        let ref board = get_board();

        assert_eq!(seek_top_left(1, 1, board), (1, 1));
        assert_eq!(seek_top_left(3, 3, board), (1, 1));
        assert_eq!(seek_top_left(4, 2, board), (3, 1));
    }

    #[test]
    fn bottom_right_end_of_chain() {
        let ref board = get_board();

        assert_eq!(seek_bottom_right(0, 4, board), (0, 4));
        assert_eq!(seek_bottom_right(1, 1, board), (3, 3));
        assert_eq!(seek_bottom_right(2, 2, board), (3, 3));
    }

    #[test]
    fn bottom_left_end_of_chain() {
        let ref board = get_board();

        assert_eq!(seek_bottom_left(0, 4, board), (3, 1));
        assert_eq!(seek_bottom_left(1, 1, board), (1, 1));
        assert_eq!(seek_bottom_left(2, 2, board), (3, 1));
    }

    #[test]
    fn top_of_chain() {
        let ref board = get_extra_board();

        assert_eq!(seek_top(0, 0, board), (0, 0));
        assert_eq!(seek_top(2, 2, board), (0, 2));
        assert_eq!(seek_top(4, 2, board), (4, 2));
    }

    #[test]
    fn bottom_of_chain() {
        let ref board = get_extra_board();

        assert_eq!(seek_bottom(0, 0, board), (0, 0));
        assert_eq!(seek_bottom(0, 2, board), (2, 2));
        assert_eq!(seek_bottom(4, 2, board), (4, 2));
    }

    #[test]
    fn right_end_of_chain() {
        let ref board = get_extra_board();

        assert_eq!(seek_right(2, 0, board), (2, 0));
        assert_eq!(seek_right(2, 3, board), (2, 4));
        assert_eq!(seek_right(4, 2, board), (4, 2));
    }

    #[test]
    fn left_end_of_chain() {
        let ref board = get_extra_board();

        assert_eq!(seek_left(2, 0, board), (2, 0));
        assert_eq!(seek_left(2, 3, board), (2, 2));
        assert_eq!(seek_left(4, 2, board), (4, 1));
    }
}
