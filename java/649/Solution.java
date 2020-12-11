import java.util.LinkedList;

public class Solution {
    public String predictPartyVictory(String senate) {
        var len = senate.length();
        var rQueue = new LinkedList<Integer>();
        var dQueue = new LinkedList<Integer>();

        var i = 0;
        for (var ch : senate.toCharArray()) {
            if (ch == 'R') {
                rQueue.offer(i++);
            } else {
                dQueue.offer(i++);
            }
        }

        while (true) {
            if (rQueue.isEmpty()) {
                return "Dire";
            } else if (dQueue.isEmpty()) {
                return "Radiant";
            }

            if (rQueue.peek() < dQueue.peek()) {
                dQueue.pop();
                rQueue.offer(rQueue.pop() + len);
            } else {
                rQueue.pop();
                dQueue.offer(dQueue.pop() + len);
            }
        }
    }
}