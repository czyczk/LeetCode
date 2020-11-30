public class Main {
    public static void main(String[] args) {
        var s1 = "aab";
        var s2 = "aaab";
        var s3 = "vvvlo";
        var s4 = "zhmyo";

        var s = new Solution();
        // Expecting "aba"
        System.out.println(s.reorganizeString(s1));
        // Expecting ""
        System.out.println(s.reorganizeString(s2));
        // Expecting "vlvov"
        System.out.println(s.reorganizeString(s3));
        // Expecting "hymzo"
        System.out.println(s.reorganizeString(s4));
    }
}