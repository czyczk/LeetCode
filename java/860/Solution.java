public class Solution {
    public boolean lemonadeChange(int[] bills) {
        var fives = 0;
        var tens = 0;

        for (final var bill : bills) {
            if (bill == 5) {
                fives++;
            } else if (bill == 10) {
                if (fives == 0) {
                    return false;
                }

                fives--;
                tens++;
            } else {
                if (tens > 0) {
                    if (fives > 0) {
                        tens--;
                        fives--;
                    } else {
                        return false;
                    }
                } else if (fives > 2) {
                    fives -= 3;
                } else {
                    return false;
                }
            }
        }

        return true;
    }
}
