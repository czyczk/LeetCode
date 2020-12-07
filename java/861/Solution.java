public class Solution {
    public int matrixScore(int[][] A) {
        var numRows = A.length;
        var numCols = A[0].length;

        var rowInverted = new boolean[numRows];

        // Ensure that the first column is all 1
        for (var i = 0; i < numRows; i++) {
            if (A[i][0] == 0) {
                rowInverted[i] = true;
            }
        }

        var score = numRows * 1 << (numCols - 1);

        if (numCols < 2) {
            return score;
        }

        // Scan from the second column to ensure the max score
        for (var j = 1; j < numCols; j++) {
            var numOnes = 0;

            for (var i = 0; i < numRows; i++) {
                if (A[i][j] == 1 ^ rowInverted[i]) {
                    numOnes++;
                }
            }

            if (numOnes < (numRows + 1) / 2) {
                numOnes = numRows - numOnes;
            }

            score += numOnes * 1 << (numCols - j - 1);
        }

        return score;
    }
}
