package com.thealphareturns.chessai.lib.util;

import com.thealphareturns.chessai.lib.board.Board;

import java.util.Scanner;

public class FirstMoveGenerator {
	public static int[] gen(Board board) {
		Scanner sc = new Scanner(System.in);

		UIUtil.consoleClear();
		UIUtil.header("Choose your first move");
		board.draw();

		System.out.println("\nPlease enter white's move in the format \"a2 a4\".");
		UIUtil.separator(40);

		String[] move = sc.nextLine().split(" ");
		int[] moveCoords = new int[4];

		// note - a8 is 0,0
		moveCoords[0] = 8 - Integer.parseInt(move[0].substring(1));
		moveCoords[1] = move[0].charAt(0) - 97;
		moveCoords[2] = 8 - Integer.parseInt(move[1].substring(1));
		moveCoords[3] = move[1].charAt(0) - 97;

		return moveCoords;
	}
}
