use crate::config::BOARD_SIZE;

pub fn setup_board() -> Vec<Vec<Vec<i8>>> {
	let mut board = vec![vec![vec![-1; BOARD_SIZE.2]; BOARD_SIZE.1]; BOARD_SIZE.0];

	board[0][0][0] = 3;
	board[1][0][0] = 1;
	board[2][0][0] = 2;
	board[3][0][0] = 4;
	board[4][0][0] = 5;
	board[5][0][0] = 2;
	board[6][0][0] = 1;
	board[7][0][0] = 3;

	board[0][1][0] = 0;
	board[1][1][0] = 0;
	board[2][1][0] = 0;
	board[3][1][0] = 0;
	board[4][1][0] = 0;
	board[5][1][0] = 0;
	board[6][1][0] = 0;
	board[7][1][0] = 0;

	board[3][6][0] = 6;
	board[4][6][0] = 6;

	board[0][7][0] = 9;
	board[7][7][0] = 9;

	board[1][1][1] = 0;
	board[2][1][1] = 0;
	board[3][1][1] = 0;
	board[4][1][1] = 0;
	board[5][1][1] = 0;
	board[6][1][1] = 0;

	board[2][2][1] = 2;
	board[5][2][1] = 2;

	board[2][6][1] = 6;
	board[3][6][1] = 6;
	board[4][6][1] = 6;
	board[5][6][1] = 6;

	board[2][7][1] = 7;
	board[5][7][1] = 7;

	board[2][0][2] = 1;
	board[5][0][2] = 1;

	board[2][1][2] = 0;
	board[3][1][2] = 0;
	board[4][1][2] = 0;
	board[5][1][2] = 0;

	board[2][5][2] = 8;
	board[5][5][2] = 8;

	board[1][6][2] = 6;
	board[2][6][2] = 6;
	board[3][6][2] = 6;
	board[4][6][2] = 6;
	board[5][6][2] = 6;
	board[6][6][2] = 6;

	board[0][0][3] = 3;
	board[7][0][3] = 3;

	board[3][1][3] = 0;
	board[4][1][3] = 0;

	board[0][6][3] = 6;
	board[1][6][3] = 6;
	board[2][6][3] = 6;
	board[3][6][3] = 6;
	board[4][6][3] = 6;
	board[5][6][3] = 6;
	board[6][6][3] = 6;
	board[7][6][3] = 6;

	board[0][7][3] = 9;
	board[1][7][3] = 7;
	board[2][7][3] = 8;
	board[3][7][3] = 11;
	board[4][7][3] = 10;
	board[5][7][3] = 8;
	board[6][7][3] = 7;
	board[7][7][3] = 9;

	board
}
