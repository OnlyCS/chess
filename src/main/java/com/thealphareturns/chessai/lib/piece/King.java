package com.thealphareturns.chessai.lib.piece;

import com.thealphareturns.chessai.lib.util.ArrayUtil;

public class King extends Piece {
	public final int value = 100;

	public King(int y, int x, char team) {
		super(y, x, team, 'K');
	}

	public int[][] getMoves(Piece[][] board) {
		int[][] moves = new int[8][2];
		int movesIndex = 0;

		// check all 8 squares around king
		for (int i = -1; i < 2; i++) {
			for (int j = -1; j < 2; j++) {
				if (this.y + i < 0 || this.y + i > 7 || this.x + j < 0 || this.x + j > 7) {
					continue;
				}

				if (board[this.y + i][this.x + j] == null) {
					moves[movesIndex][0] = this.y + i;
					moves[movesIndex][1] = this.x + j;
					movesIndex++;
				} else if (board[this.y + i][this.x + j].team != this.team) {
					moves[movesIndex][0] = this.y + i;
					moves[movesIndex][1] = this.x + j;
					movesIndex++;
				}
			}
		}

		// trim moves array
		moves = ArrayUtil.trim2d(moves, movesIndex);
		return moves;
	}
}
