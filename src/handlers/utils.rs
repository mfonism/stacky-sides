pub fn is_winning_move(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> bool {
    // line: |
    let (row_top, _col_top) = seek_top(row, col, game_board);
    let (row_btm, _col_btm) = seek_bottom(row, col, game_board);
    if row_btm - row_top >= 3 {
        return true;
    }

    // line: -
    let (_row_left, col_left) = seek_left(row, col, game_board);
    let (_row_right, col_right) = seek_right(row, col, game_board);
    if col_right - col_left >= 3 {
        return true;
    }

    // line: /
    let (_row_top_right, col_top_right) = seek_top_right(row, col, game_board);
    let (_row_btm_left, col_btm_left) = seek_bottom_left(row, col, game_board);
    if col_top_right - col_btm_left >= 3 {
        return true;
    }

    // line: \
    let (_row_top_left, col_top_left) = seek_top_left(row, col, game_board);
    let (_row_btm_right, col_btm_right) = seek_bottom_right(row, col, game_board);
    if col_btm_right - col_top_left >= 3 {
        return true;
    }

    return false;
}

fn seek_top(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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

fn seek_bottom(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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

fn seek_right(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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

fn seek_left(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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

fn seek_top_right(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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

fn seek_top_left(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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

fn seek_bottom_right(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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

fn seek_bottom_left(row: usize, col: usize, game_board: &Vec<Vec<u8>>) -> (usize, usize) {
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
