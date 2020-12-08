import java.util.List;

public class Main {
    public static void main(String[] args) {
        final var str1 = "123456579";
        final var str2 = "11235813";
        final var str3 = "112358130";
        final var str4 = "0123";
        final var str5 = "1101111";

        final var s = new Solution();
        // Expecting [123, 456, 579]
        final var result1 = s.splitIntoFibonacci(str1);
        printResult(result1);
        // Expecting [1, 1, 2, 3, 5, 8, 13]
        final var result2 = s.splitIntoFibonacci(str2);
        printResult(result2);
        // Expecting []
        final var result3 = s.splitIntoFibonacci(str3);
        printResult(result3);
        // Expecting []
        final var result4 = s.splitIntoFibonacci(str4);
        printResult(result4);
        // Expecting [110, 1, 111]
        final var result5 = s.splitIntoFibonacci(str5);
        printResult(result5);
    }

    public static void printResult(List<Integer> result) {
        if (result == null || result.size() == 0) {
            System.out.println("[ ]");
            return;
        }

        final var len = result.size();

        System.out.print("[ ");
        for (var i = 0; i < len; i++) {
            System.out.print(result.get(i) + " ");

            if (i == len - 1) {
                System.out.println("]");
            }
        }
    }
}