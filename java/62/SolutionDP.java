public class SolutionDP {
    public int uniquePaths(int m, int n) {
        var numPaths = new int[m][n];
        numPaths[0][0] = 1;

        for (var i = 1; i < m; i++) {
            numPaths[i][0] = 1;
        }

        for (var j = 1; j < n; j++) {
            numPaths[0][j] = 1;
        }

        for (var i = 1; i < m; i++) {
            for (var j = 1; j < n; j++) {
                numPaths[i][j] = numPaths[i - 1][j] + numPaths[i][j - 1];
            }
        }

        return numPaths[m - 1][n - 1];
    }
}
