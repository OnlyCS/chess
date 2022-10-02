package com.thealphareturns.chessai.lib.util;

import com.google.gson.Gson;

public class DeepCopy {
	public static Object copy(Object original) {
		Gson gson = new Gson();
		return gson.fromJson(gson.toJson(original), original.getClass());
	}
}
