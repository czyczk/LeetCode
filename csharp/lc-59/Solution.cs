using System;
using System.Collections.Generic;
using System.Linq;

namespace lc_59
{
    public class Solution
    {
        public int[][] GenerateMatrix(int n)
        {
            // 0: right, 1: down, 2: left, 3: up
            var curDirection = 0;
            var totalSteps = new List<int> { n, n - 1, n - 1, n - 2 };
            var remainingSteps = new List<int>(totalSteps);
            var ret = new int[n][];
            Enumerable.Range(0, n).ToList().ForEach(i => ret[i] = new int[n]);

            remainingSteps[0]--;
            ret[0][0] = 1;
            var cnt = 1;
            var i = 0;
            var j = 0;

            while (cnt < n * n)
            {
                if (remainingSteps[curDirection] > 0)
                {
                    remainingSteps[curDirection]--;
                    // Update the next coordinate (position) in the current direction
                    switch (curDirection)
                    {
                        case 0:
                            j++;
                            break;
                        case 1:
                            i++;
                            break;
                        case 2:
                            j--;
                            break;
                        case 3:
                            i--;
                            break;
                        default:
                            throw new Exception();
                    }
                    // Set the matrix in the position with the desired number and update the count
                    cnt++;
                    ret[i][j] = cnt;
                }
                else
                {
                    // Each time the remaining steps of a direction are exausted, sub
                    // total_steps of the current direction by 2.
                    totalSteps[curDirection] -= 2;
                    // Reset the remaining_steps of the current direction.
                    remainingSteps[curDirection] = totalSteps[curDirection];
                    // Change the direction clockwise.
                    curDirection = (curDirection + 1) % 4;
                }
            }

            return ret;
        }
    }
}
