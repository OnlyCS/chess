package com.thealphareturns.chessai.lib.util;

public class UIUtil {
	public static void header(String header) {
		System.out.println("-".repeat(header.length() + 4));
		System.out.println(header);
		System.out.println("-".repeat(header.length() + 4));
		System.out.println();
	}

	public static void separator(int len) {
		System.out.println("-".repeat(len));
	}

	public static void consoleClear() {
		System.out.print("\033[H\033[2J");
		System.out.flush();
	}
}
