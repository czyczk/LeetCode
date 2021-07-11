public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var cost1 = new int[] { 10, 15, 20 };
		final var cost2 = new int[] { 1, 100, 1, 1, 1, 100, 1, 1, 100, 1 };

		// Expecting 15
		System.out.println(s.minCostClimbingStairs(cost1));
		// Expecting 6
		System.out.println(s.minCostClimbingStairs(cost2));
	}
}
