import java.util.ArrayList;
import java.util.Arrays;

public class SolutionNormal {
    public String sortString(String s) {
        if (s.isEmpty()) {
            return "";
        }

        // Sort the string and turn it to an ArrayList
        var charArray = s.toCharArray();
        Arrays.sort(charArray);
        var sorted = new ArrayList<Character>(s.length());
        for (char c : charArray) {
            sorted.add(c);
        }

        var result = new StringBuilder();

        // Scan back and forth. Iterate the string multiple times.
        // Small mode: Scan from left to right. Move the char to the result if it's
        // larger.
        // ! Small mode: Scan from right to left. Move the char to the result if it's
        // smaller.
        boolean isSmallMode = true;

        int cursor = 0;
        char veryChar = '\0';
        while (sorted.size() != 0) {
            char cursorChar = sorted.get(cursor);
            if (isSmallMode) {
                if (cursorChar > veryChar) {
                    veryChar = cursorChar;
                    result.append(sorted.remove(cursor));
                } else {
                    cursor++;
                }
            } else {
                if (cursorChar < veryChar) {
                    veryChar = cursorChar;
                    result.append(sorted.remove(cursor));
                    cursor--;
                } else {
                    cursor--;
                }
            }

            if (isSmallMode && cursor == sorted.size() - 1) {
                isSmallMode = false;
                veryChar = (char) 255;
            } else if (!isSmallMode && cursor == 0) {
                isSmallMode = true;
                veryChar = '\0';
            }
        }

        return result.toString();
    }
}
