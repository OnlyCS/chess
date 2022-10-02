package com.thealphareturns.chessai.lib.util;

import com.thealphareturns.chessai.lib.board.Board;
import com.thealphareturns.chessai.lib.piece.Piece;

public class MoveUtil {
	/**
	 * Trims moves that put the King in check
	 * @param board the board
	 * @param moves array of generated moves
	 * @param piece the piece that generated the moves
	 * @return the trimmed array of moves
	 */
	public static int[][] trimCheck(Piece[][] board, int[][] moves, Piece piece) {
		int[][] trimmedMoves;

		int index = 0;
		for (int[] move : moves) {
			Board newBoard = (Board) DeepCopy.copy(board);
			int[] pieceCoords = piece.getCoords();

			newBoard.move(pieceCoords[0], pieceCoords[1], move[0], move[1]);
			if (!(newBoard.teamInCheck() == piece.team)) {
				moves[index] = move;
				index++;
			}
		}

		trimmedMoves = ArrayUtil.trim2d(moves, index);
		return trimmedMoves;
	}
}
