package com.thealphareturns.chessai.lib.piece;

import com.thealphareturns.chessai.lib.util.ArrayUtil;

public class Rook extends Piece {
	public final int value = 5;

	public Rook(int y, int x, char team) {
		super(y, x, team, 'R');
	}

	public int[][] getMoves(Piece[][] board) {
		int[][] moves = new int[14][2]; // max moves are 14. 7 in each direction if in corner
		int movesIndex = 1;
		int movesAdded = 0;

		// generates all moves in positive y direction
		while (true) {
			if (this.y + movesIndex > 7) {
				break;
			}

			// check empty or takable
			if (board[this.y + movesIndex][this.x] == null) {
				moves[movesAdded][0] = this.y + movesIndex;
				moves[movesAdded][1] = this.x;

				movesIndex++;
				movesAdded++;
			} else if (board[this.y + movesIndex][this.x].team != this.team) {
				moves[movesAdded][0] = this.y + movesIndex;
				moves[movesAdded][1] = this.x;
				movesAdded++;
				break;
			} else {
				break;
			}
		}
		movesIndex = 1;

		// generates all moves in negative y direction
		while (true) {
			if (this.y - movesIndex < 0) {
				break;
			}

			if (board[this.y - movesIndex][this.x] == null) {
				moves[movesAdded][0] = this.y - movesIndex;
				moves[movesAdded][1] = this.x;

				movesIndex++;
				movesAdded++;
			} else if (board[this.y - movesIndex][this.x].team != this.team) {
				moves[movesAdded][0] = this.y - movesIndex;
				moves[movesAdded][1] = this.x;
				movesAdded++;
				break;
			} else {
				break;
			}
		}
		movesIndex = 1;

		// generates all moves in positive x direction
		while (true) {
			if (this.x + movesIndex > 7) {
				break;
			}

			if (board[this.y][this.x + movesIndex] == null) {
				moves[movesAdded][0] = this.y;
				moves[movesAdded][1] = this.x + movesIndex;

				movesIndex++;
				movesAdded++;
			} else if (board[this.y][this.x + movesIndex].team != this.team) {
				moves[movesAdded][0] = this.y;
				moves[movesAdded][1] = this.x + movesIndex;
				movesAdded++;
				break;
			} else {
				break;
			}
		}
		movesIndex = 1;

		// generates all moves in negative x direction
		while (true) {
			if (this.x - movesIndex < 0) {
				break;
			}

			if (board[this.y][this.x - movesIndex] == null) {
				moves[movesAdded][0] = this.y;
				moves[movesAdded][1] = this.x - movesIndex;

				movesIndex++;
				movesAdded++;
			} else if (board[this.y][this.x - movesIndex].team != this.team) {
				moves[movesAdded][0] = this.y;
				moves[movesAdded][1] = this.x - movesIndex;
				movesAdded++;
				break;
			} else {
				break;
			}
		}

		// trim moves array
		moves = ArrayUtil.trim2d(moves, movesAdded);
		return moves;
	}
}
