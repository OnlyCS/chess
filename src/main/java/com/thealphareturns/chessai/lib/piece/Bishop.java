package com.thealphareturns.chessai.lib.piece;

import com.thealphareturns.chessai.lib.util.ArrayUtil;

public class Bishop extends Piece {
	public final int value = 3;

	public Bishop(int y, int x, char team) {
		super(y, x, team, 'B');
	}

	public int[][] getMoves(Piece[][] board) {
		int[][] moves = new int[13][2]; // max moves are 13
		int movesIndex = 1;
		int movesAdded = 0;

		// generate upper-left moves
		while (true) {
			if (this.y == 0 || this.x == 0) {
				break;
			}

			if (board[this.y - movesIndex][this.x - movesIndex] == null) {
				moves[movesAdded][0] = this.y - movesIndex;
				moves[movesAdded][1] = this.x - movesIndex;
				movesAdded++;
				movesIndex++;
			} else if (board[this.y - movesIndex][this.x - movesIndex].team != this.team) {
				moves[movesAdded][0] = this.y - movesIndex;
				moves[movesAdded][1] = this.x - movesIndex;
				movesAdded++;
				break;
			} else {
				break;
			}
		}
		movesIndex = 1;

		// generate upper-right moves
		while (true) {
			if (this.y == 0 || this.x == 7) {
				break;
			}

			if (board[this.y - movesIndex][this.x + movesIndex] == null) {
				moves[movesAdded][0] = this.y - movesIndex;
				moves[movesAdded][1] = this.x + movesIndex;
				movesAdded++;
				movesIndex++;
			} else if (board[this.y - movesIndex][this.x + movesIndex].team != this.team) {
				moves[movesAdded][0] = this.y - movesIndex;
				moves[movesAdded][1] = this.x + movesIndex;
				movesAdded++;
				break;
			} else {
				break;
			}
		}
		movesIndex = 1;

		// generate lower-left moves
		while (true) {
			if (this.y == 7 || this.x == 0) {
				break;
			}

			if (board[this.y + movesIndex][this.x - movesIndex] == null) {
				moves[movesAdded][0] = this.y + movesIndex;
				moves[movesAdded][1] = this.x - movesIndex;
				movesAdded++;
				movesIndex++;
			} else if (board[this.y + movesIndex][this.x - movesIndex].team != this.team) {
				moves[movesAdded][0] = this.y + movesIndex;
				moves[movesAdded][1] = this.x - movesIndex;
				movesAdded++;
				break;
			} else {
				break;
			}
		}
		movesIndex = 1;

		// generate lower-right moves
		while (true) {
			if (this.y == 7 || this.x == 7) {
				break;
			}

			if (board[this.y + movesIndex][this.x + movesIndex] == null) {
				moves[movesAdded][0] = this.y + movesIndex;
				moves[movesAdded][1] = this.x + movesIndex;
				movesAdded++;
				movesIndex++;
			} else if (board[this.y + movesIndex][this.x + movesIndex].team != this.team) {
				moves[movesAdded][0] = this.y + movesIndex;
				moves[movesAdded][1] = this.x + movesIndex;
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
