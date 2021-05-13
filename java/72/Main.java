public class Main {
    public static void main(String[] args) {
        var s = new Solution();

        var w11 = "horse";
        var w12 = "ros";

        var w21 = "intention";
        var w22 = "execution";

        // Expecting 3
        System.out.println(s.minDistance(w11, w12));

        // Expecting 5
        System.out.println(s.minDistance(w21, w22));
    }
}