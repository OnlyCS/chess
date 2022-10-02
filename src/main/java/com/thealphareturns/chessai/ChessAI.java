package com.thealphareturns.chessai;

import com.thealphareturns.chessai.lib.board.Board;
import com.thealphareturns.chessai.lib.piece.Piece;
import com.thealphareturns.chessai.lib.util.FirstMoveGenerator;
import com.thealphareturns.chessai.lib.util.UIUtil;
import org.json.JSONObject;

import java.util.Scanner;

public class ChessAI {
	public static void main(String[] args) {
		Scanner sc = new Scanner(System.in);

		UIUtil.consoleClear();
		UIUtil.header("Chess AI by TheAlphaReturns");
		System.out.println("Welcome to the Chess AI by TheAlphaReturns!");
		System.out.println("First off, what color would you like to play as (w/b)?\n");
		UIUtil.separator(40);
		char color = sc.nextLine().charAt(0);

		JSONObject config = new JSONObject()
			.put("nextTurn", "w");

		if (color == 'b') {
			int[] firstMove = FirstMoveGenerator.gen(new Board(new Piece[1][1], config));
			config.put("whiteMove", new JSONObject().put("y1", firstMove[0]).put("x1", firstMove[1]).put("y2", firstMove[2]).put("x2", firstMove[3]));
			config.put("nextTurn", "b");
		}

		Board board = new Board(new Piece[1][1], config);
		UIUtil.consoleClear();
		board.draw();
	}
}