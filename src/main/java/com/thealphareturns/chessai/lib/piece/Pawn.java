package com.thealphareturns.chessai.lib.piece;

// TODO: change itself into a whatever when reaching the end

public class Pawn extends Piece {
	public final int value = 1;

	public Pawn(int y, int x, char team) {
		super(y, x, team, 'P');
	}

	public int[][] getMoves(Piece[][] board) {
		int[][] moves = new int[2][2]; // max moves are 3. 1 forward, 1 double-forward, 1 diagonal if attacking
		int movesAdded = 0;

		switch (this.team) {
			case 'w' -> {
				// check max forward movements
				if (this.y == 0) {
					return moves;
				}

				// check double-forward
				if (this.y == 6 && board[this.y - 2][this.x] == null) {
					moves[movesAdded][0] = this.y - 2;
					moves[movesAdded][1] = this.x;
					movesAdded++;
				}

				// check forward
				if (board[this.y - 1][this.x] == null) {
					moves[movesAdded][0] = this.y - 1;
					moves[movesAdded][1] = this.x;
					movesAdded++;
				}

				// check diagonal/take
				if (this.x != 0 && board[this.y - 1][this.x - 1] != null && board[this.y - 1][this.x - 1].team != this.team) {
					moves[movesAdded][0] = this.y - 1;
					moves[movesAdded][1] = this.x - 1;
					movesAdded++;
				}
				if (this.x != 7 && board[this.y - 1][this.x + 1] != null && board[this.y - 1][this.x + 1].team != this.team) {
					moves[movesAdded][0] = this.y - 1;
					moves[movesAdded][1] = this.x + 1;
				}
			}
			case 'b' -> {
				// check max forward movements
				if (this.y == 7) {
					return moves;
				}

				// check double-forward
				if (this.y == 1 && board[this.y + 2][this.x] == null) {
					moves[movesAdded][0] = this.y + 2;
					moves[movesAdded][1] = this.x;
					movesAdded++;
				}

				// check forward
				if (board[this.y + 1][this.x] == null) {
					moves[movesAdded][0] = this.y + 1;
					moves[movesAdded][1] = this.x;
					movesAdded++;
				}

				// check diagonal/take
				if (this.x != 0 && board[this.y + 1][this.x - 1] != null && board[this.y + 1][this.x - 1].team != this.team) {
					moves[movesAdded][0] = this.y + 1;
					moves[movesAdded][1] = this.x - 1;
					movesAdded++;
				}
				if (this.x != 7 && board[this.y + 1][this.x + 1] != null && board[this.y + 1][this.x + 1].team != this.team) {
					moves[movesAdded][0] = this.y + 1;
					moves[movesAdded][1] = this.x + 1;
				}
			}
		}

		return moves;
	}
}
