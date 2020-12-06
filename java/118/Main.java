public class Main {
    public static void main(String args[]) {
        var num1 = 5;

        var s = new Solution();
        var result = s.generate(num1);

        for (var row : result) {
            for (var num : row) {
                System.out.print(num + " ");
            }

            System.out.println();
        }
    }
}