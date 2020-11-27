import java.util.Random;

class Solution {
    private ListNode head;
    private Random random;

    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    public Solution(ListNode head) {
        this.head = head;
        this.random = new Random();
    }
    
    /** Returns a random node's value. */
    public int getRandom() {
        int result = 0; // The number here exists only for error suppression. It doesn't matter since it will very soon be replaced with the first value in the list.
        ListNode cursor = head;
        int count = 1;

        while (cursor != null) {
            if (random.nextInt(count) == 0) {
                result = cursor.val;
            }

            cursor = cursor.next;
            count++;
        }

        return result;
    }
}
