public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var l11 = new ListNode(1, new ListNode(2, new ListNode(4)));
		final var l12 = new ListNode(1, new ListNode(3, new ListNode(4)));

		final ListNode l21 = null;
		final ListNode l22 = null;

		final ListNode l31 = null;
		final ListNode l32 = new ListNode(0);

		// Expecting [1, 1, 2, 3, 4, 4]
		printLinkedList(s.mergeTwoLists(l11, l12));
		// Expecting []
		printLinkedList(s.mergeTwoLists(l21, l22));
		// Expecting [0]
		printLinkedList(s.mergeTwoLists(l31, l32));
	}

	private static void printLinkedList(final ListNode head) {
		var isFirst = true;
		var cur = head;

		System.out.print("[");
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
	public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
		final var fakeHead = new ListNode();
		var cur = fakeHead;

		while (l1 != null || l2 != null) {
			if (l1 == null) {
				cur.next = l2;
				l2 = l2.next;
			} else if (l2 == null) {
				cur.next = l1;
				l1 = l1.next;
			} else {
				if (l1.val < l2.val) {
					cur.next = l1;
					l1 = l1.next;
				} else {
					cur.next = l2;
					l2 = l2.next;
				}
			}

			cur = cur.next;
		}

		return fakeHead.next;
	}
}