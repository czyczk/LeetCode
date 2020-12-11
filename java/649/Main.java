public class Main {
    public static void main(String[] args) {
        var senate1 = "RD";
        var senate2 = "RDD";
        var senate3 = "RDRDD";

        var s = new Solution();
        // Expecting "Radiant"
        System.out.println(s.predictPartyVictory(senate1));
        // Expecting "Dire"
        System.out.println(s.predictPartyVictory(senate2));
        // Expecting "Radiant"
        System.out.println(s.predictPartyVictory(senate3));
    }
}