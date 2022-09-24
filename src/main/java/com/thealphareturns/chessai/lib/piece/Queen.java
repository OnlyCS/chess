package com.thealphareturns.chessai.lib.piece;

import com.thealphareturns.chessai.lib.util.ArrayUtil;

public class Queen extends Piece {
	public final int value = 9;

	public Queen(int y, int x, char team) {
		super(y, x, team, 'Q');
	}

	public int[][] getMoves(Piece[][] board) {
		// time to become lazy
		Piece rook = new Rook(this.y, this.x, this.team);
		Piece bishop = new Bishop(this.y, this.x, this.team);

		return ArrayUtil.concat2d(rook.getMoves(board), bishop.getMoves(board));
	}
}
