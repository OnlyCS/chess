package com.thealphareturns.chessai.lib.board;

import com.thealphareturns.chessai.lib.util.ArrayUtil;
import com.thealphareturns.chessai.lib.piece.*;
import org.json.JSONObject;

public class Board {
	private char turn = 'w';
	private Piece[][] board;

	private int whitePoints = 0;
	private int blackPoints = 0;

	public Board(Piece[][] presetBoard, JSONObject config) {
		this.reset();
		char nextTurn = config.getString("nextTurn").charAt(0);

		if (presetBoard.length == 8 & presetBoard[0].length == 8) {
			this.board = presetBoard;
			this.turn = nextTurn;
		} else if (nextTurn == 'b') {
				JSONObject whiteMove = config.getJSONObject("whiteMove");

				int y1 = whiteMove.getInt("y1");
				int x1 = whiteMove.getInt("x1");
				int y2 = whiteMove.getInt("y2");
				int x2 = whiteMove.getInt("x2");

				this.move(y1, x1, y2, x2);
		}

	}

	public void reset() {
		this.board = new Piece[8][8];

		// populate board with pieces
		this.board[0][0] = new Rook(0, 0, 'b');
		this.board[0][1] = new Knight(0, 1, 'b');
		this.board[0][2] = new Bishop(0, 2, 'b');
		this.board[0][3] = new Queen(0, 3, 'b');
		this.board[0][4] = new King(0, 4, 'b');
		this.board[0][5] = new Bishop(0, 5, 'b');
		this.board[0][6] = new Knight(0, 6, 'b');
		this.board[0][7] = new Rook(0, 7, 'b');

		for (int i = 0; i < 8; i++) {
			this.board[1][i] = new Pawn(1, i, 'b');
		}

		this.board[7][0] = new Rook(7, 0, 'w');
		this.board[7][1] = new Knight(7, 1, 'w');
		this.board[7][2] = new Bishop(7, 2, 'w');
		this.board[7][3] = new Queen(7, 3, 'w');
		this.board[7][4] = new King(7, 4, 'w');
		this.board[7][5] = new Bishop(7, 5, 'w');
		this.board[7][6] = new Knight(7, 6, 'w');
		this.board[7][7] = new Rook(7, 7, 'w');

		for (int i = 0; i < 8; i++) {
			this.board[6][i] = new Pawn(6, i, 'w');
		}
	}

	public void move(int y1, int x1, int y2, int x2) {
		if (this.board[y1][x1] == null) {
			throw new IllegalArgumentException("No piece at (" + y1 + ", " + x1 + ")");
		}

		if (this.board[y1][x1].team != this.turn) {
			throw new IllegalArgumentException("It is not " + (this.board[y1][x1].team == 'w' ? "white" : "black") + "'s turn");
		}

		if (!this.board[y1][x1].canMakeMove(y2, x2, this.board)) {
			throw new IllegalArgumentException("The piece at (" + y1 + ", " + x1 + ") cannot move to (" + y2 + ", " + x2 + ")");
		}

		// check takeover
		if (this.board[y2][x2] != null) {
			if (this.board[y2][x2].team == 'w') {
				this.blackPoints += this.board[y2][x2].value;
			} else {
				this.whitePoints += this.board[y2][x2].value;
			}
		}

		this.board = this.board[y1][x1].move(y2, x2, this.board);
	}

	public void draw() {
		System.out.println("  a b c d e f g h");
		for (int i = 0; i < 8; i++) {
			System.out.print(8 - i + " ");
			for (int j = 0; j < 8; j++) {
				if (this.board[i][j] == null) {
					System.out.print("  ");
				} else {
					System.out.print(this.board[i][j].toString() + " ");
				}
			}
			System.out.println(8 - i);
		}
		System.out.println("  a b c d e f g h");
	}

	public int[][] getMoves() {
		int[][] moves = new int[1024][2];

		for (int y = 0; y < 8; y++) {
			for (int x = 0; x < 8; x++) {
				int[][] pieceMoves = this.board[y][x].getMoves(this.board);
				moves = ArrayUtil.concat2d(moves, pieceMoves);
			}
		}

		moves = ArrayUtil.trim2d(moves, -1);

		return moves;
	}

	public int getWhitePoints() {
		return whitePoints;
	}

	public int getBlackPoints() {
		return blackPoints;
	}
}
