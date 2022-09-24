package com.thealphareturns.chessai.lib.util;

public class ArrayUtil {
	/**
	 * Returns a 2d array where all elements after a specified length are cut off
	 * @param array The 2d array to be cut
	 * @param yCutoff The y index to cut off at (or -1 for automatic)
	 * @return The cut 2d array
	 */
	public static int[][] trim2d(int[][] array, int yCutoff) {
		int[][] trimmedArray;

		if (yCutoff != -1) {
			// y starts with 1, like an array's length (not index)!
			trimmedArray = new int[yCutoff][array[0].length];

			for (int y = 0; y < yCutoff - 1; y++) {
				System.arraycopy(array[y], 0, trimmedArray[y], 0, array[0].length);
			}
		} else {
			// remove any zeros from the end of the array
			int y = array.length - 1;
			while (true) {
				if (array[y][0] == 0) {
					y--;
				} else {
					break;
				}
			}

			trimmedArray = new int[y + 1][array[0].length];

			for (int i = 0; i < y + 1; i++) {
				System.arraycopy(array[i], 0, trimmedArray[i], 0, array[0].length);
			}
		}
		return trimmedArray;
	}

	public static int[][] concat2d(int[][] array1, int[][] array2) {
		if (array1[0].length != array2[0].length) {
			throw new IllegalArgumentException("Cannot concatenate arrays with different x-lengths");
		}

		int[][] concatArray = new int[array1.length + array2.length][array1[0].length];

		for (int y = 0; y < array1.length; y++) {
			System.arraycopy(array1[y], 0, concatArray[y], 0, array1[0].length);
		}

		for (int y = 0; y < array2.length; y++) {
			System.arraycopy(array2[y], 0, concatArray[y + array1.length], 0, array2[0].length);
		}

		return concatArray;
	}
}
