public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var h1 = new ListNode(4, new ListNode(2, new ListNode(1, new ListNode(3))));
		final var h2 = new ListNode(-1, new ListNode(5, new ListNode(3, new ListNode(4, new ListNode(0)))));
		final var h3 = (ListNode) null;

		// Expecting [1, 2, 3, 4]
		printLinkedList(s.sortList(h1));
		// Expecting [-1, 0, 3, 4, 5]
		printLinkedList(s.sortList(h2));
		// Expecting []
		printLinkedList(s.sortList(h3));
	}

	private static void printLinkedList(final ListNode head) {
		var isFirst = true;
		System.out.print("[");
		var cur = head;
		while (cur != null) {
			if (!isFirst) {
				System.out.print(", ");
			}
			System.out.print(cur.val);
			cur = cur.next;
			isFirst = false;
		}
		System.out.println("]");
	}
}

/**
 * Definition for singly-linked list.
 */
class ListNode {
	int val;
	ListNode next;

	ListNode() {
	}

	ListNode(final int val) {
		this.val = val;
	}

	ListNode(final int val, final ListNode next) {
		this.val = val;
		this.next = next;
	}
}

class Solution {
	public ListNode sortList(final ListNode head) {
		if (head == null) {
			return null;
		}

		final var fakeHead = new ListNode(0, head);
		var subLen = 1;

		loop: while (true) {
			var prevTail = fakeHead;
			var cur = fakeHead.next;
			var isMerged = false;
			while (cur != null) {
				final var h1 = cur;
				cur = skip(cur, subLen);
				final var h2 = cur;
				if (h2 == null && !isMerged) {
					break loop;
				}
				cur = skip(cur, subLen);
				final var mergeResult = mergeSorted(h1, h2, subLen, cur);
				isMerged = true;
				prevTail.next = mergeResult.getHead();
				prevTail = mergeResult.getTail();
			}

			subLen <<= 1;
		}

		return fakeHead.next;
	}

	private ListNode skip(ListNode cur, final int steps) {
		for (int i = 0; i < steps; i++) {
			if (cur == null) {
				return null;
			}

			cur = cur.next;
		}

		return cur;
	}

	public MergeResult mergeSorted(ListNode h1, ListNode h2, final int length, final ListNode next) {
		final var fakeHead = new ListNode();
		var cur = fakeHead;
		var idx1 = 0;
		var idx2 = 0;

		while (!isConsumed(h1, idx1, length) || !isConsumed(h2, idx2, length)) {
			if (isConsumed(h1, idx1, length)) {
				cur.next = h2;
				h2 = h2.next;
				idx2++;
			} else if (isConsumed(h2, idx2, length)) {
				cur.next = h1;
				h1 = h1.next;
				idx1++;
			} else {
				if (h1.val < h2.val) {
					cur.next = h1;
					h1 = h1.next;
					idx1++;
				} else {
					cur.next = h2;
					h2 = h2.next;
					idx2++;
				}
			}

			cur = cur.next;
		}

		cur.next = next;

		return new MergeResult(fakeHead.next, cur);
	}

	private boolean isConsumed(final ListNode cur, final int i, final int length) {
		return cur == null || i >= length;
	}
}

class MergeResult {
	private final ListNode head;
	private final ListNode tail;

	public MergeResult(final ListNode head, final ListNode tail) {
		this.head = head;
		this.tail = tail;
	}

	public ListNode getHead() {
		return this.head;
	}

	public ListNode getTail() {
		return this.tail;
	}
}
