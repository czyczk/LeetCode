import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Solution {
    public List<Integer> splitIntoFibonacci(String S) {
        final var strLen = S.length();

        if (strLen < 3) {
            return new ArrayList<Integer>();
        }

        final var resultStr = new ArrayList<String>();
        final var result = new ArrayList<Integer>();
        final var stack = new LinkedList<LinkedList<String>>();

        var idx = 0;

        {
            final var queue = new LinkedList<String>();
            for (var i = 1; i <= strLen; i++) {
                queue.add(S.substring(0, i));
            }

            stack.add(queue);
        }

        var shouldSkipLayer = false;

        while (!stack.isEmpty()) {
            final var queue = stack.peekLast();

            // If the queue is consumed and the process is still going on, this trace has
            // reached a dead end. Go one step back.
            if (queue.isEmpty()) {
                stack.removeLast();

                shouldSkipLayer = false;

                // If the stack is empty, it should fail fast.
                if (stack.isEmpty()) {
                    continue;
                }

                final var removedStr = resultStr.remove(resultStr.size() - 1);
                idx -= removedStr.length();
                result.remove(result.size() - 1);
                continue;
            }

            // Validate the first number in the queue.
            final var numStr = queue.removeFirst();
            if (shouldSkipLayer) {
                continue;
            }

            // If it starts with "0" and is not "0", discard the number.
            if (numStr.length() > 1 && numStr.startsWith("0")) {
                continue;
            }

            final int num;
            try {
                num = Integer.parseInt(numStr);
            } catch (NumberFormatException ex) {
                continue;
            }

            // Accept the number in two cases:
            // 1. The result length < 2;
            // 2. The number meets the Fibonacci rule.
            final var resultLen = result.size();
            if (resultLen < 2 || result.get(resultLen - 2) + result.get(resultLen - 1) == num) {
                resultStr.add(numStr);
                result.add(num);
                idx += numStr.length();

                // Main.printResult(result);

                // If the index has reached the end, don't go one step inward.
                if (idx == strLen && result.size() > 2) {
                    if (result.size() > 2) {
                        break;
                    }

                    continue;
                }

                // After accepting the number, go one step inward.
                final var inwardQueue = new LinkedList<String>();
                for (var i = idx + 1; i <= strLen; i++) {
                    inwardQueue.add(S.substring(idx, i));
                }

                stack.add(inwardQueue);
            } else if (result.get(resultLen - 2) + result.get(resultLen - 1) < num) {
                shouldSkipLayer = true;
            }
        }

        return result;
    }
}
