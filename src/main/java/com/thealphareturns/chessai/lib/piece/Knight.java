package com.thealphareturns.chessai.lib.piece;

public class Knight extends Piece {
	public final int value = 3;

	public Knight(int y, int x, char team) {
		super(y, x, team, 'N');
	}

	public int[][] getMoves(Piece[][] board) {
		int[][] moves = new int[8][2];
		int[][] check = {{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2}, {1, -2}, {1, 2}, {2, -1}, {2, 1}};
		int movesAdded = 0;

		for (int[] move : check) {
			if (this.y + move[0] < 0 || this.y + move[0] > 7 || this.x + move[1] < 0 || this.x + move[1] > 7) {
				continue;
			}

			if (board[this.y + move[0]][this.x + move[1]] == null) {
				moves[movesAdded][0] = this.y + move[0];
				moves[movesAdded][1] = this.x + move[1];
				movesAdded++;
			} else if (board[this.y + move[0]][this.x + move[1]].team != this.team) {
				moves[movesAdded][0] = this.y + move[0];
				moves[movesAdded][1] = this.x + move[1];
				movesAdded++;
			}
		}

		return moves;
	}
}
