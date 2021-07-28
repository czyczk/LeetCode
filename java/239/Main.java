import java.util.ArrayDeque;
import java.util.Arrays;
import java.util.Deque;

public class Main {
	public static void main(String[] args) {
		var s = new Solution();

		var n1 = new int[] { 1, 3, -1, -3, 5, 3, 6, 7 };
		var k1 = 3;
		var n2 = new int[] { 1 };
		var k2 = 1;
		var n3 = new int[] { 1, -1 };
		var k3 = 1;
		var n4 = new int[] { 9, 11 };
		var k4 = 2;
		var n5 = new int[] { 4, -2 };
		var k5 = 2;

		// Expecting [3, 3, 5, 5, 6, 7]
		System.out.println(Arrays.toString(s.maxSlidingWindow(n1, k1)));
		// Expecting [1]
		System.out.println(Arrays.toString(s.maxSlidingWindow(n2, k2)));
		// Expecting [1, -1]
		System.out.println(Arrays.toString(s.maxSlidingWindow(n3, k3)));
		// Expecting [11]
		System.out.println(Arrays.toString(s.maxSlidingWindow(n4, k4)));
		// Expecting [4]
		System.out.println(Arrays.toString(s.maxSlidingWindow(n5, k5)));
	}
}

class Solution {
	public int[] maxSlidingWindow(int[] nums, int k) {
		final var n = nums.length;
		var q = new MonotonousQueue();
		for (int i = 0; i < k; i++) {
			q.add(nums[i]);
		}

		final var ret = new int[n - k + 1];
		ret[0] = q.peek();

		for (int i = k; i < n; i++) {
			final var idxRemoved = i - k;
			q.add(nums[i]);
			q.remove(nums[idxRemoved]);
			ret[idxRemoved + 1] = q.peek();
		}

		return ret;
	}
}

class MonotonousQueue {
	private Deque<Integer> queue;

	public MonotonousQueue() {
		this.queue = new ArrayDeque<>();
	}

	public int size() {
		return this.queue.size();
	}

	public int getFirst() {
		return this.queue.getFirst();
	}

	public void add(int e) {
		while (this.queue.size() > 0 && this.queue.getLast() < e) {
			this.queue.removeLast();
		}

		this.queue.addLast(e);
	}

	public int peek() {
		return this.queue.getFirst();
	}

	public int remove(int e) {
		if (this.queue.peek() == e) {
			this.queue.removeFirst();
		}

		return e;
	}
}