public class Main {
    public static void main(String[] args) {
        var string1 = "aaaabbbbcccc";
        var string2 = "rat";
        var string3 = "leetcode";
        var string4 = "ggggggg";
        var string5 = "spo";

        var s = new Solution();

        // "abccbaabccba"
        System.out.println(s.sortString(string1));
        // "art"
        System.out.println(s.sortString(string2));
        // "cdelotee"
        System.out.println(s.sortString(string3));
        // "ggggggg"
        System.out.println(s.sortString(string4));
        // "ops"
        System.out.println(s.sortString(string5));
    }
}