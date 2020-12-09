public class SolutionMath {
    public int uniquePaths(int m, int n) {
        if (m == 1 || n == 1) {
            return 1;
        }

        if (m > n) {
            var temp = m;
            m = n;
            n = temp;
        }

        long ans = 1;
        for (int i = n, j = 1; j < m; i++, j++) {
            ans = ans * i / j;
        }

        return (int) ans;
    }
}
