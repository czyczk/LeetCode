public class Main {
    public static void main(String[] args) {
        var bills1 = new int[] { 5, 5, 5, 10, 20 };
        var bills2 = new int[] { 5, 5, 10 };
        var bills3 = new int[] { 10, 10 };
        var bills4 = new int[] { 5, 5, 10, 10, 20 };
        var bills5 = new int[] { 5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5 };

        var s = new Solution();
        // Expecting true
        System.out.println(s.lemonadeChange(bills1));
        // Expecting true
        System.out.println(s.lemonadeChange(bills2));
        // Expecting false
        System.out.println(s.lemonadeChange(bills3));
        // Expecting false
        System.out.println(s.lemonadeChange(bills4));
        // Expecting true
        System.out.println(s.lemonadeChange(bills5));
    }
}