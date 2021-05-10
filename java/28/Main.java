public class Main {
	public static void main(String[] args) {
		var s = new Solution();

		var h1 = "hello";
		var n1 = "ll";

		var h2 = "aaaaa";
		var n2 = "bba";

		var h3 = "";
		var n3 = "";

		var h4 = "mississippi;
		var n4 = "issip";

		// Expecting 2
		System.out.println(s.strStr(h1, n1));
		// Expecting -1
		System.out.println(s.strStr(h2, n2));
		// Expecting 0
		System.out.println(s.strStr(h3, n3));
		// Expecting 4
		System.out.println(s.strStr(h4, n4));
	}
}
