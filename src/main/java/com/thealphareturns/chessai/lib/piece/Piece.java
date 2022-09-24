package com.thealphareturns.chessai.lib.piece;

/* Reference:
 * The team can be "w" for white or "b" for black
 * The default white left-side rook starting position would be at the bottom-left of the two-dimensional array
 * so [7][0]
 *
 * The default black left-side rook starting position would be at the top-left of the two-dimensional array
 * so [0][0]
 */

public class Piece {
	public char team;
	private char type;
	public int value = 0;

	int y;
	int x;

	public Piece(int y, int x, char team, char type) {
		this.y = y;
		this.x = x;
		this.team = team;
		this.type = type;
	}

	public Piece[][] move(int y, int x, Piece[][] board) {
		board[this.y][this.x] = null;
		this.y = y;
		this.x = x;

		board[y][x] = this;

		return board;
	}

	/**
	 * Gets all the moves that the piece can make
	 * @param board the board
	 * @return an array of all the moves.
	 */
	public int[][] getMoves(Piece[][] board) {
		throw new UnsupportedOperationException("This piece does not have a getMoves() method implemented");
	}

	public boolean canMakeMove(int y, int x, Piece[][] board) {
		int[][] moves = this.getMoves(board);
		for (int i = 0; i < moves.length; i++) {
			if (moves[i][0] == y && moves[i][1] == x) {
				return true;
			}
		}
		return false;
	}

	public String toString() {
		switch (this.type) {
			case 'P' -> {
				return this.team == 'w'
						? "\u2659"
						: "\u265F";
			}
			case 'R' -> {
				return this.team == 'w'
						? "\u2656"
						: "\u265C";
			}
			case 'N' -> {
				return this.team == 'w'
						? "\u2658"
						: "\u265E";
			}
			case 'B' -> {
				return this.team == 'w'
						? "\u2657"
						: "\u265D";
			}
			case 'Q' -> {
				return this.team == 'w'
						? "\u2655"
						: "\u265B";
			}
			case 'K' -> {
				return this.team == 'w'
						? "\u2654"
						: "\u265A";
			}
		}

		throw new UnsupportedOperationException("This piece does not have a toString() method implemented");
	}
}
